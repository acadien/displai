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
