//! Drawing primitives and shape rendering for the displai application.
//!
//! This module handles:
//! - Basic drawing primitives (pixels, lines, circles)
//! - Shape drawing (square, rectangle, circle, oval, triangle)
//! - Fill functions for shapes
//! - Canvas clearing

use crate::{ToolMode, CANVAS_BOTTOM, CANVAS_TOP, WHITE, WIDTH};

/// Set a single pixel, checking canvas bounds
pub fn set_pixel(buffer: &mut [u32], x: usize, y: usize, color: u32) {
    if x < WIDTH && (CANVAS_TOP..CANVAS_BOTTOM).contains(&y) {
        buffer[y * WIDTH + x] = color;
    }
}

/// Draw a line using Bresenham's algorithm
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

/// Draw a filled circle at the given center point
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

/// Draw a brush stroke line (circles along a line path)
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

/// Clear the canvas area to white
pub fn clear_canvas(buffer: &mut [u32]) {
    for y in CANVAS_TOP..CANVAS_BOTTOM {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = WHITE;
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

/// Draw a shape with optional edge and fill colors
/// Fill is drawn first, then edge on top
pub fn draw_shape_with_fill(
    buffer: &mut [u32],
    tool: ToolMode,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    edge_color: Option<u32>,
    fill_color: Option<u32>,
    brush_size: usize,
) {
    // Draw fill first (if any)
    if let Some(fill) = fill_color {
        match tool {
            ToolMode::Brush | ToolMode::Line => {
                // Lines don't have fill
            }
            ToolMode::Square => {
                fill_square(buffer, x1, y1, x2, y2, fill);
            }
            ToolMode::Rectangle => {
                fill_rectangle(buffer, x1, y1, x2, y2, fill);
            }
            ToolMode::Circle => {
                fill_circle(buffer, x1, y1, x2, y2, fill);
            }
            ToolMode::Oval => {
                fill_oval(buffer, x1, y1, x2, y2, fill);
            }
            ToolMode::Triangle => {
                fill_triangle(buffer, x1, y1, x2, y2, fill);
            }
        }
    }

    // Draw edge on top (if any)
    if let Some(edge) = edge_color {
        draw_shape(buffer, tool, x1, y1, x2, y2, edge, brush_size);
    }
}

/// Fill a square region (largest square that fits in drag bounds)
pub fn fill_square(buffer: &mut [u32], x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
    let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    let width = right - left;
    let height = bottom - top;
    let side = width.min(height);

    for y in top..=top + side {
        for x in left..=left + side {
            set_pixel(buffer, x, y, color);
        }
    }
}

/// Fill a rectangle region
pub fn fill_rectangle(buffer: &mut [u32], x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
    let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    for y in top..=bottom {
        for x in left..=right {
            set_pixel(buffer, x, y, color);
        }
    }
}

/// Fill a circle region
pub fn fill_circle(buffer: &mut [u32], x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
    let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    let width = right - left;
    let height = bottom - top;
    let diameter = width.min(height);
    let radius = diameter as f64 / 2.0;

    let cx = left as f64 + diameter as f64 / 2.0;
    let cy = top as f64 + diameter as f64 / 2.0;

    for y in top..=top + diameter {
        for x in left..=left + diameter {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            if dx * dx + dy * dy <= radius * radius {
                set_pixel(buffer, x, y, color);
            }
        }
    }
}

/// Fill an oval region
pub fn fill_oval(buffer: &mut [u32], x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
    let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    let cx = (left + right) as f64 / 2.0;
    let cy = (top + bottom) as f64 / 2.0;
    let rx = (right - left) as f64 / 2.0;
    let ry = (bottom - top) as f64 / 2.0;

    if rx == 0.0 || ry == 0.0 {
        return;
    }

    for y in top..=bottom {
        for x in left..=right {
            let dx = (x as f64 - cx) / rx;
            let dy = (y as f64 - cy) / ry;
            if dx * dx + dy * dy <= 1.0 {
                set_pixel(buffer, x, y, color);
            }
        }
    }
}

/// Fill a triangle region using scanline algorithm
pub fn fill_triangle(buffer: &mut [u32], x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
    let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
    let pointing_up = y2 < y1;

    let mid_x = (left + right) / 2;

    if pointing_up {
        // Apex at top, base at bottom
        let apex = (mid_x as f64, top as f64);
        let left_base = (left as f64, bottom as f64);
        let right_base = (right as f64, bottom as f64);

        for y in top..=bottom {
            let yf = y as f64;
            // Find x bounds at this y
            let t = if bottom != top {
                (yf - top as f64) / (bottom - top) as f64
            } else {
                0.0
            };
            let x_left = apex.0 + t * (left_base.0 - apex.0);
            let x_right = apex.0 + t * (right_base.0 - apex.0);

            for x in (x_left as usize)..=(x_right as usize) {
                set_pixel(buffer, x, y, color);
            }
        }
    } else {
        // Apex at bottom, base at top
        let apex = (mid_x as f64, bottom as f64);
        let left_base = (left as f64, top as f64);
        let right_base = (right as f64, top as f64);

        for y in top..=bottom {
            let yf = y as f64;
            let t = if bottom != top {
                (bottom as f64 - yf) / (bottom - top) as f64
            } else {
                0.0
            };
            let x_left = apex.0 + t * (left_base.0 - apex.0);
            let x_right = apex.0 + t * (right_base.0 - apex.0);

            for x in (x_left as usize)..=(x_right as usize) {
                set_pixel(buffer, x, y, color);
            }
        }
    }
}

/// Draw a square from corner to corner (largest square that fits in drag bounds)
pub fn draw_shape_square(
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
    let side = width.min(height);

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
        draw_circle(
            buffer,
            (left + right) / 2,
            (top + bottom) / 2,
            brush_size,
            color,
        );
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
        draw_brush_line(buffer, left, base_y, right, base_y, color, brush_size);
    // Base
    } else {
        // Apex at bottom, base at top (pointing down)
        let apex_x = mid_x;
        let apex_y = bottom;
        let base_y = top;

        draw_brush_line(buffer, apex_x, apex_y, left, base_y, color, brush_size); // Left edge
        draw_brush_line(buffer, apex_x, apex_y, right, base_y, color, brush_size); // Right edge
        draw_brush_line(buffer, left, base_y, right, base_y, color, brush_size);
        // Base
    }
}
