use displai::*;

// ===================
// Button Detection Tests
// ===================

#[test]
fn test_close_button_detection() {
    let close_x = WIDTH - BUTTON_SIZE - BUTTON_MARGIN;
    let close_y = BUTTON_MARGIN;

    // Center of close button should be detected
    assert!(is_in_close_button(
        close_x + BUTTON_SIZE / 2,
        close_y + BUTTON_SIZE / 2
    ));

    // Corners of close button
    assert!(is_in_close_button(close_x, close_y));
    assert!(is_in_close_button(
        close_x + BUTTON_SIZE - 1,
        close_y + BUTTON_SIZE - 1
    ));

    // Just outside close button
    assert!(!is_in_close_button(close_x - 1, close_y));
    assert!(!is_in_close_button(close_x, close_y + BUTTON_SIZE));
}

#[test]
fn test_close_button_not_triggered_elsewhere() {
    // Canvas area
    assert!(!is_in_close_button(100, 100));

    // Bottom toolbar area
    assert!(!is_in_close_button(BUTTON_MARGIN + 5, CANVAS_BOTTOM + 10));

    // Random title bar location
    assert!(!is_in_close_button(WIDTH / 2, 15));
}

#[test]
fn test_color_palette_button_detection_bottom() {
    // Test each of the 14 color buttons in bottom toolbar
    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;
    for i in 0..14 {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);

        // Center of button should return the correct index
        let center_x = bx + BUTTON_SIZE / 2;
        let center_y = row1_y + BUTTON_SIZE / 2;
        assert_eq!(
            get_clicked_color_index_bottom(center_x, center_y),
            Some(i),
            "Center of button {} not detected",
            i
        );

        // Corners of button
        assert_eq!(get_clicked_color_index_bottom(bx, row1_y), Some(i));
        assert_eq!(
            get_clicked_color_index_bottom(bx + BUTTON_SIZE - 1, row1_y + BUTTON_SIZE - 1),
            Some(i)
        );
    }

    // Gaps between buttons should return None
    for i in 0..13 {
        let gap_x = BUTTON_MARGIN + (i + 1) * (BUTTON_SIZE + BUTTON_MARGIN) - 1;
        let by = row1_y + BUTTON_SIZE / 2;
        // This position is in the margin between buttons
        assert_eq!(
            get_clicked_color_index_bottom(gap_x, by),
            None,
            "Gap after button {} incorrectly detected as a button",
            i
        );
    }
}

#[test]
fn test_color_buttons_outside_bounds_bottom() {
    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;

    // Above buttons (y too small - in canvas area)
    assert_eq!(
        get_clicked_color_index_bottom(BUTTON_MARGIN + 5, row1_y - 1),
        None
    );

    // Below buttons (y too large - in row 2)
    assert_eq!(
        get_clicked_color_index_bottom(BUTTON_MARGIN + 5, row1_y + BUTTON_SIZE),
        None
    );

    // Before first button (x too small)
    assert_eq!(
        get_clicked_color_index_bottom(BUTTON_MARGIN - 1, row1_y + 5),
        None
    );

    // After last button (past color buttons)
    let after_last = BUTTON_MARGIN + 14 * (BUTTON_SIZE + BUTTON_MARGIN);
    assert_eq!(get_clicked_color_index_bottom(after_last, row1_y + 5), None);
}

#[test]
fn test_color_buttons_do_not_overlap_close() {
    // Close button is in title bar, color buttons are in bottom toolbar
    // They shouldn't overlap by definition, but let's verify
    let close_x = WIDTH - BUTTON_SIZE - BUTTON_MARGIN;
    let close_y = BUTTON_MARGIN;

    for y in close_y..close_y + BUTTON_SIZE {
        for x in close_x..close_x + BUTTON_SIZE {
            assert!(
                get_clicked_color_index_bottom(x, y).is_none(),
                "Close button pixel ({}, {}) overlaps with color buttons",
                x,
                y
            );
        }
    }
}

#[test]
fn test_color_buttons_do_not_overlap_each_other() {
    // For each button, verify no pixel is claimed by another button
    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;
    for i in 0..14 {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);

        for y in row1_y..row1_y + BUTTON_SIZE {
            for x in bx..bx + BUTTON_SIZE {
                let result = get_clicked_color_index_bottom(x, y);
                assert_eq!(
                    result,
                    Some(i),
                    "Button {} pixel ({}, {}) detected as {:?}",
                    i,
                    x,
                    y,
                    result
                );
            }
        }
    }
}

#[test]
fn test_minus_button_detection() {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    // Size display is now after 7 tool buttons
    let size_display_x = BUTTON_MARGIN + 7 * (BUTTON_SIZE + BUTTON_MARGIN) + BUTTON_MARGIN;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;

    // Center of minus button
    assert!(is_in_minus_button(
        minus_x + BUTTON_SIZE / 2,
        row2_y + BUTTON_SIZE / 2
    ));

    // Corners
    assert!(is_in_minus_button(minus_x, row2_y));
    assert!(is_in_minus_button(
        minus_x + BUTTON_SIZE - 1,
        row2_y + BUTTON_SIZE - 1
    ));

    // Just outside
    assert!(!is_in_minus_button(minus_x - 1, row2_y));
    assert!(!is_in_minus_button(minus_x, row2_y - 1));
}

