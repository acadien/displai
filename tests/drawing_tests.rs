use displai::*;

// Helper to create a fresh white buffer
fn new_buffer() -> Vec<u32> {
    vec![WHITE; WIDTH * HEIGHT]
}

// ===================
// Drawing Tests
// ===================

#[test]
fn test_set_pixel_adds_content_to_buffer() {
    let mut buffer = new_buffer();
    let x = 100;
    let y = 100;

    // Buffer should start white
    assert_eq!(buffer[y * WIDTH + x], WHITE);

    // After set_pixel, should be the specified color
    set_pixel(&mut buffer, x, y, BLACK);
    assert_eq!(buffer[y * WIDTH + x], BLACK);
}

#[test]
fn test_set_pixel_with_different_colors() {
    let mut buffer = new_buffer();

    set_pixel(&mut buffer, 200, 200, RED);
    assert_eq!(buffer[200 * WIDTH + 200], RED);

    set_pixel(&mut buffer, 201, 200, BLACK);
    assert_eq!(buffer[200 * WIDTH + 201], BLACK);
}

#[test]
fn test_set_pixel_respects_title_bar_boundary() {
    let mut buffer = new_buffer();

    // Attempt to draw in title bar area - should not modify buffer
    set_pixel(&mut buffer, 100, 10, BLACK);
    assert_eq!(buffer[10 * WIDTH + 100], WHITE);

    // Draw just below title bar - should work
    set_pixel(&mut buffer, 100, CANVAS_TOP, BLACK);
    assert_eq!(buffer[CANVAS_TOP * WIDTH + 100], BLACK);
}

#[test]
fn test_set_pixel_respects_bottom_toolbar_boundary() {
    let mut buffer = new_buffer();

    // Attempt to draw in bottom toolbar area - should not modify buffer
    set_pixel(&mut buffer, 100, CANVAS_BOTTOM, BLACK);
    assert_eq!(buffer[CANVAS_BOTTOM * WIDTH + 100], WHITE);

    set_pixel(&mut buffer, 100, CANVAS_BOTTOM + 10, BLACK);
    assert_eq!(buffer[(CANVAS_BOTTOM + 10) * WIDTH + 100], WHITE);

    // Draw just above bottom toolbar - should work
    set_pixel(&mut buffer, 100, CANVAS_BOTTOM - 1, BLACK);
    assert_eq!(buffer[(CANVAS_BOTTOM - 1) * WIDTH + 100], BLACK);
}

#[test]
fn test_set_pixel_respects_canvas_boundaries() {
    let mut buffer = new_buffer();

    // These should not panic or modify anything outside bounds
    set_pixel(&mut buffer, WIDTH + 10, 100, BLACK);
    set_pixel(&mut buffer, 100, HEIGHT + 10, BLACK);

    // Buffer should remain unchanged (all white in drawable area)
    assert_eq!(buffer[100 * WIDTH + 100], WHITE);
}

#[test]
fn test_draw_line_horizontal() {
    let mut buffer = new_buffer();
    let y = 100;

    draw_line(&mut buffer, 50, y, 60, y, BLACK);

    // All pixels from x=50 to x=60 should be black
    for x in 50..=60 {
        assert_eq!(
            buffer[y * WIDTH + x],
            BLACK,
            "Pixel at ({}, {}) should be black",
            x,
            y
        );
    }
}

#[test]
fn test_draw_line_vertical() {
    let mut buffer = new_buffer();
    let x = 100;

    draw_line(&mut buffer, x, 50, x, 60, BLACK);

    // All pixels from y=50 to y=60 should be black
    for y in 50..=60 {
        assert_eq!(
            buffer[y * WIDTH + x],
            BLACK,
            "Pixel at ({}, {}) should be black",
            x,
            y
        );
    }
}

#[test]
fn test_draw_line_diagonal() {
    let mut buffer = new_buffer();

    draw_line(&mut buffer, 50, 50, 55, 55, BLACK);

    // Diagonal line should have pixels set
    for i in 0..=5 {
        assert_eq!(buffer[(50 + i) * WIDTH + (50 + i)], BLACK);
    }
}

