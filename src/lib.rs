use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;
pub const WHITE: u32 = 0xFFFFFF;
pub const BLACK: u32 = 0x000000;
pub const GRAY: u32 = 0xE0E0E0;
pub const DARK_GRAY: u32 = 0x808080;
pub const RED: u32 = 0xE04040;
pub const BLUE: u32 = 0x4040E0;

pub const COLOR_PALETTE: [u32; 13] = [
    0x000000, // Black (default)
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

pub fn run() {
    let mut buffer: Vec<u32> = vec![WHITE; WIDTH * HEIGHT];

    let mut window = Window::new("displai - v0.1", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut is_drawing = false;
    let mut last_pos: Option<(usize, usize)> = None;
    let mut mouse_was_down = false;
    let mut selected_color_index: usize = 0;
    let mut eraser_active: bool = false;
    let mut brush_size: usize = DEFAULT_BRUSH_SIZE;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_title_bar(&mut buffer);
        draw_bottom_toolbar(&mut buffer, selected_color_index, eraser_active, brush_size);

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
                    eraser_active = false;
                }
                if is_in_eraser_button(x, y) {
                    eraser_active = true;
                }
                if is_in_minus_button(x, y) && brush_size > MIN_BRUSH_SIZE {
                    brush_size -= 1;
                }
                if is_in_plus_button(x, y) && brush_size < MAX_BRUSH_SIZE {
                    brush_size += 1;
                }
            }

            let pen_color = if eraser_active {
                WHITE
            } else {
                COLOR_PALETTE[selected_color_index]
            };

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
    eraser_active: bool,
    brush_size: usize,
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

    // Row 1: 13 color buttons
    let row1_y = toolbar_top + BUTTON_MARGIN;
    for (i, &color) in COLOR_PALETTE.iter().enumerate() {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);
        draw_button(buffer, bx, row1_y, color);

        // Draw border to indicate selection
        if !eraser_active && i == selected_color_index {
            draw_button_border(buffer, bx, row1_y, WHITE);
        } else {
            draw_button_border(buffer, bx, row1_y, DARK_GRAY);
        }
    }

    // Row 2: Eraser button + size display + [-] [+] buttons
    let row2_y = toolbar_top + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;

    // Eraser button
    let eraser_x = BUTTON_MARGIN;
    draw_button(buffer, eraser_x, row2_y, WHITE);
    draw_eraser_icon(buffer, eraser_x, row2_y);
    if eraser_active {
        draw_button_border(buffer, eraser_x, row2_y, 0x4040E0); // Blue border when active
    } else {
        draw_button_border(buffer, eraser_x, row2_y, DARK_GRAY);
    }

    // Size display (text showing current brush size)
    let size_display_x = eraser_x + BUTTON_SIZE + BUTTON_MARGIN * 2;
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

pub fn draw_eraser_icon(buffer: &mut [u32], bx: usize, by: usize) {
    // Draw a simple "E" icon for eraser
    let padding = 6;
    let start_x = bx + padding;
    let end_x = bx + BUTTON_SIZE - padding;
    let start_y = by + padding;
    let mid_y = by + BUTTON_SIZE / 2;
    let end_y = by + BUTTON_SIZE - padding;

    // Top horizontal line
    for x in start_x..end_x {
        if x < WIDTH && start_y < HEIGHT {
            buffer[start_y * WIDTH + x] = BLACK;
        }
    }
    // Middle horizontal line
    for x in start_x..end_x {
        if x < WIDTH && mid_y < HEIGHT {
            buffer[mid_y * WIDTH + x] = BLACK;
        }
    }
    // Bottom horizontal line
    for x in start_x..end_x {
        if x < WIDTH && end_y < HEIGHT {
            buffer[end_y * WIDTH + x] = BLACK;
        }
    }
    // Vertical line
    for y in start_y..=end_y {
        if start_x < WIDTH && y < HEIGHT {
            buffer[y * WIDTH + start_x] = BLACK;
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

pub fn get_clicked_color_index_bottom(x: usize, y: usize) -> Option<usize> {
    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;
    if y < row1_y || y >= row1_y + BUTTON_SIZE {
        return None;
    }
    for i in 0..13 {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);
        if x >= bx && x < bx + BUTTON_SIZE {
            return Some(i);
        }
    }
    None
}

pub fn is_in_eraser_button(x: usize, y: usize) -> bool {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let eraser_x = BUTTON_MARGIN;
    x >= eraser_x && x < eraser_x + BUTTON_SIZE && y >= row2_y && y < row2_y + BUTTON_SIZE
}

pub fn is_in_minus_button(x: usize, y: usize) -> bool {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let eraser_x = BUTTON_MARGIN;
    let size_display_x = eraser_x + BUTTON_SIZE + BUTTON_MARGIN * 2;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    x >= minus_x && x < minus_x + BUTTON_SIZE && y >= row2_y && y < row2_y + BUTTON_SIZE
}

pub fn is_in_plus_button(x: usize, y: usize) -> bool {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let eraser_x = BUTTON_MARGIN;
    let size_display_x = eraser_x + BUTTON_SIZE + BUTTON_MARGIN * 2;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    let plus_x = minus_x + BUTTON_SIZE + BUTTON_MARGIN;
    x >= plus_x && x < plus_x + BUTTON_SIZE && y >= row2_y && y < row2_y + BUTTON_SIZE
}
