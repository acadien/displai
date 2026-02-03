//! displai - A whiteboard-style drawing application with AI interaction.
//!
//! This library provides the core functionality for the displai application,
//! including drawing primitives, UI rendering, and command handling.

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::io::{self, BufRead, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

pub mod command;
pub mod drawing;
pub mod ui;

pub use command::*;
pub use drawing::*;
pub use ui::*;

// ============================================================================
// Constants
// ============================================================================

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;
pub const WHITE: u32 = 0xFFFFFF;
pub const BLACK: u32 = 0x000000;
pub const GRAY: u32 = 0xE0E0E0;
pub const DARK_GRAY: u32 = 0x808080;
pub const RED: u32 = 0xE04040;
pub const BLUE: u32 = 0x4040E0;

pub const COLOR_PALETTE: [u32; 14] = [
    0x000000, // Black (default)
    0xFFFFFF, // White (acts as eraser)
    0xE04040, // Red
    0xE07040, // Red-Orange
    0xE0A040, // Orange
    0xE0E040, // Yellow
    0xA0E040, // Yellow-Green
    0x40E040, // Green
    0x40E0A0, // Cyan-Green
    0x40E0E0, // Cyan
    0x4080E0, // Blue
    0x4040E0, // Blue-Violet
    0x8040E0, // Violet
    0xE040E0, // Magenta
];

pub const TITLE_BAR_HEIGHT: usize = 30;
pub const BUTTON_SIZE: usize = 24;
pub const BUTTON_MARGIN: usize = 3;

pub const BOTTOM_TOOLBAR_HEIGHT: usize = 60;
pub const TOOLBAR_ROW_HEIGHT: usize = 30;
pub const CANVAS_TOP: usize = TITLE_BAR_HEIGHT;
pub const CANVAS_BOTTOM: usize = HEIGHT - BOTTOM_TOOLBAR_HEIGHT;

pub const MIN_BRUSH_SIZE: usize = 1;
pub const MAX_BRUSH_SIZE: usize = 20;
pub const DEFAULT_BRUSH_SIZE: usize = 1;

pub const SOCKET_PATH: &str = "/tmp/displai.sock";

// ============================================================================
// Types
// ============================================================================

/// Tool modes for drawing
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToolMode {
    #[default]
    Brush,
    Line,
    Square,
    Rectangle,
    Circle,
    Oval,
    Triangle,
}

// ============================================================================
// Socket/Stdin Communication
// ============================================================================

/// Spawn a thread that reads lines from stdin and sends them to the receiver
fn spawn_stdin_reader() -> Receiver<String> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let stdin = io::stdin();
        let reader = stdin.lock();

        for line in reader.lines().map_while(Result::ok) {
            if tx.send(line).is_err() {
                break;
            }
        }
    });

    rx
}

/// A command received from the socket, with an optional stream to write the response back to
struct SocketCommand {
    line: String,
    stream: Option<UnixStream>, // Only first command per connection gets response
}

/// Spawn a thread that listens on a Unix socket and sends received commands to the receiver
/// Supports multi-line mode: all lines in a connection are processed, but only the first gets a response
fn spawn_unix_socket_listener() -> Receiver<SocketCommand> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // Remove stale socket file if it exists
        let _ = std::fs::remove_file(SOCKET_PATH);

        if let Ok(listener) = UnixListener::bind(SOCKET_PATH) {
            for stream in listener.incoming().flatten() {
                let tx = tx.clone();
                // Handle each connection in its own thread to avoid blocking
                thread::spawn(move || {
                    let mut stream_for_response = stream.try_clone().ok();
                    let reader = io::BufReader::new(stream);
                    for line in reader.lines().map_while(Result::ok) {
                        // First command gets the response stream, subsequent commands get None
                        let response_stream = stream_for_response.take();
                        if tx
                            .send(SocketCommand {
                                line,
                                stream: response_stream,
                            })
                            .is_err()
                        {
                            return;
                        }
                    }
                });
            }
        }
    });

    rx
}

// ============================================================================
// Main Application Loop
// ============================================================================