#[test]
fn test_draw_line_respects_title_bar() {
    let mut buffer = new_buffer();

    // Draw line that would cross into title bar
    draw_line(&mut buffer, 100, 20, 100, 50, BLACK);

    // Pixels in title bar should remain white
    for y in 20..CANVAS_TOP {
        assert_eq!(buffer[y * WIDTH + 100], WHITE);
    }

    // Pixels below title bar should be black
    for y in CANVAS_TOP..=50 {
        assert_eq!(buffer[y * WIDTH + 100], BLACK);
    }
}

#[test]
fn test_draw_line_respects_bottom_toolbar() {
    let mut buffer = new_buffer();

    // Draw line that would cross into bottom toolbar
    draw_line(
        &mut buffer,
        100,
        CANVAS_BOTTOM - 10,
        100,
        CANVAS_BOTTOM + 10,
        BLACK,
    );

    // Pixels above bottom toolbar should be black
    for y in (CANVAS_BOTTOM - 10)..CANVAS_BOTTOM {
        assert_eq!(buffer[y * WIDTH + 100], BLACK, "y={} should be black", y);
    }

    // Pixels in bottom toolbar should remain white
    for y in CANVAS_BOTTOM..=(CANVAS_BOTTOM + 10) {
        assert_eq!(buffer[y * WIDTH + 100], WHITE, "y={} should be white", y);
    }
}

// ===================
// Circle Drawing Tests
// ===================

#[test]
fn test_draw_circle_size_1() {
    let mut buffer = new_buffer();
    let cx = 100;
    let cy = 100;

    draw_circle(&mut buffer, cx, cy, 1, BLACK);

    // Size 1 should draw a single pixel
    assert_eq!(buffer[cy * WIDTH + cx], BLACK);

    // Surrounding pixels should be white
    assert_eq!(buffer[(cy - 1) * WIDTH + cx], WHITE);
    assert_eq!(buffer[(cy + 1) * WIDTH + cx], WHITE);
    assert_eq!(buffer[cy * WIDTH + (cx - 1)], WHITE);
    assert_eq!(buffer[cy * WIDTH + (cx + 1)], WHITE);
}

#[test]
fn test_draw_circle_size_3() {
    let mut buffer = new_buffer();
    let cx = 100;
    let cy = 100;

    draw_circle(&mut buffer, cx, cy, 3, BLACK);

    // Size 3 has radius 1, should be a small cross/circle shape
    // Center should be black
    assert_eq!(buffer[cy * WIDTH + cx], BLACK);

    // Cardinal directions should be black (radius 1)
    assert_eq!(buffer[(cy - 1) * WIDTH + cx], BLACK);
    assert_eq!(buffer[(cy + 1) * WIDTH + cx], BLACK);
    assert_eq!(buffer[cy * WIDTH + (cx - 1)], BLACK);
    assert_eq!(buffer[cy * WIDTH + (cx + 1)], BLACK);
}

#[test]
fn test_draw_circle_size_5() {
    let mut buffer = new_buffer();
    let cx = 100;
    let cy = 100;

    draw_circle(&mut buffer, cx, cy, 5, BLACK);

    // Size 5 has radius 2
    // Center and surrounding should be filled
    assert_eq!(buffer[cy * WIDTH + cx], BLACK);
    assert_eq!(buffer[(cy - 1) * WIDTH + cx], BLACK);
    assert_eq!(buffer[(cy + 1) * WIDTH + cx], BLACK);
    assert_eq!(buffer[(cy - 2) * WIDTH + cx], BLACK);
    assert_eq!(buffer[(cy + 2) * WIDTH + cx], BLACK);
}

