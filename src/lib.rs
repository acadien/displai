use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::io::{self, BufRead, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

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

/// Commands that can be sent via stdin
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Snapshot,
    Color(usize),
    Size(usize),
    Stroke {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    },
    Dot {
        x: usize,
        y: usize,
    },
    Clear,
    State,
}

/// Parse a command string into a Command enum
pub fn parse_command(input: &str) -> Option<Command> {
    let input = input.trim();
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    match parts[0] {
        "snapshot" => Some(Command::Snapshot),
        "clear" => Some(Command::Clear),
        "state" => Some(Command::State),
        "color" => {
            if parts.len() >= 2 {
                parts[1]
                    .parse::<usize>()
                    .ok()
                    .filter(|&i| i < COLOR_PALETTE.len())
                    .map(Command::Color)
            } else {
                None
            }
        }
        "size" => {
            if parts.len() >= 2 {
                parts[1]
                    .parse::<usize>()
                    .ok()
                    .filter(|&s| (MIN_BRUSH_SIZE..=MAX_BRUSH_SIZE).contains(&s))
                    .map(Command::Size)
            } else {
                None
            }
        }
        "stroke" => {
            // stroke x1,y1 x2,y2
            if parts.len() >= 3 {
                let p1: Vec<&str> = parts[1].split(',').collect();
                let p2: Vec<&str> = parts[2].split(',').collect();
                if p1.len() == 2 && p2.len() == 2 {
                    let x1 = p1[0].parse::<usize>().ok()?;
                    let y1 = p1[1].parse::<usize>().ok()?;
                    let x2 = p2[0].parse::<usize>().ok()?;
                    let y2 = p2[1].parse::<usize>().ok()?;
                    Some(Command::Stroke { x1, y1, x2, y2 })
                } else {
                    None
                }
            } else {
                None
            }
        }
        "dot" => {
            // dot x,y
            if parts.len() >= 2 {
                let coords: Vec<&str> = parts[1].split(',').collect();
                if coords.len() == 2 {
                    let x = coords[0].parse::<usize>().ok()?;
                    let y = coords[1].parse::<usize>().ok()?;
                    Some(Command::Dot { x, y })
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Execute a command, modifying the buffer and/or state
/// Returns an optional response string to print to stdout
pub fn execute_command(
    cmd: &Command,
    buffer: &mut [u32],
    selected_color_index: &mut usize,
    brush_size: &mut usize,
) -> Option<String> {
    match cmd {
        Command::Snapshot => {
            if let Err(e) = save_canvas_png(buffer, "canvas.png") {
                Some(format!("error: {}", e))
            } else {
                Some("saved canvas.png".to_string())
            }
        }
        Command::Color(index) => {
            *selected_color_index = *index;
            None
        }
        Command::Size(size) => {
            *brush_size = *size;
            None
        }
        Command::Stroke { x1, y1, x2, y2 } => {
            let color = COLOR_PALETTE[*selected_color_index];
            draw_brush_line(buffer, *x1, *y1, *x2, *y2, color, *brush_size);
            None
        }
        Command::Dot { x, y } => {
            let color = COLOR_PALETTE[*selected_color_index];
            draw_circle(buffer, *x, *y, *brush_size, color);
            None
        }
        Command::Clear => {
            clear_canvas(buffer);
            None
        }
        Command::State => Some(format!(
            "color:{} size:{}",
            *selected_color_index,
            *brush_size
        )),
    }
}

/// Save the canvas portion of the buffer to a PNG file
pub fn save_canvas_png(buffer: &[u32], path: &str) -> Result<(), String> {
    use image::{ImageBuffer, Rgb};

    let canvas_height = CANVAS_BOTTOM - CANVAS_TOP;
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(WIDTH as u32, canvas_height as u32);

    for y in 0..canvas_height {
        for x in 0..WIDTH {
            let pixel = buffer[(y + CANVAS_TOP) * WIDTH + x];
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
            img.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
        }
    }

    img.save(path).map_err(|e| e.to_string())
}

/// Clear the canvas area to white
pub fn clear_canvas(buffer: &mut [u32]) {
    for y in CANVAS_TOP..CANVAS_BOTTOM {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = WHITE;
        }
    }
}

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

pub const SOCKET_PATH: &str = "/tmp/displai.sock";

/// A command received from the socket, with the stream to write the response back to
struct SocketCommand {
    line: String,
    stream: UnixStream,
}

/// Spawn a thread that listens on a Unix socket and sends received commands to the receiver
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
                        if let Some(response_stream) = stream_for_response.take() {
                            if tx
                                .send(SocketCommand {
                                    line,
                                    stream: response_stream,
                                })
                                .is_err()
                            {
                                return;
                            }
                            // Only handle first line per connection for request/response pattern
                            return;
                        }
                    }
                });
            }
        }
    });

    rx
}

