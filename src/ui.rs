//! UI rendering and hit detection for the displai application.
//!
//! This module handles:
//! - Title bar and toolbar rendering
//! - Button drawing (color palette, tools, etc.)
//! - Icon rendering for tools
//! - Hit detection for clickable UI elements

use crate::{
    ToolMode, BLACK, BUTTON_MARGIN, BUTTON_SIZE, CANVAS_BOTTOM, COLOR_PALETTE, DARK_GRAY, GRAY,
    HEIGHT, TITLE_BAR_HEIGHT, TOOLBAR_ROW_HEIGHT, WHITE, WIDTH,
};

/// Draw the title bar with close button
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
    draw_button(buffer, close_x, close_y, crate::RED);
    draw_x(buffer, close_x, close_y);
}

/// Draw a filled button at the given position
pub fn draw_button(buffer: &mut [u32], bx: usize, by: usize, color: u32) {
    for y in by..by + BUTTON_SIZE {
        for x in bx..bx + BUTTON_SIZE {
            if x < WIDTH && y < HEIGHT {
                buffer[y * WIDTH + x] = color;
            }
        }
    }
}

/// Draw a border around a button
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

/// Draw an inner border (1 pixel inside the button)
pub fn draw_button_inner_border(buffer: &mut [u32], bx: usize, by: usize, color: u32) {
    // Draw a border 1 pixel inside the button
    for x in (bx + 1)..(bx + BUTTON_SIZE - 1) {
        if x < WIDTH {
            buffer[(by + 1) * WIDTH + x] = color;
            buffer[(by + BUTTON_SIZE - 2) * WIDTH + x] = color;
        }
    }
    for y in (by + 1)..(by + BUTTON_SIZE - 1) {
        if y < HEIGHT {
            buffer[y * WIDTH + bx + 1] = color;
            buffer[y * WIDTH + bx + BUTTON_SIZE - 2] = color;
        }
    }
}

/// Draw the transparent color button with checkerboard pattern
pub fn draw_transparent_button(
    buffer: &mut [u32],
    bx: usize,
    by: usize,
    edge_selected: bool,
    fill_selected: bool,
) {
    // Draw checkerboard pattern
    for dy in 0..BUTTON_SIZE {
        for dx in 0..BUTTON_SIZE {
            let px = bx + dx;
            let py = by + dy;
            if px < WIDTH && py < HEIGHT {
                let checker = ((dx / 4) + (dy / 4)) % 2 == 0;
                buffer[py * WIDTH + px] = if checker { WHITE } else { GRAY };
            }
        }
    }

    // Draw border based on selection
    if edge_selected && fill_selected {
        draw_button_border(buffer, bx, by, WHITE);
        draw_button_inner_border(buffer, bx, by, 0x40E040);
    } else if edge_selected {
        draw_button_border(buffer, bx, by, WHITE);
    } else if fill_selected {
        draw_button_border(buffer, bx, by, 0x40E040);
    } else {
        draw_button_border(buffer, bx, by, DARK_GRAY);
    }
}

/// Check if click is on transparent button
pub fn is_in_transparent_button(x: usize, y: usize) -> bool {
    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;
    let transparent_x = BUTTON_MARGIN + 14 * (BUTTON_SIZE + BUTTON_MARGIN);
    x >= transparent_x && x < transparent_x + BUTTON_SIZE && y >= row1_y && y < row1_y + BUTTON_SIZE
}