#[test]
fn test_draw_circle_respects_boundaries() {
    let mut buffer = new_buffer();

    // Draw circle near top boundary - should not draw in title bar
    draw_circle(&mut buffer, 100, CANVAS_TOP + 2, 7, BLACK);

    // Pixels in title bar should remain white
    for y in 0..CANVAS_TOP {
        assert_eq!(
            buffer[y * WIDTH + 100],
            WHITE,
            "Title bar pixel should be white at y={}",
            y
        );
    }

    // Draw circle near bottom boundary - should not draw in bottom toolbar
    let mut buffer2 = new_buffer();
    draw_circle(&mut buffer2, 100, CANVAS_BOTTOM - 3, 7, BLACK);

    // Pixels in bottom toolbar should remain white
    for y in CANVAS_BOTTOM..HEIGHT {
        assert_eq!(
            buffer2[y * WIDTH + 100],
            WHITE,
            "Bottom toolbar pixel should be white at y={}",
            y
        );
    }
}

#[test]
fn test_draw_circle_at_various_sizes() {
    let mut buffer = new_buffer();

    // Test that circles of various sizes don't panic
    for size in MIN_BRUSH_SIZE..=MAX_BRUSH_SIZE {
        draw_circle(&mut buffer, 400, 300, size, BLACK);
    }

    // Center should definitely be black after all those circles
    assert_eq!(buffer[300 * WIDTH + 400], BLACK);
}

// Helper to count non-white pixels in a buffer
fn count_drawn_pixels(buffer: &[u32]) -> usize {
    buffer.iter().filter(|&&pixel| pixel != WHITE).count()
}

#[test]
fn test_brush_size_increase_draws_more_pixels() {
    let cx = 400;
    let cy = 300;
    let mut prev_pixel_count = 0;

    // Test that increasing brush size draws more pixels
    for size in MIN_BRUSH_SIZE..=MAX_BRUSH_SIZE {
        let mut buffer = new_buffer();
        draw_circle(&mut buffer, cx, cy, size, BLACK);
        let pixel_count = count_drawn_pixels(&buffer);

        assert!(
            pixel_count >= prev_pixel_count,
            "Brush size {} drew {} pixels, but size {} drew {} pixels (should be >= previous)",
            size,
            pixel_count,
            size - 1,
            prev_pixel_count
        );

        prev_pixel_count = pixel_count;
    }
}

#[test]
fn test_brush_size_decrease_draws_fewer_pixels() {
    let cx = 400;
    let cy = 300;
    let mut prev_pixel_count = usize::MAX;

    // Test that decreasing brush size draws fewer pixels
    for size in (MIN_BRUSH_SIZE..=MAX_BRUSH_SIZE).rev() {
        let mut buffer = new_buffer();
        draw_circle(&mut buffer, cx, cy, size, BLACK);
        let pixel_count = count_drawn_pixels(&buffer);

        assert!(
            pixel_count <= prev_pixel_count,
            "Brush size {} drew {} pixels, but size {} drew {} pixels (should be <= previous)",
            size,
            pixel_count,
            size + 1,
            prev_pixel_count
        );

        prev_pixel_count = pixel_count;
    }
}

#[test]
fn test_brush_size_pixel_count_progression() {
    let cx = 400;
    let cy = 300;

    // Collect pixel counts for all sizes
    let mut pixel_counts: Vec<(usize, usize)> = Vec::new();

    for size in MIN_BRUSH_SIZE..=MAX_BRUSH_SIZE {
        let mut buffer = new_buffer();
        draw_circle(&mut buffer, cx, cy, size, BLACK);
        let count = count_drawn_pixels(&buffer);
        pixel_counts.push((size, count));
    }

    // Verify STRICTLY increasing pixel counts for each size increase
    // Each brush size should draw MORE pixels than the previous size
    for i in 1..pixel_counts.len() {
        let (prev_size, prev_count) = pixel_counts[i - 1];
        let (curr_size, curr_count) = pixel_counts[i];

        assert!(
            curr_count > prev_count,
            "Brush size {} drew {} pixels, same as size {} ({} pixels). Each size increase should draw more pixels!",
            curr_size,
            curr_count,
            prev_size,
            prev_count
        );
    }

    // Verify size 1 draws exactly 1 pixel
    assert_eq!(
        pixel_counts[0].1, 1,
        "Brush size 1 should draw exactly 1 pixel, got {}",
        pixel_counts[0].1
    );

    // Verify max size draws more than min size
    assert!(
        pixel_counts.last().unwrap().1 > pixel_counts[0].1,
        "Max brush size should draw more pixels than min size"
    );
}