pub fn run() {
    let mut buffer: Vec<u32> = vec![WHITE; WIDTH * HEIGHT];

    let mut window = Window::new("displai - v0.1", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut is_drawing = false;
    let mut last_pos: Option<(usize, usize)> = None;
    let mut mouse_was_down = false;
    let mut right_mouse_was_down = false;
    let mut edge_color_index: Option<usize> = Some(0); // Some(index) = color, None = transparent
    let mut fill_color_index: Option<usize> = None; // None = transparent (no fill)
    let mut brush_size: usize = DEFAULT_BRUSH_SIZE;
    let mut current_tool: ToolMode = ToolMode::default();
    let mut drag_start: Option<(usize, usize)> = None;

    // Start stdin reader thread for command protocol
    let stdin_rx = spawn_stdin_reader();
    // Start Unix socket listener thread
    let socket_rx = spawn_unix_socket_listener();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Process any stdin commands (non-blocking)
        loop {
            match stdin_rx.try_recv() {
                Ok(line) => {
                    if let Some(cmd) = parse_command(&line) {
                        if let Some(response) = execute_command(
                            &cmd,
                            &mut buffer,
                            &mut edge_color_index,
                            &mut fill_color_index,
                            &mut brush_size,
                        ) {
                            println!("{}", response);
                            let _ = io::stdout().flush();
                        }
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        // Process any Unix socket commands (non-blocking)
        loop {
            match socket_rx.try_recv() {
                Ok(socket_cmd) => {
                    if let Some(cmd) = parse_command(&socket_cmd.line) {
                        let response = execute_command(
                            &cmd,
                            &mut buffer,
                            &mut edge_color_index,
                            &mut fill_color_index,
                            &mut brush_size,
                        );
                        // Only send response if we have a stream (first command per connection)
                        if let Some(mut stream) = socket_cmd.stream {
                            if let Some(resp) = response {
                                let _ = writeln!(stream, "{}", resp);
                            } else {
                                let _ = writeln!(stream, "ok");
                            }
                        }
                    } else if let Some(mut stream) = socket_cmd.stream {
                        let _ = writeln!(stream, "error: unknown command");
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
        draw_title_bar(&mut buffer);
        draw_bottom_toolbar(
            &mut buffer,
            edge_color_index,
            fill_color_index,
            brush_size,
            current_tool,
        );

        let mouse_down = window.get_mouse_down(MouseButton::Left);
        let right_mouse_down = window.get_mouse_down(MouseButton::Right);
        let mouse_clicked = mouse_down && !mouse_was_down;
        let right_mouse_clicked = right_mouse_down && !right_mouse_was_down;

        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Pass) {
            let x = mx as usize;
            let y = my as usize;

            if mouse_clicked {
                if is_in_close_button(x, y) {
                    break;
                }
                if let Some(color_index) = get_clicked_color_index_bottom(x, y) {
                    edge_color_index = Some(color_index);
                }
                if is_in_transparent_button(x, y) {
                    edge_color_index = None; // Transparent edge
                }
                if let Some(tool) = get_clicked_tool(x, y) {
                    current_tool = tool;
                }
                if is_in_minus_button(x, y) && brush_size > MIN_BRUSH_SIZE {
                    brush_size -= 1;
                }
                if is_in_plus_button(x, y) && brush_size < MAX_BRUSH_SIZE {
                    brush_size += 1;
                }
                if is_in_clear_button(x, y) {
                    clear_canvas(&mut buffer);
                }
                // Click on fill indicator to toggle fill off
                if is_in_fill_indicator(x, y) {
                    fill_color_index = None;
                }
            }

            // Right-click to set fill color
            if right_mouse_clicked {
                if let Some(color_index) = get_clicked_color_index_bottom(x, y) {
                    // Toggle fill: if same color, turn off fill; otherwise set it
                    if fill_color_index == Some(color_index) {
                        fill_color_index = None;
                    } else {
                        fill_color_index = Some(color_index);
                    }
                }
                if is_in_transparent_button(x, y) {
                    fill_color_index = None; // Transparent fill
                }
            }

            let edge_color = edge_color_index.map(|i| COLOR_PALETTE[i]);
            let fill_color = fill_color_index.map(|i| COLOR_PALETTE[i]);

            // Freehand drawing only in Brush mode
            if current_tool == ToolMode::Brush {
                if mouse_down && x < WIDTH && (CANVAS_TOP..CANVAS_BOTTOM).contains(&y) {
                    if let Some(color) = edge_color {
                        if is_drawing {
                            if let Some((lx, ly)) = last_pos {
                                draw_brush_line(&mut buffer, lx, ly, x, y, color, brush_size);
                            }
                        } else {
                            draw_circle(&mut buffer, x, y, brush_size, color);
                        }
                    }
                    is_drawing = true;
                    last_pos = Some((x, y));
                } else {
                    is_drawing = false;
                    last_pos = None;
                }
            } else {
                // Shape tools: click-drag to define shape bounds
                let in_canvas = x < WIDTH && (CANVAS_TOP..CANVAS_BOTTOM).contains(&y);

                if mouse_clicked && in_canvas {
                    // Start drag
                    drag_start = Some((x, y));
                } else if !mouse_down && mouse_was_down {
                    // Mouse released - draw the shape if we have a valid drag
                    if let Some((start_x, start_y)) = drag_start {
                        if in_canvas {
                            draw_shape_with_fill(
                                &mut buffer,
                                current_tool,
                                start_x,
                                start_y,
                                x,
                                y,
                                edge_color,
                                fill_color,
                                brush_size,
                            );
                        }
                        drag_start = None;
                    }
                }

                is_drawing = false;
                last_pos = None;
            }
        } else {
            is_drawing = false;
            last_pos = None;
        }

        mouse_was_down = mouse_down;
        right_mouse_was_down = right_mouse_down;

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update buffer");
    }
}