pub fn run() {
    let mut buffer: Vec<u32> = vec![WHITE; WIDTH * HEIGHT];

    let mut window = Window::new("displai - v0.1", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut is_drawing = false;
    let mut last_pos: Option<(usize, usize)> = None;
    let mut mouse_was_down = false;
    let mut selected_color_index: usize = 0;
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
                            &mut selected_color_index,
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
                    let mut stream = socket_cmd.stream;
                    if let Some(cmd) = parse_command(&socket_cmd.line) {
                        let response = execute_command(
                            &cmd,
                            &mut buffer,
                            &mut selected_color_index,
                            &mut brush_size,
                        );
                        if let Some(resp) = response {
                            let _ = writeln!(stream, "{}", resp);
                        } else {
                            let _ = writeln!(stream, "ok");
                        }
                    } else {
                        let _ = writeln!(stream, "error: unknown command");
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
        draw_title_bar(&mut buffer);
        draw_bottom_toolbar(&mut buffer, selected_color_index, brush_size, current_tool);

        let mouse_down = window.get_mouse_down(MouseButton::Left);
        let mouse_clicked = mouse_down && !mouse_was_down;

        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Pass) {
            let x = mx as usize;
            let y = my as usize;

            if mouse_clicked {
                if is_in_close_button(x, y) {
                    break;
                }
                if let Some(color_index) = get_clicked_color_index_bottom(x, y) {
                    selected_color_index = color_index;
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
            }

            let pen_color = COLOR_PALETTE[selected_color_index];

            // Freehand drawing only in Brush mode
            if current_tool == ToolMode::Brush {
                if mouse_down && x < WIDTH && (CANVAS_TOP..CANVAS_BOTTOM).contains(&y) {
                    if is_drawing {
                        if let Some((lx, ly)) = last_pos {
                            draw_brush_line(&mut buffer, lx, ly, x, y, pen_color, brush_size);
                        }
                    } else {
                        draw_circle(&mut buffer, x, y, brush_size, pen_color);
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
                            draw_shape(
                                &mut buffer,
                                current_tool,
                                start_x,
                                start_y,
                                x,
                                y,
                                pen_color,
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

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update buffer");
    }
}

pub fn draw_title_bar(buffer: &mut [u32]) {
    for y in 0..TITLE_BAR_HEIGHT {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = GRAY;
        }
    }

    for x in 0..WIDTH {
        buffer[(TITLE_BAR_HEIGHT - 1) * WIDTH + x] = DARK_GRAY;
    }

    // Draw close button
    let close_x = WIDTH - BUTTON_SIZE - BUTTON_MARGIN;
    let close_y = BUTTON_MARGIN;
    draw_button(buffer, close_x, close_y, RED);
    draw_x(buffer, close_x, close_y);
}

pub fn draw_button(buffer: &mut [u32], bx: usize, by: usize, color: u32) {
    for y in by..by + BUTTON_SIZE {
        for x in bx..bx + BUTTON_SIZE {
            if x < WIDTH && y < HEIGHT {
                buffer[y * WIDTH + x] = color;
            }
        }
    }
}

pub fn draw_button_border(buffer: &mut [u32], bx: usize, by: usize, color: u32) {
    for x in bx..bx + BUTTON_SIZE {
        if x < WIDTH {
            buffer[by * WIDTH + x] = color;
            buffer[(by + BUTTON_SIZE - 1) * WIDTH + x] = color;
        }
    }
    for y in by..by + BUTTON_SIZE {
        if y < HEIGHT {
            buffer[y * WIDTH + bx] = color;
            buffer[y * WIDTH + bx + BUTTON_SIZE - 1] = color;
        }
    }
}

pub fn draw_x(buffer: &mut [u32], bx: usize, by: usize) {
    let padding = 6;
    let start = padding;
    let end = BUTTON_SIZE - padding;

    for i in 0..(end - start) {
        let x1 = bx + start + i;
        let y1 = by + start + i;
        let x2 = bx + end - 1 - i;
        let y2 = by + start + i;

        if x1 < WIDTH && y1 < HEIGHT {
            buffer[y1 * WIDTH + x1] = WHITE;
        }
        if x2 < WIDTH && y2 < HEIGHT {
            buffer[y2 * WIDTH + x2] = WHITE;
        }
    }
}

pub fn is_in_close_button(x: usize, y: usize) -> bool {
    let bx = WIDTH - BUTTON_SIZE - BUTTON_MARGIN;
    let by = BUTTON_MARGIN;
    x >= bx && x < bx + BUTTON_SIZE && y >= by && y < by + BUTTON_SIZE
}

pub fn get_clicked_color_index(x: usize, y: usize) -> Option<usize> {
    let by = BUTTON_MARGIN;
    if y < by || y >= by + BUTTON_SIZE {
        return None;
    }
    for i in 0..12 {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);
        if x >= bx && x < bx + BUTTON_SIZE {
            return Some(i);
        }
    }
    None
}

pub fn set_pixel(buffer: &mut [u32], x: usize, y: usize, color: u32) {
    if x < WIDTH && (CANVAS_TOP..CANVAS_BOTTOM).contains(&y) {
        buffer[y * WIDTH + x] = color;
    }
}

pub fn draw_line(buffer: &mut [u32], x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
    let x0 = x0 as isize;
    let y0 = y0 as isize;
    let x1 = x1 as isize;
    let y1 = y1 as isize;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && x < WIDTH as isize && y >= CANVAS_TOP as isize && y < CANVAS_BOTTOM as isize {
            buffer[y as usize * WIDTH + x as usize] = color;
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

pub fn draw_bottom_toolbar(
    buffer: &mut [u32],
    selected_color_index: usize,
    brush_size: usize,
    current_tool: ToolMode,
) {
    let toolbar_top = CANVAS_BOTTOM;

    // Fill toolbar background with gray
    for y in toolbar_top..HEIGHT {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = GRAY;
        }
    }

    // Draw top border
    for x in 0..WIDTH {
        buffer[toolbar_top * WIDTH + x] = DARK_GRAY;
    }

    // Row 1: 14 color buttons (Black, White, then colors)
    let row1_y = toolbar_top + BUTTON_MARGIN;
    for (i, &color) in COLOR_PALETTE.iter().enumerate() {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);
        draw_button(buffer, bx, row1_y, color);

        // Draw border to indicate selection (use blue for white button to be visible)
        if i == selected_color_index {
            let border_color = if color == WHITE { 0x4040E0 } else { WHITE };
            draw_button_border(buffer, bx, row1_y, border_color);
        } else {
            draw_button_border(buffer, bx, row1_y, DARK_GRAY);
        }
    }

    // Row 2: Tool buttons + Size display + [-] [+] buttons
    let row2_y = toolbar_top + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;

    // Tool buttons: [Brush] [Line] [Sq] [Rect] [Circ] [Oval] [Tri]
    let tools = [
        ToolMode::Brush,
        ToolMode::Line,
        ToolMode::Square,
        ToolMode::Rectangle,
        ToolMode::Circle,
        ToolMode::Oval,
        ToolMode::Triangle,
    ];

    for (i, &tool) in tools.iter().enumerate() {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);
        draw_button(buffer, bx, row2_y, GRAY);
        draw_tool_icon(buffer, bx, row2_y, tool);

        // Highlight selected tool
        if tool == current_tool {
            draw_button_border(buffer, bx, row2_y, 0x4040E0); // Blue border
        } else {
            draw_button_border(buffer, bx, row2_y, DARK_GRAY);
        }
    }

    // Size display (after tool buttons)
    let size_display_x = BUTTON_MARGIN + 7 * (BUTTON_SIZE + BUTTON_MARGIN) + BUTTON_MARGIN;
    draw_size_display(buffer, size_display_x, row2_y, brush_size);

    // Minus button
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    draw_button(buffer, minus_x, row2_y, DARK_GRAY);
    draw_minus_icon(buffer, minus_x, row2_y);

    // Plus button
    let plus_x = minus_x + BUTTON_SIZE + BUTTON_MARGIN;
    draw_button(buffer, plus_x, row2_y, DARK_GRAY);
    draw_plus_icon(buffer, plus_x, row2_y);
}

/// Draw an icon representing a tool
pub fn draw_tool_icon(buffer: &mut [u32], bx: usize, by: usize, tool: ToolMode) {
    let padding = 5;
    let start_x = bx + padding;
    let end_x = bx + BUTTON_SIZE - padding;
    let start_y = by + padding;
    let end_y = by + BUTTON_SIZE - padding;
    let mid_x = bx + BUTTON_SIZE / 2;
    let mid_y = by + BUTTON_SIZE / 2;

    match tool {
        ToolMode::Brush => {
            // Draw a small brush stroke (diagonal line with dot)
            for i in 0..6 {
                let x = start_x + i;
                let y = end_y - i;
                if x < WIDTH && y < HEIGHT {
                    buffer[y * WIDTH + x] = BLACK;
                    if y > 0 {
                        buffer[(y - 1) * WIDTH + x] = BLACK;
                    }
                }
            }
        }
        ToolMode::Line => {
            // Diagonal line
            for i in 0..(end_x - start_x) {
                let x = start_x + i;
                let y = start_y + i;
                if x < WIDTH && y < HEIGHT {
                    buffer[y * WIDTH + x] = BLACK;
                }
            }
        }
        ToolMode::Square => {
            // Square outline
            let size = end_x - start_x;
            for i in 0..size {
                buffer[start_y * WIDTH + start_x + i] = BLACK; // top
                buffer[end_y * WIDTH + start_x + i] = BLACK;   // bottom
                buffer[(start_y + i) * WIDTH + start_x] = BLACK; // left
                buffer[(start_y + i) * WIDTH + end_x] = BLACK;   // right
            }
        }
        ToolMode::Rectangle => {
            // Rectangle (wider than tall)
            let rect_start_y = start_y + 3;
            let rect_end_y = end_y - 3;
            for x in start_x..=end_x {
                buffer[rect_start_y * WIDTH + x] = BLACK; // top
                buffer[rect_end_y * WIDTH + x] = BLACK;   // bottom
            }
            for y in rect_start_y..=rect_end_y {
                buffer[y * WIDTH + start_x] = BLACK; // left
                buffer[y * WIDTH + end_x] = BLACK;   // right
            }
        }
        ToolMode::Circle => {
            // Simple circle approximation
            let radius = (end_x - start_x) / 2;
            let cx = mid_x;
            let cy = mid_y;
            for angle in 0..32 {
                let theta = (angle as f64) * std::f64::consts::PI * 2.0 / 32.0;
                let x = cx as f64 + (radius as f64) * theta.cos();
                let y = cy as f64 + (radius as f64) * theta.sin();
                if x >= 0.0 && (x as usize) < WIDTH && y >= 0.0 && (y as usize) < HEIGHT {
                    buffer[(y as usize) * WIDTH + (x as usize)] = BLACK;
                }
            }
        }
        ToolMode::Oval => {
            // Oval (ellipse - wider than tall)
            let rx = (end_x - start_x) / 2;
            let ry = (end_y - start_y) / 3;
            let cx = mid_x;
            let cy = mid_y;
            for angle in 0..32 {
                let theta = (angle as f64) * std::f64::consts::PI * 2.0 / 32.0;
                let x = cx as f64 + (rx as f64) * theta.cos();
                let y = cy as f64 + (ry as f64) * theta.sin();
                if x >= 0.0 && (x as usize) < WIDTH && y >= 0.0 && (y as usize) < HEIGHT {
                    buffer[(y as usize) * WIDTH + (x as usize)] = BLACK;
                }
            }
        }
        ToolMode::Triangle => {
            // Triangle pointing up
            let apex_x = mid_x;
            let apex_y = start_y;
            let left_x = start_x;
            let right_x = end_x;
            let base_y = end_y;

            // Left edge
            for i in 0..=(base_y - apex_y) {
                let x = apex_x as isize - (i as isize * (apex_x - left_x) as isize / (base_y - apex_y) as isize);
                let y = apex_y + i;
                if x >= 0 && (x as usize) < WIDTH && y < HEIGHT {
                    buffer[y * WIDTH + x as usize] = BLACK;
                }
            }
            // Right edge
            for i in 0..=(base_y - apex_y) {
                let x = apex_x as isize + (i as isize * (right_x - apex_x) as isize / (base_y - apex_y) as isize);
                let y = apex_y + i;
                if x >= 0 && (x as usize) < WIDTH && y < HEIGHT {
                    buffer[y * WIDTH + x as usize] = BLACK;
                }
            }
            // Base
            for x in left_x..=right_x {
                buffer[base_y * WIDTH + x] = BLACK;
            }
        }
    }
}

pub fn draw_minus_icon(buffer: &mut [u32], bx: usize, by: usize) {
    let padding = 6;
    let start_x = bx + padding;
    let end_x = bx + BUTTON_SIZE - padding;
    let mid_y = by + BUTTON_SIZE / 2;

    for x in start_x..end_x {
        if x < WIDTH && mid_y < HEIGHT {
            buffer[mid_y * WIDTH + x] = WHITE;
        }
    }
}

pub fn draw_plus_icon(buffer: &mut [u32], bx: usize, by: usize) {
    let padding = 6;
    let start_x = bx + padding;
    let end_x = bx + BUTTON_SIZE - padding;
    let start_y = by + padding;
    let end_y = by + BUTTON_SIZE - padding;
    let mid_x = bx + BUTTON_SIZE / 2;
    let mid_y = by + BUTTON_SIZE / 2;

    // Horizontal line
    for x in start_x..end_x {
        if x < WIDTH && mid_y < HEIGHT {
            buffer[mid_y * WIDTH + x] = WHITE;
        }
    }
    // Vertical line
    for y in start_y..end_y {
        if mid_x < WIDTH && y < HEIGHT {
            buffer[y * WIDTH + mid_x] = WHITE;
        }
    }
}

pub fn draw_size_display(buffer: &mut [u32], x: usize, y: usize, size: usize) {
    // Draw a small box showing the brush size number
    let width = 40;
    let height = BUTTON_SIZE;

    // Fill background
    for dy in 0..height {
        for dx in 0..width {
            if x + dx < WIDTH && y + dy < HEIGHT {
                buffer[(y + dy) * WIDTH + (x + dx)] = WHITE;
            }
        }
    }

    // Draw border
    for dx in 0..width {
        if x + dx < WIDTH {
            buffer[y * WIDTH + (x + dx)] = DARK_GRAY;
            buffer[(y + height - 1) * WIDTH + (x + dx)] = DARK_GRAY;
        }
    }
    for dy in 0..height {
        if y + dy < HEIGHT {
            buffer[(y + dy) * WIDTH + x] = DARK_GRAY;
            buffer[(y + dy) * WIDTH + (x + width - 1)] = DARK_GRAY;
        }
    }

    // Draw the size number using simple pixel font
    draw_number(buffer, x + 8, y + 6, size);
}

pub fn draw_number(buffer: &mut [u32], x: usize, y: usize, num: usize) {
    // Simple 5x7 pixel font for digits 0-9
    let digits: [[u8; 5]; 10] = [
        [0b01110, 0b10001, 0b10001, 0b10001, 0b01110], // 0
        [0b00100, 0b01100, 0b00100, 0b00100, 0b01110], // 1
        [0b01110, 0b10001, 0b00110, 0b01000, 0b11111], // 2
        [0b01110, 0b10001, 0b00110, 0b10001, 0b01110], // 3
        [0b00010, 0b00110, 0b01010, 0b11111, 0b00010], // 4
        [0b11111, 0b10000, 0b11110, 0b00001, 0b11110], // 5
        [0b01110, 0b10000, 0b11110, 0b10001, 0b01110], // 6
        [0b11111, 0b00001, 0b00010, 0b00100, 0b00100], // 7
        [0b01110, 0b10001, 0b01110, 0b10001, 0b01110], // 8
        [0b01110, 0b10001, 0b01111, 0b00001, 0b01110], // 9
    ];

    // Convert number to string to handle multi-digit
    let num_str = num.to_string();
    let mut offset = 0;

    for ch in num_str.chars() {
        if let Some(digit) = ch.to_digit(10) {
            let pattern = &digits[digit as usize];
            for (row, &bits) in pattern.iter().enumerate() {
                for col in 0..5 {
                    if (bits >> (4 - col)) & 1 == 1 {
                        let px = x + offset + col;
                        let py = y + row * 2; // Scale up vertically
                        if px < WIDTH && py < HEIGHT {
                            buffer[py * WIDTH + px] = BLACK;
                        }
                        if px < WIDTH && py + 1 < HEIGHT {
                            buffer[(py + 1) * WIDTH + px] = BLACK;
                        }
                    }
                }
            }
            offset += 7; // Character width + spacing
        }
    }
}

pub fn draw_circle(buffer: &mut [u32], cx: usize, cy: usize, size: usize, color: u32) {
    let radius = (size as isize) - 1;
    if radius <= 0 {
        // Size 1: draw single pixel
        set_pixel(buffer, cx, cy, color);
        return;
    }

    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                let x = cx as isize + dx;
                let y = cy as isize + dy;
                if x >= 0 && y >= 0 {
                    set_pixel(buffer, x as usize, y as usize, color);
                }
            }
        }
    }
}