// ===================
// Shape Drawing Tests
// ===================

#[test]
fn test_draw_shape_line() {
    let mut buffer = new_buffer();
    let y = CANVAS_TOP + 100;

    draw_shape(&mut buffer, ToolMode::Line, 50, y, 150, y, BLACK, 1);

    // Check that pixels along the line are drawn
    for x in 50..=150 {
        assert_eq!(buffer[y * WIDTH + x], BLACK, "Line pixel at x={} should be black", x);
    }
}

#[test]
fn test_draw_shape_rectangle() {
    let mut buffer = new_buffer();
    let x1 = 100;
    let y1 = CANVAS_TOP + 50;
    let x2 = 200;
    let y2 = CANVAS_TOP + 150;

    draw_shape_rectangle(&mut buffer, x1, y1, x2, y2, BLACK, 1);

    // Check top edge
    for x in x1..=x2 {
        assert_eq!(buffer[y1 * WIDTH + x], BLACK, "Top edge at x={}", x);
    }

    // Check bottom edge
    for x in x1..=x2 {
        assert_eq!(buffer[y2 * WIDTH + x], BLACK, "Bottom edge at x={}", x);
    }

    // Check left edge
    for y in y1..=y2 {
        assert_eq!(buffer[y * WIDTH + x1], BLACK, "Left edge at y={}", y);
    }

    // Check right edge
    for y in y1..=y2 {
        assert_eq!(buffer[y * WIDTH + x2], BLACK, "Right edge at y={}", y);
    }

    // Check interior is empty (not filled)
    assert_eq!(buffer[(y1 + 50) * WIDTH + (x1 + 50)], WHITE, "Interior should be white");
}

#[test]
fn test_draw_shape_rectangle_reverse_coords() {
    let mut buffer = new_buffer();
    // Draw with reversed coordinates (end before start)
    let x1 = 200;
    let y1 = CANVAS_TOP + 150;
    let x2 = 100;
    let y2 = CANVAS_TOP + 50;

    draw_shape_rectangle(&mut buffer, x1, y1, x2, y2, BLACK, 1);

    // Rectangle should still be drawn correctly (at same position as forward coords)
    let top = CANVAS_TOP + 50;
    let left = 100;

    assert_eq!(buffer[top * WIDTH + left], BLACK, "Top-left corner");
    assert_eq!(buffer[top * WIDTH + 200], BLACK, "Top-right corner");
}

#[test]
fn test_draw_shape_square() {
    let mut buffer = new_buffer();
    let x1 = 100;
    let y1 = CANVAS_TOP + 100;
    let x2 = 200;
    let y2 = CANVAS_TOP + 150;  // Drag defines a 100x50 rectangle

    draw_shape_square(&mut buffer, x1, y1, x2, y2, BLACK, 1);

    // The side length should be min(width, height) = min(100, 50) = 50
    // So we get a 50x50 square starting at (100, CANVAS_TOP+100)

    // Check all four corners are black
    assert_eq!(buffer[y1 * WIDTH + x1], BLACK, "Top-left corner");
    assert_eq!(buffer[y1 * WIDTH + (x1 + 50)], BLACK, "Top-right corner");
    assert_eq!(buffer[(y1 + 50) * WIDTH + x1], BLACK, "Bottom-left corner");
    assert_eq!(buffer[(y1 + 50) * WIDTH + (x1 + 50)], BLACK, "Bottom-right corner");

    // Point beyond square (at old 100x100 corner) should be white
    assert_eq!(buffer[(y1 + 100) * WIDTH + (x1 + 100)], WHITE, "Beyond square should be white");
}