/// Draw edge/fill color indicator showing current colors
pub fn draw_edge_fill_indicator(
    buffer: &mut [u32],
    x: usize,
    y: usize,
    edge_color_index: Option<usize>,
    fill_color_index: Option<usize>,
) {
    let size = 20;
    let offset = 8;

    // Draw fill color square (behind, offset)
    if let Some(fill_idx) = fill_color_index {
        let fill_color = COLOR_PALETTE[fill_idx];
        for dy in 0..size {
            for dx in 0..size {
                let px = x + offset + dx;
                let py = y + offset + dy;
                if px < WIDTH && py < HEIGHT {
                    buffer[py * WIDTH + px] = fill_color;
                }
            }
        }
        // Border for fill square
        for dx in 0..size {
            buffer[(y + offset) * WIDTH + x + offset + dx] = DARK_GRAY;
            buffer[(y + offset + size - 1) * WIDTH + x + offset + dx] = DARK_GRAY;
        }
        for dy in 0..size {
            buffer[(y + offset + dy) * WIDTH + x + offset] = DARK_GRAY;
            buffer[(y + offset + dy) * WIDTH + x + offset + size - 1] = DARK_GRAY;
        }
    } else {
        // Draw "no fill" indicator (checkerboard for transparent)
        for dy in 0..size {
            for dx in 0..size {
                let px = x + offset + dx;
                let py = y + offset + dy;
                if px < WIDTH && py < HEIGHT {
                    let checker = ((dx / 4) + (dy / 4)) % 2 == 0;
                    buffer[py * WIDTH + px] = if checker { WHITE } else { GRAY };
                }
            }
        }
        // Border
        for dx in 0..size {
            buffer[(y + offset) * WIDTH + x + offset + dx] = DARK_GRAY;
            buffer[(y + offset + size - 1) * WIDTH + x + offset + dx] = DARK_GRAY;
        }
        for dy in 0..size {
            buffer[(y + offset + dy) * WIDTH + x + offset] = DARK_GRAY;
            buffer[(y + offset + dy) * WIDTH + x + offset + size - 1] = DARK_GRAY;
        }
    }

    // Draw edge color square (front, at origin)
    if let Some(edge_idx) = edge_color_index {
        let edge_color = COLOR_PALETTE[edge_idx];
        for dy in 0..size {
            for dx in 0..size {
                let px = x + dx;
                let py = y + dy;
                if px < WIDTH && py < HEIGHT {
                    buffer[py * WIDTH + px] = edge_color;
                }
            }
        }
        // Border for edge square
        let border_color = if edge_color == WHITE {
            DARK_GRAY
        } else {
            WHITE
        };
        for dx in 0..size {
            buffer[y * WIDTH + x + dx] = border_color;
            buffer[(y + size - 1) * WIDTH + x + dx] = border_color;
        }
        for dy in 0..size {
            buffer[(y + dy) * WIDTH + x] = border_color;
            buffer[(y + dy) * WIDTH + x + size - 1] = border_color;
        }
    } else {
        // Draw checkerboard for transparent edge
        for dy in 0..size {
            for dx in 0..size {
                let px = x + dx;
                let py = y + dy;
                if px < WIDTH && py < HEIGHT {
                    let checker = ((dx / 4) + (dy / 4)) % 2 == 0;
                    buffer[py * WIDTH + px] = if checker { WHITE } else { GRAY };
                }
            }
        }
        // Border
        for dx in 0..size {
            buffer[y * WIDTH + x + dx] = DARK_GRAY;
            buffer[(y + size - 1) * WIDTH + x + dx] = DARK_GRAY;
        }
        for dy in 0..size {
            buffer[(y + dy) * WIDTH + x] = DARK_GRAY;
            buffer[(y + dy) * WIDTH + x + size - 1] = DARK_GRAY;
        }
    }
}

/// Check if click is on the fill indicator (to clear fill)
pub fn is_in_fill_indicator(x: usize, y: usize) -> bool {
    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;
    let transparent_x = BUTTON_MARGIN + 14 * (BUTTON_SIZE + BUTTON_MARGIN);
    let indicator_x = transparent_x + BUTTON_SIZE + BUTTON_MARGIN * 2;
    let offset = 8;
    let size = 20;

    // Check if in the fill square area (the back square)
    x >= indicator_x + offset
        && x < indicator_x + offset + size
        && y >= row1_y + offset
        && y < row1_y + offset + size
}

/// Draw an X icon (for close button)
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

/// Check if coordinates are within the close button
pub fn is_in_close_button(x: usize, y: usize) -> bool {
    let bx = WIDTH - BUTTON_SIZE - BUTTON_MARGIN;
    let by = BUTTON_MARGIN;
    x >= bx && x < bx + BUTTON_SIZE && y >= by && y < by + BUTTON_SIZE
}

/// Get color index from title bar color palette (legacy, not currently used)
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

