use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const WHITE: u32 = 0xFFFFFF;
const BLACK: u32 = 0x000000;

fn main() {
    // Initialize canvas buffer with white pixels
    let mut buffer: Vec<u32> = vec![WHITE; WIDTH * HEIGHT];

    let mut window = Window::new(
        "displai - v0.0",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    // Limit update rate to ~60fps
    window.set_target_fps(60);

    let mut is_drawing = false;
    let mut last_pos: Option<(usize, usize)> = None;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Check mouse button state
        let mouse_down = window.get_mouse_down(MouseButton::Left);

        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard) {
            let x = mx as usize;
            let y = my as usize;

            if mouse_down && x < WIDTH && y < HEIGHT {
                if is_drawing {
                    // Draw line from last position to current position
                    if let Some((lx, ly)) = last_pos {
                        draw_line(&mut buffer, lx, ly, x, y);
                    }
                } else {
                    // Start drawing - place single pixel
                    set_pixel(&mut buffer, x, y);
                }
                is_drawing = true;
                last_pos = Some((x, y));
            } else {
                is_drawing = false;
                last_pos = None;
            }
        } else {
            is_drawing = false;
            last_pos = None;
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update buffer");
    }
}

/// Set a single pixel to black
fn set_pixel(buffer: &mut [u32], x: usize, y: usize) {
    if x < WIDTH && y < HEIGHT {
        buffer[y * WIDTH + x] = BLACK;
    }
}

/// Draw a line between two points using Bresenham's algorithm
fn draw_line(buffer: &mut [u32], x0: usize, y0: usize, x1: usize, y1: usize) {
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
        if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
            buffer[y as usize * WIDTH + x as usize] = BLACK;
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