#[test]
fn test_draw_shape_circle() {
    let mut buffer = new_buffer();
    // Circle bounded by rectangle from (300, 200) to (400, 300)
    // This is a 100x100 box, so diameter = 100, radius = 50
    // Center will be at (350, 250)
    let x1 = 300;
    let y1 = 200;
    let x2 = 400;
    let y2 = 300;

    draw_shape_circle(&mut buffer, x1, y1, x2, y2, BLACK, 1);

    let cx = 350;
    let cy = 250;
    let radius = 50;

    // Check some points on the circle
    // Right point
    assert_eq!(buffer[cy * WIDTH + (cx + radius)], BLACK, "Right point of circle");

    // Left point
    assert_eq!(buffer[cy * WIDTH + (cx - radius)], BLACK, "Left point of circle");

    // Top point
    assert_eq!(buffer[(cy - radius) * WIDTH + cx], BLACK, "Top point of circle");

    // Bottom point
    assert_eq!(buffer[(cy + radius) * WIDTH + cx], BLACK, "Bottom point of circle");

    // Center should be empty (circle is outline only)
    assert_eq!(buffer[cy * WIDTH + cx], WHITE, "Center should be white");
}

#[test]
fn test_draw_shape_oval() {
    let mut buffer = new_buffer();
    let x1 = 300;
    let y1 = CANVAS_TOP + 100;
    let x2 = 500;
    let y2 = CANVAS_TOP + 200;

    draw_shape_oval(&mut buffer, x1, y1, x2, y2, BLACK, 1);

    // Check points on the oval
    // Center is at (400, CANVAS_TOP+150), rx=100, ry=50
    let cx = 400;
    let cy = CANVAS_TOP + 150;

    // Right point
    assert_eq!(buffer[cy * WIDTH + (cx + 100)], BLACK, "Right point of oval");

    // Left point
    assert_eq!(buffer[cy * WIDTH + (cx - 100)], BLACK, "Left point of oval");

    // Top point
    assert_eq!(buffer[(cy - 50) * WIDTH + cx], BLACK, "Top point of oval");

    // Bottom point
    assert_eq!(buffer[(cy + 50) * WIDTH + cx], BLACK, "Bottom point of oval");

    // Center should be empty
    assert_eq!(buffer[cy * WIDTH + cx], WHITE, "Center should be white");
}

#[test]
fn test_draw_shape_triangle_pointing_down() {
    let mut buffer = new_buffer();
    let x1 = 100;
    let y1 = CANVAS_TOP + 50;
    let x2 = 200;
    let y2 = CANVAS_TOP + 150;

    // Dragging downward: triangle points down (apex at bottom, base at top)
    draw_shape_triangle(&mut buffer, x1, y1, x2, y2, BLACK, 1);

    // Apex at bottom center (150, CANVAS_TOP+150)
    let apex_x = 150;
    let apex_y = CANVAS_TOP + 150;

    // Base at top
    let base_y = CANVAS_TOP + 50;

    // Check apex is black
    assert_eq!(buffer[apex_y * WIDTH + apex_x], BLACK, "Apex of triangle (at bottom)");

    // Check base is drawn at top
    for x in x1..=x2 {
        assert_eq!(buffer[base_y * WIDTH + x], BLACK, "Base at x={}", x);
    }
}

#[test]
fn test_draw_shape_triangle_pointing_up() {
    let mut buffer = new_buffer();
    // Dragging upward: start below, end above -> triangle points up
    let x1 = 100;
    let y1 = CANVAS_TOP + 150;  // Start lower
    let x2 = 200;
    let y2 = CANVAS_TOP + 50;   // End higher (dragging up)

    draw_shape_triangle(&mut buffer, x1, y1, x2, y2, BLACK, 1);

    // Pointing up: apex at top, base at bottom
    let apex_x = 150;
    let apex_y = CANVAS_TOP + 50;
    let base_y = CANVAS_TOP + 150;

    // Check apex is at top
    assert_eq!(buffer[apex_y * WIDTH + apex_x], BLACK, "Apex of triangle (at top)");

    // Check base is at bottom
    for x in 100..=200 {
        assert_eq!(buffer[base_y * WIDTH + x], BLACK, "Base at bottom, x={}", x);
    }
}