#[test]
fn test_plus_button_detection() {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    // Size display is now after 7 tool buttons
    let size_display_x = BUTTON_MARGIN + 7 * (BUTTON_SIZE + BUTTON_MARGIN) + BUTTON_MARGIN;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    let plus_x = minus_x + BUTTON_SIZE + BUTTON_MARGIN;

    // Center of plus button
    assert!(is_in_plus_button(
        plus_x + BUTTON_SIZE / 2,
        row2_y + BUTTON_SIZE / 2
    ));

    // Corners
    assert!(is_in_plus_button(plus_x, row2_y));
    assert!(is_in_plus_button(
        plus_x + BUTTON_SIZE - 1,
        row2_y + BUTTON_SIZE - 1
    ));

    // Just outside
    assert!(!is_in_plus_button(plus_x - 1, row2_y));
    assert!(!is_in_plus_button(plus_x, row2_y - 1));
}

#[test]
fn test_tool_button_detection() {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;

    // Test each tool button
    let expected_tools = [
        ToolMode::Brush,
        ToolMode::Line,
        ToolMode::Square,
        ToolMode::Rectangle,
        ToolMode::Circle,
        ToolMode::Oval,
        ToolMode::Triangle,
    ];

    for (i, &expected_tool) in expected_tools.iter().enumerate() {
        let bx = BUTTON_MARGIN + i * (BUTTON_SIZE + BUTTON_MARGIN);
        let center_x = bx + BUTTON_SIZE / 2;
        let center_y = row2_y + BUTTON_SIZE / 2;

        assert_eq!(
            get_clicked_tool(center_x, center_y),
            Some(expected_tool),
            "Tool button {} not detected correctly",
            i
        );
    }

    // Test outside tool buttons area
    assert_eq!(get_clicked_tool(0, 0), None); // Top-left corner
    assert_eq!(get_clicked_tool(WIDTH / 2, CANVAS_TOP + 50), None); // Canvas area
}

#[test]
fn test_row2_buttons_do_not_overlap() {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    // Size display is now after 7 tool buttons
    let size_display_x = BUTTON_MARGIN + 7 * (BUTTON_SIZE + BUTTON_MARGIN) + BUTTON_MARGIN;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    let plus_x = minus_x + BUTTON_SIZE + BUTTON_MARGIN;

    // Check minus button area - should only trigger minus
    for y in row2_y..row2_y + BUTTON_SIZE {
        for x in minus_x..minus_x + BUTTON_SIZE {
            assert!(
                is_in_minus_button(x, y),
                "Minus button miss at ({}, {})",
                x,
                y
            );
            assert!(
                !is_in_plus_button(x, y),
                "Minus overlaps plus at ({}, {})",
                x,
                y
            );
        }
    }

    // Check plus button area - should only trigger plus
    for y in row2_y..row2_y + BUTTON_SIZE {
        for x in plus_x..plus_x + BUTTON_SIZE {
            assert!(
                !is_in_minus_button(x, y),
                "Plus overlaps minus at ({}, {})",
                x,
                y
            );
            assert!(
                is_in_plus_button(x, y),
                "Plus button miss at ({}, {})",
                x,
                y
            );
        }
    }
}

#[test]
fn test_clear_button_detection() {
    let row2_y = CANVAS_BOTTOM + TOOLBAR_ROW_HEIGHT + BUTTON_MARGIN;
    let size_display_x = BUTTON_MARGIN + 7 * (BUTTON_SIZE + BUTTON_MARGIN) + BUTTON_MARGIN;
    let minus_x = size_display_x + 44 + BUTTON_MARGIN;
    let plus_x = minus_x + BUTTON_SIZE + BUTTON_MARGIN;
    let clear_x = plus_x + BUTTON_SIZE + BUTTON_MARGIN * 2;

    // Center of clear button
    assert!(is_in_clear_button(
        clear_x + BUTTON_SIZE / 2,
        row2_y + BUTTON_SIZE / 2
    ));

    // Corners
    assert!(is_in_clear_button(clear_x, row2_y));
    assert!(is_in_clear_button(
        clear_x + BUTTON_SIZE - 1,
        row2_y + BUTTON_SIZE - 1
    ));

    // Just outside
    assert!(!is_in_clear_button(clear_x - 1, row2_y));
    assert!(!is_in_clear_button(clear_x, row2_y - 1));
}

#[test]
fn test_transparent_button_detection() {
    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;
    let transparent_x = BUTTON_MARGIN + 14 * (BUTTON_SIZE + BUTTON_MARGIN);

    // Center of transparent button
    assert!(is_in_transparent_button(
        transparent_x + BUTTON_SIZE / 2,
        row1_y + BUTTON_SIZE / 2
    ));

    // Corners
    assert!(is_in_transparent_button(transparent_x, row1_y));
    assert!(is_in_transparent_button(
        transparent_x + BUTTON_SIZE - 1,
        row1_y + BUTTON_SIZE - 1
    ));

    // Just outside
    assert!(!is_in_transparent_button(transparent_x - 1, row1_y));
    assert!(!is_in_transparent_button(transparent_x, row1_y - 1));
    assert!(!is_in_transparent_button(transparent_x, row1_y + BUTTON_SIZE));
}

#[test]
fn test_transparent_button_does_not_overlap_colors() {
    let row1_y = CANVAS_BOTTOM + BUTTON_MARGIN;

    // Transparent button is after all 14 color buttons
    let transparent_x = BUTTON_MARGIN + 14 * (BUTTON_SIZE + BUTTON_MARGIN);

    // Verify no color button is detected in transparent button area
    for y in row1_y..row1_y + BUTTON_SIZE {
        for x in transparent_x..transparent_x + BUTTON_SIZE {
            assert!(
                get_clicked_color_index_bottom(x, y).is_none(),
                "Transparent button pixel ({}, {}) overlaps with color buttons",
                x,
                y
            );
        }
    }
}
