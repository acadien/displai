use displai::*;

// Helper to create a fresh white buffer
fn new_buffer() -> Vec<u32> {
    vec![WHITE; WIDTH * HEIGHT]
}

// ===================
// Title Bar Rendering Tests
// ===================

#[test]
fn test_draw_button_fills_area() {
    let mut buffer = new_buffer();
    let bx = 50;
    let by = 50;

    draw_button(&mut buffer, bx, by, RED);

    // All pixels in button area should be RED
    for y in by..by + BUTTON_SIZE {
        for x in bx..bx + BUTTON_SIZE {
            assert_eq!(buffer[y * WIDTH + x], RED);
        }
    }

    // Pixel just outside should still be white
    assert_eq!(buffer[by * WIDTH + (bx + BUTTON_SIZE)], WHITE);
}

#[test]
fn test_draw_title_bar_covers_top() {
    let mut buffer = new_buffer();

    draw_title_bar(&mut buffer);

    // Title bar area should be gray (except close button)
    let mid_x = WIDTH / 2;
    let mid_y = TITLE_BAR_HEIGHT / 2;
    assert_eq!(buffer[mid_y * WIDTH + mid_x], GRAY);

    // Bottom border should be dark gray
    assert_eq!(buffer[(TITLE_BAR_HEIGHT - 1) * WIDTH + mid_x], DARK_GRAY);
}

#[test]
fn test_title_bar_only_has_close_button() {
    let mut buffer = new_buffer();

    draw_title_bar(&mut buffer);

    // Close button should be present (red with white X)
    let close_x = WIDTH - BUTTON_SIZE - BUTTON_MARGIN;
    let close_y = BUTTON_MARGIN;
    assert_eq!(buffer[(close_y + 1) * WIDTH + (close_x + 1)], RED);

    // Area where color buttons used to be should now be gray
    let color_btn_area_x = BUTTON_MARGIN + BUTTON_SIZE / 2;
    let color_btn_area_y = BUTTON_MARGIN + BUTTON_SIZE / 2;
    assert_eq!(buffer[color_btn_area_y * WIDTH + color_btn_area_x], GRAY);
}

#[test]
fn test_close_button_is_red() {
    let mut buffer = new_buffer();

    draw_title_bar(&mut buffer);

    // Close button center should be red (before X is drawn over it)
    // Actually the X is white, so check a corner pixel
    let close_x = WIDTH - BUTTON_SIZE - BUTTON_MARGIN;
    let close_y = BUTTON_MARGIN;

    // Check a pixel that's in the button but not part of the X
    // The X has padding of 6, so corners should still be red
    assert_eq!(buffer[(close_y + 1) * WIDTH + (close_x + 1)], RED);
}

// ===================
// Bottom Toolbar Rendering Tests
// ===================

#[test]
fn test_draw_bottom_toolbar_covers_bottom() {
    let mut buffer = new_buffer();

    draw_bottom_toolbar(&mut buffer, 0, 1);

    // Bottom toolbar area should be filled
    let mid_x = WIDTH / 2;

    // Check the top border of the toolbar
    assert_eq!(buffer[CANVAS_BOTTOM * WIDTH + mid_x], DARK_GRAY);
}

#[test]
fn test_all_14_palette_colors_rendered() {
    let mut buffer = new_buffer();

    draw_bottom_toolbar(&mut buffer, 0, 1);

    // Verify each of the 14 color buttons shows its corresponding color
    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;
    for i in 0..14 {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);

        // Check center of button (avoid border pixels)
        let center_x = bx + BUTTON_SIZE / 2;
        let center_y = row1_y + BUTTON_SIZE / 2;
        assert_eq!(
            buffer[center_y * WIDTH + center_x],
            COLOR_PALETTE[i],
            "Button {} does not show correct color",
            i
        );
    }
}

#[test]
fn test_selected_color_has_white_border() {
    let mut buffer = new_buffer();

    // Select color index 5
    draw_bottom_toolbar(&mut buffer, 5, 1);

    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;

    // Button 5 should have white border
    let btn5_x = BUTTON_MARGIN + 5 * (BUTTON_SIZE + BUTTON_MARGIN);
    let border_pixel_5 = buffer[row1_y * WIDTH + btn5_x];
    assert_eq!(border_pixel_5, WHITE);

    // Button 0 should have dark gray border (not selected)
    let border_pixel_0 = buffer[row1_y * WIDTH + BUTTON_MARGIN];
    assert_eq!(border_pixel_0, DARK_GRAY);
}

#[test]
fn test_white_color_selected_has_blue_border() {
    let mut buffer = new_buffer();

    // Select white color (index 1)
    draw_bottom_toolbar(&mut buffer, 1, 1);

    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;

    // Button 1 (white) should have blue border (to be visible on white)
    let btn1_x = BUTTON_MARGIN + 1 * (BUTTON_SIZE + BUTTON_MARGIN);
    let border_pixel_1 = buffer[row1_y * WIDTH + btn1_x];
    assert_eq!(border_pixel_1, 0x4040E0); // Blue border for white color
}

#[test]
fn test_plus_minus_buttons_rendered() {
    let mut buffer = new_buffer();

    draw_bottom_toolbar(&mut buffer, 0, 5);

    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let size_display_x = BUTTON_MARGIN;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    let plus_x = minus_x + BUTTON_SIZE + BUTTON_MARGIN;

    // Minus button should be dark gray background
    assert_eq!(buffer[(row2_y + 1) * WIDTH + (minus_x + 1)], DARK_GRAY);

    // Plus button should be dark gray background
    assert_eq!(buffer[(row2_y + 1) * WIDTH + (plus_x + 1)], DARK_GRAY);
}

#[test]
fn test_size_display_rendered() {
    let mut buffer = new_buffer();

    draw_bottom_toolbar(&mut buffer, 0, 10);

    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let size_display_x = BUTTON_MARGIN;

    // Size display should have white background (check near edge to avoid number)
    assert_eq!(buffer[(row2_y + 1) * WIDTH + (size_display_x + 1)], WHITE);

    // Border should be dark gray
    assert_eq!(buffer[row2_y * WIDTH + size_display_x], DARK_GRAY);
}

#[test]
fn test_canvas_area_dimensions() {
    // Verify canvas area is correctly defined
    assert_eq!(CANVAS_TOP, TITLE_BAR_HEIGHT);
    assert_eq!(CANVAS_TOP, 30);
    assert_eq!(CANVAS_BOTTOM, HEIGHT - BOTTOM_TOOLBAR_HEIGHT);
    assert_eq!(CANVAS_BOTTOM, 540);
    assert_eq!(BOTTOM_TOOLBAR_HEIGHT, 60);
    assert_eq!(TOOLBAR_ROW_HEIGHT, 30);
}

#[test]
fn test_color_palette_has_14_colors() {
    assert_eq!(COLOR_PALETTE.len(), 14);
    assert_eq!(COLOR_PALETTE[0], BLACK); // First color should be black
    assert_eq!(COLOR_PALETTE[1], WHITE); // Second color should be white (acts as eraser)
}

#[test]
fn test_brush_size_constants() {
    assert_eq!(MIN_BRUSH_SIZE, 1);
    assert_eq!(MAX_BRUSH_SIZE, 20);
    assert_eq!(DEFAULT_BRUSH_SIZE, 1);
}