#[test]
fn test_draw_shape_with_larger_brush() {
    let mut buffer1 = new_buffer();
    let mut buffer2 = new_buffer();

    // Draw same rectangle with different brush sizes
    draw_shape_rectangle(&mut buffer1, 100, CANVAS_TOP + 50, 200, CANVAS_TOP + 150, BLACK, 1);
    draw_shape_rectangle(&mut buffer2, 100, CANVAS_TOP + 50, 200, CANVAS_TOP + 150, BLACK, 5);

    // Larger brush should draw more pixels
    let count1 = count_drawn_pixels(&buffer1);
    let count2 = count_drawn_pixels(&buffer2);

    assert!(count2 > count1, "Larger brush ({}) should draw more pixels than smaller brush ({})", count2, count1);
}

#[test]
fn test_draw_shape_dispatcher() {
    // Test that draw_shape dispatches correctly to each shape function
    let mut buffer = new_buffer();
    let y = CANVAS_TOP + 100;

    // Line
    draw_shape(&mut buffer, ToolMode::Line, 50, y, 100, y, BLACK, 1);
    assert_eq!(buffer[y * WIDTH + 75], BLACK, "Line via dispatcher");

    // Rectangle
    let mut buffer = new_buffer();
    draw_shape(&mut buffer, ToolMode::Rectangle, 100, y, 150, y + 50, BLACK, 1);
    assert_eq!(buffer[y * WIDTH + 100], BLACK, "Rectangle via dispatcher");

    // Square
    let mut buffer = new_buffer();
    draw_shape(&mut buffer, ToolMode::Square, 100, y, 150, y + 30, BLACK, 1);
    assert_eq!(buffer[y * WIDTH + 100], BLACK, "Square via dispatcher");

    // Circle (bounding box from 350,250 to 450,350 -> 100x100 circle, center at 400,300)
    let mut buffer = new_buffer();
    draw_shape(&mut buffer, ToolMode::Circle, 350, 250, 450, 350, BLACK, 1);
    // Right edge of circle at center_x + radius = 400 + 50 = 450
    assert_eq!(buffer[300 * WIDTH + 450], BLACK, "Circle via dispatcher");

    // Oval
    let mut buffer = new_buffer();
    draw_shape(&mut buffer, ToolMode::Oval, 300, y, 400, y + 50, BLACK, 1);
    let cy = y + 25;
    assert_eq!(buffer[cy * WIDTH + 400], BLACK, "Oval via dispatcher");

    // Triangle (drag down = points down, apex at bottom)
    let mut buffer = new_buffer();
    draw_shape(&mut buffer, ToolMode::Triangle, 100, y, 200, y + 100, BLACK, 1);
    // Apex at bottom center when dragging down
    assert_eq!(buffer[(y + 100) * WIDTH + 150], BLACK, "Triangle via dispatcher");
}

#[test]
fn test_fill_rectangle() {
    let mut buffer = new_buffer();
    let x1 = 100;
    let y1 = CANVAS_TOP + 50;
    let x2 = 150;
    let y2 = CANVAS_TOP + 100;

    fill_rectangle(&mut buffer, x1, y1, x2, y2, RED);

    // Interior should be filled
    assert_eq!(buffer[(y1 + 25) * WIDTH + (x1 + 25)], RED, "Interior should be filled");

    // Corners should be filled
    assert_eq!(buffer[y1 * WIDTH + x1], RED, "Top-left corner");
    assert_eq!(buffer[y2 * WIDTH + x2], RED, "Bottom-right corner");

    // Just outside should be white
    assert_eq!(buffer[(y1 - 1) * WIDTH + x1], WHITE, "Above should be white");
    assert_eq!(buffer[y1 * WIDTH + (x2 + 1)], WHITE, "Right should be white");
}