/// Draw the bottom toolbar with color palette and tool buttons
pub fn draw_bottom_toolbar(
    buffer: &mut [u32],
    edge_color_index: Option<usize>,
    fill_color_index: Option<usize>,
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

    // Row 1: 14 color buttons + transparent button + edge/fill indicator
    let row1_y = toolbar_top + BUTTON_MARGIN;
    for (i, &color) in COLOR_PALETTE.iter().enumerate() {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);
        draw_button(buffer, bx, row1_y, color);

        // Draw border: white/blue for edge selection, green for fill selection
        let is_edge = edge_color_index == Some(i);
        let is_fill = fill_color_index == Some(i);

        if is_edge && is_fill {
            // Both edge and fill: white outer, green inner
            let border_color = if color == WHITE { 0x4040E0 } else { WHITE };
            draw_button_border(buffer, bx, row1_y, border_color);
            draw_button_inner_border(buffer, bx, row1_y, 0x40E040); // Green inner for fill
        } else if is_edge {
            let border_color = if color == WHITE { 0x4040E0 } else { WHITE };
            draw_button_border(buffer, bx, row1_y, border_color);
        } else if is_fill {
            draw_button_border(buffer, bx, row1_y, 0x40E040); // Green for fill
        } else {
            draw_button_border(buffer, bx, row1_y, DARK_GRAY);
        }
    }

    // Transparent button (after 14 color buttons)
    let transparent_x = BUTTON_MARGIN + 14 * (BUTTON_SIZE + BUTTON_MARGIN);
    draw_transparent_button(
        buffer,
        transparent_x,
        row1_y,
        edge_color_index.is_none(),
        fill_color_index.is_none(),
    );

    // Edge/Fill indicator (after transparent button)
    let indicator_x = transparent_x + BUTTON_SIZE + BUTTON_MARGIN * 2;
    draw_edge_fill_indicator(
        buffer,
        indicator_x,
        row1_y,
        edge_color_index,
        fill_color_index,
    );

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

    // Clear button
    let clear_x = plus_x + BUTTON_SIZE + BUTTON_MARGIN * 2;
    draw_button(buffer, clear_x, row2_y, 0xC04040); // Reddish color
    draw_clear_icon(buffer, clear_x, row2_y);
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
                buffer[end_y * WIDTH + start_x + i] = BLACK; // bottom
                buffer[(start_y + i) * WIDTH + start_x] = BLACK; // left
                buffer[(start_y + i) * WIDTH + end_x] = BLACK; // right
            }
        }
        ToolMode::Rectangle => {
            // Rectangle (wider than tall)
            let rect_start_y = start_y + 3;
            let rect_end_y = end_y - 3;
            for x in start_x..=end_x {
                buffer[rect_start_y * WIDTH + x] = BLACK; // top
                buffer[rect_end_y * WIDTH + x] = BLACK; // bottom
            }
            for y in rect_start_y..=rect_end_y {
                buffer[y * WIDTH + start_x] = BLACK; // left
                buffer[y * WIDTH + end_x] = BLACK; // right
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
                let x = apex_x as isize
                    - (i as isize * (apex_x - left_x) as isize / (base_y - apex_y) as isize);
                let y = apex_y + i;
                if x >= 0 && (x as usize) < WIDTH && y < HEIGHT {
                    buffer[y * WIDTH + x as usize] = BLACK;
                }
            }
            // Right edge
            for i in 0..=(base_y - apex_y) {
                let x = apex_x as isize
                    + (i as isize * (right_x - apex_x) as isize / (base_y - apex_y) as isize);
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

/// Draw a minus icon
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

/// Draw a plus icon
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

/// Draw a clear icon (X shape)
pub fn draw_clear_icon(buffer: &mut [u32], bx: usize, by: usize) {
    // Draw an X to represent clear
    let padding = 6;
    let start = padding;
    let end = BUTTON_SIZE - padding;

    for i in 0..(end - start) {
        // Top-left to bottom-right diagonal
        let x1 = bx + start + i;
        let y1 = by + start + i;
        if x1 < WIDTH && y1 < HEIGHT {
            buffer[y1 * WIDTH + x1] = WHITE;
        }

        // Top-right to bottom-left diagonal
        let x2 = bx + end - 1 - i;
        let y2 = by + start + i;
        if x2 < WIDTH && y2 < HEIGHT {
            buffer[y2 * WIDTH + x2] = WHITE;
        }
    }
}

/// Draw a box displaying the current brush size
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

/// Draw a number using a simple 5x7 pixel font
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

/// Get color index from bottom toolbar color palette
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

/// Check if coordinates are within the minus button
pub fn is_in_minus_button(x: usize, y: usize) -> bool {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let size_display_x = BUTTON_MARGIN + 7 * (BUTTON_SIZE + BUTTON_MARGIN) + BUTTON_MARGIN;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    x >= minus_x && x < minus_x + BUTTON_SIZE && y >= row2_y && y < row2_y + BUTTON_SIZE
}

/// Check if coordinates are within the plus button
pub fn is_in_plus_button(x: usize, y: usize) -> bool {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let size_display_x = BUTTON_MARGIN + 7 * (BUTTON_SIZE + BUTTON_MARGIN) + BUTTON_MARGIN;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    let plus_x = minus_x + BUTTON_SIZE + BUTTON_MARGIN;
    x >= plus_x && x < plus_x + BUTTON_SIZE && y >= row2_y && y < row2_y + BUTTON_SIZE
}

/// Check if coordinates are within the clear button
pub fn is_in_clear_button(x: usize, y: usize) -> bool {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let size_display_x = BUTTON_MARGIN + 7 * (BUTTON_SIZE + BUTTON_MARGIN) + BUTTON_MARGIN;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    let plus_x = minus_x + BUTTON_SIZE + BUTTON_MARGIN;
    let clear_x = plus_x + BUTTON_SIZE + BUTTON_MARGIN * 2;
    x >= clear_x && x < clear_x + BUTTON_SIZE && y >= row2_y && y < row2_y + BUTTON_SIZE
}