pub fn draw_brush_line(
    buffer: &mut [u32],
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    color: u32,
    brush_size: usize,
) {
    // Draw circles along the line using Bresenham's algorithm
    let x0 = x0 as isize;
    let y0 = y0 as isize;
    let x1 = x1 as isize;
    let y1 = y1 as isize;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && y >= 0 {
            draw_circle(buffer, x as usize, y as usize, brush_size, color);
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

/// Draw a shape based on the current tool mode
/// (x1, y1) is the drag start point, (x2, y2) is the drag end point
pub fn draw_shape(
    buffer: &mut [u32],
    tool: ToolMode,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: u32,
    brush_size: usize,
) {
    match tool {
        ToolMode::Brush => {
            // Brush mode doesn't use this function
        }
        ToolMode::Line => {
            draw_brush_line(buffer, x1, y1, x2, y2, color, brush_size);
        }
        ToolMode::Square => {
            draw_shape_square(buffer, x1, y1, x2, y2, color, brush_size);
        }
        ToolMode::Rectangle => {
            draw_shape_rectangle(buffer, x1, y1, x2, y2, color, brush_size);
        }
        ToolMode::Circle => {
            draw_shape_circle(buffer, x1, y1, x2, y2, color, brush_size);
        }
        ToolMode::Oval => {
            draw_shape_oval(buffer, x1, y1, x2, y2, color, brush_size);
        }
        ToolMode::Triangle => {
            draw_shape_triangle(buffer, x1, y1, x2, y2, color, brush_size);
        }
    }
}

/// Draw a square from drag start to end (side length = max of width/height)
pub fn draw_shape_square(
    buffer: &mut [u32],
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: u32,
    brush_size: usize,
) {
    let dx = if x2 > x1 { x2 - x1 } else { x1 - x2 };
    let dy = if y2 > y1 { y2 - y1 } else { y1 - y2 };
    let side = dx.max(dy);

    // Determine direction
    let (left, top) = (
        if x2 >= x1 { x1 } else { x1.saturating_sub(side) },
        if y2 >= y1 { y1 } else { y1.saturating_sub(side) },
    );
    let right = left + side;
    let bottom = top + side;

    // Draw four sides
    draw_brush_line(buffer, left, top, right, top, color, brush_size); // Top
    draw_brush_line(buffer, right, top, right, bottom, color, brush_size); // Right
    draw_brush_line(buffer, right, bottom, left, bottom, color, brush_size); // Bottom
    draw_brush_line(buffer, left, bottom, left, top, color, brush_size); // Left
}

/// Draw a rectangle from drag start to end
pub fn draw_shape_rectangle(
    buffer: &mut [u32],
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: u32,
    brush_size: usize,
) {
    let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    // Draw four sides
    draw_brush_line(buffer, left, top, right, top, color, brush_size); // Top
    draw_brush_line(buffer, right, top, right, bottom, color, brush_size); // Right
    draw_brush_line(buffer, right, bottom, left, bottom, color, brush_size); // Bottom
    draw_brush_line(buffer, left, bottom, left, top, color, brush_size); // Left
}

/// Draw a circle bounded by drag start and end points (diameter, not radius)
/// Circle fits inside the bounding box as a perfect circle (uses min dimension)
pub fn draw_shape_circle(
    buffer: &mut [u32],
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: u32,
    brush_size: usize,
) {
    let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    let width = right - left;
    let height = bottom - top;
    let diameter = width.min(height);
    let radius = diameter as f64 / 2.0;

    if radius < 1.0 {
        draw_circle(buffer, (left + right) / 2, (top + bottom) / 2, brush_size, color);
        return;
    }

    // Center the circle in the bounding box
    let cx = left as f64 + diameter as f64 / 2.0;
    let cy = top as f64 + diameter as f64 / 2.0;

    // Draw circle using parametric form with brush
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let steps = (circumference * 2.0).max(32.0) as usize;

    let mut prev_x = cx + radius;
    let mut prev_y = cy;

    for i in 1..=steps {
        let theta = (i as f64) * 2.0 * std::f64::consts::PI / (steps as f64);
        let curr_x = cx + radius * theta.cos();
        let curr_y = cy + radius * theta.sin();

        draw_brush_line(
            buffer,
            prev_x as usize,
            prev_y as usize,
            curr_x as usize,
            curr_y as usize,
            color,
            brush_size,
        );

        prev_x = curr_x;
        prev_y = curr_y;
    }
}

/// Draw an oval bounded by drag start and end points
pub fn draw_shape_oval(
    buffer: &mut [u32],
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: u32,
    brush_size: usize,
) {
    let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    let cx = (left + right) / 2;
    let cy = (top + bottom) / 2;
    let rx = (right - left) / 2;
    let ry = (bottom - top) / 2;

    if rx == 0 || ry == 0 {
        draw_brush_line(buffer, x1, y1, x2, y2, color, brush_size);
        return;
    }

    // Draw ellipse using parametric form
    let steps = ((rx + ry) * 4).max(32);

    let mut prev_x = cx as f64 + rx as f64;
    let mut prev_y = cy as f64;

    for i in 1..=steps {
        let theta = (i as f64) * 2.0 * std::f64::consts::PI / (steps as f64);
        let curr_x = cx as f64 + (rx as f64) * theta.cos();
        let curr_y = cy as f64 + (ry as f64) * theta.sin();

        draw_brush_line(
            buffer,
            prev_x as usize,
            prev_y as usize,
            curr_x as usize,
            curr_y as usize,
            color,
            brush_size,
        );

        prev_x = curr_x;
        prev_y = curr_y;
    }
}

/// Draw a triangle in the bounding box from drag start to end
/// If dragging upward: apex at top (pointing up)
/// If dragging downward: apex at bottom (pointing down)
pub fn draw_shape_triangle(
    buffer: &mut [u32],
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: u32,
    brush_size: usize,
) {
    let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
    let pointing_up = y2 < y1; // Dragging upward = triangle points up

    let mid_x = (left + right) / 2;

    if pointing_up {
        // Apex at top, base at bottom (pointing up)
        let apex_x = mid_x;
        let apex_y = top;
        let base_y = bottom;

        draw_brush_line(buffer, apex_x, apex_y, left, base_y, color, brush_size); // Left edge
        draw_brush_line(buffer, apex_x, apex_y, right, base_y, color, brush_size); // Right edge
        draw_brush_line(buffer, left, base_y, right, base_y, color, brush_size); // Base
    } else {
        // Apex at bottom, base at top (pointing down)
        let apex_x = mid_x;
        let apex_y = bottom;
        let base_y = top;

        draw_brush_line(buffer, apex_x, apex_y, left, base_y, color, brush_size); // Left edge
        draw_brush_line(buffer, apex_x, apex_y, right, base_y, color, brush_size); // Right edge
        draw_brush_line(buffer, left, base_y, right, base_y, color, brush_size); // Base
    }
}

pub fn get_clicked_color_index_bottom(x: usize, y: usize) -> Option<usize> {
    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;
    if y < row1_y || y >= row1_y + BUTTON_SIZE {
        return None;
    }
    for i in 0..COLOR_PALETTE.len() {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);
        if x >= bx && x < bx + BUTTON_SIZE {
            return Some(i);
        }
    }
    None
}

/// Returns which tool button was clicked, if any
pub fn get_clicked_tool(x: usize, y: usize) -> Option<ToolMode> {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    if y < row2_y || y >= row2_y + BUTTON_SIZE {
        return None;
    }

    let tools = [
        ToolMode::Brush,
        ToolMode::Line,
        ToolMode::Square,
        ToolMode::Rectangle,
        ToolMode::Circle,
        ToolMode::Oval,
        ToolMode::Triangle,
    ];

    for (i, &tool) in tools.iter().enumerate() {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);
        if x >= bx && x < bx + BUTTON_SIZE {
            return Some(tool);
        }
    }
    None
}

pub fn is_in_minus_button(x: usize, y: usize) -> bool {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let size_display_x = BUTTON_MARGIN + 7 * (BUTTON_SIZE + BUTTON_MARGIN) + BUTTON_MARGIN;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    x >= minus_x && x < minus_x + BUTTON_SIZE && y >= row2_y && y < row2_y + BUTTON_SIZE
}

pub fn is_in_plus_button(x: usize, y: usize) -> bool {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let size_display_x = BUTTON_MARGIN + 7 * (BUTTON_SIZE + BUTTON_MARGIN) + BUTTON_MARGIN;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    let plus_x = minus_x + BUTTON_SIZE + BUTTON_MARGIN;
    x >= plus_x && x < plus_x + BUTTON_SIZE && y >= row2_y && y < row2_y + BUTTON_SIZE
}