#[test]
fn test_fill_circle() {
    let mut buffer = new_buffer();
    // Circle bounded by 300,200 to 400,300 (100x100 box, radius 50)
    fill_circle(&mut buffer, 300, 200, 400, 300, RED);

    let cx = 350;
    let cy = 250;

    // Center should be filled
    assert_eq!(buffer[cy * WIDTH + cx], RED, "Center should be filled");

    // Points just inside radius should be filled
    assert_eq!(buffer[cy * WIDTH + (cx + 40)], RED, "Near right edge");
    assert_eq!(buffer[(cy + 40) * WIDTH + cx], RED, "Near bottom edge");

    // Points outside radius should be white
    assert_eq!(buffer[cy * WIDTH + (cx + 60)], WHITE, "Outside right");
}

#[test]
fn test_fill_triangle() {
    let mut buffer = new_buffer();
    // Triangle pointing down: drag from top to bottom
    let x1 = 100;
    let y1 = CANVAS_TOP + 50;
    let x2 = 200;
    let y2 = CANVAS_TOP + 150;

    fill_triangle(&mut buffer, x1, y1, x2, y2, RED);

    // Center of triangle should be filled
    let mid_x = 150;
    let mid_y = CANVAS_TOP + 100;
    assert_eq!(buffer[mid_y * WIDTH + mid_x], RED, "Center should be filled");

    // Top corners (base) should be filled
    assert_eq!(buffer[y1 * WIDTH + x1], RED, "Top-left base");
    assert_eq!(buffer[y1 * WIDTH + x2], RED, "Top-right base");
}

#[test]
fn test_draw_shape_with_fill() {
    let mut buffer = new_buffer();

    // Draw rectangle with red fill and black edge
    draw_shape_with_fill(
        &mut buffer,
        ToolMode::Rectangle,
        100,
        CANVAS_TOP + 50,
        200,
        CANVAS_TOP + 150,
        Some(BLACK),
        Some(RED),
        1,
    );

    // Interior should be red (fill)
    let interior_x = 150;
    let interior_y = CANVAS_TOP + 100;
    assert_eq!(buffer[interior_y * WIDTH + interior_x], RED, "Interior should be fill color");

    // Edge should be black
    let edge_y = CANVAS_TOP + 50;
    assert_eq!(buffer[edge_y * WIDTH + 100], BLACK, "Edge should be edge color");
}

#[test]
fn test_draw_shape_without_fill() {
    let mut buffer = new_buffer();

    // Draw rectangle with no fill (None)
    draw_shape_with_fill(
        &mut buffer,
        ToolMode::Rectangle,
        100,
        CANVAS_TOP + 50,
        200,
        CANVAS_TOP + 150,
        Some(BLACK),
        None,
        1,
    );

    // Interior should be white (no fill)
    let interior_x = 150;
    let interior_y = CANVAS_TOP + 100;
    assert_eq!(buffer[interior_y * WIDTH + interior_x], WHITE, "Interior should be white (no fill)");

    // Edge should be black
    let edge_y = CANVAS_TOP + 50;
    assert_eq!(buffer[edge_y * WIDTH + 100], BLACK, "Edge should be edge color");
}

#[test]
fn test_shapes_respect_canvas_boundaries() {
    let mut buffer = new_buffer();

    // Draw rectangle that would extend into title bar
    draw_shape_rectangle(&mut buffer, 100, 10, 200, CANVAS_TOP + 50, BLACK, 1);

    // Title bar should remain white
    for y in 0..CANVAS_TOP {
        for x in 100..=200 {
            assert_eq!(buffer[y * WIDTH + x], WHITE, "Title bar at ({}, {}) should be white", x, y);
        }
    }

    // Draw rectangle that would extend into bottom toolbar
    let mut buffer = new_buffer();
    draw_shape_rectangle(&mut buffer, 100, CANVAS_BOTTOM - 50, 200, CANVAS_BOTTOM + 50, BLACK, 1);

    // Bottom toolbar should remain white
    for y in CANVAS_BOTTOM..HEIGHT {
        for x in 100..=200 {
            assert_eq!(buffer[y * WIDTH + x], WHITE, "Bottom toolbar at ({}, {}) should be white", x, y);
        }
    }
}
