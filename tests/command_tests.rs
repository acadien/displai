use displai::*;

// Helper to create a fresh white buffer
fn new_buffer() -> Vec<u32> {
    vec![WHITE; WIDTH * HEIGHT]
}

// ===================
// Command Parsing Tests
// ===================

#[test]
fn test_parse_snapshot() {
    assert_eq!(parse_command("snapshot"), Some(Command::Snapshot));
    assert_eq!(parse_command("  snapshot  "), Some(Command::Snapshot));
}

#[test]
fn test_parse_clear() {
    assert_eq!(parse_command("clear"), Some(Command::Clear));
}

#[test]
fn test_parse_state() {
    assert_eq!(parse_command("state"), Some(Command::State));
}

#[test]
fn test_parse_color() {
    assert_eq!(parse_command("color 0"), Some(Command::Color(0)));
    assert_eq!(parse_command("color 5"), Some(Command::Color(5)));
    assert_eq!(parse_command("color 13"), Some(Command::Color(13)));

    // Invalid color indices (14 colors means indices 0-13)
    assert_eq!(parse_command("color 14"), None);
    assert_eq!(parse_command("color -1"), None);
    assert_eq!(parse_command("color abc"), None);
    assert_eq!(parse_command("color"), None);
}

#[test]
fn test_parse_eraser_no_longer_valid() {
    // Eraser command has been removed - should return None
    assert_eq!(parse_command("eraser on"), None);
    assert_eq!(parse_command("eraser off"), None);
}

#[test]
fn test_parse_size() {
    assert_eq!(parse_command("size 1"), Some(Command::Size(1)));
    assert_eq!(parse_command("size 10"), Some(Command::Size(10)));
    assert_eq!(parse_command("size 20"), Some(Command::Size(20)));

    // Invalid sizes (out of range)
    assert_eq!(parse_command("size 0"), None);
    assert_eq!(parse_command("size 21"), None);
    assert_eq!(parse_command("size abc"), None);
    assert_eq!(parse_command("size"), None);
}

#[test]
fn test_parse_stroke() {
    assert_eq!(
        parse_command("stroke 100,200 300,400"),
        Some(Command::Stroke {
            x1: 100,
            y1: 200,
            x2: 300,
            y2: 400
        })
    );

    // Invalid stroke formats
    assert_eq!(parse_command("stroke 100,200"), None);
    assert_eq!(parse_command("stroke"), None);
    assert_eq!(parse_command("stroke abc,def 100,200"), None);
}

#[test]
fn test_parse_dot() {
    assert_eq!(
        parse_command("dot 150,250"),
        Some(Command::Dot { x: 150, y: 250 })
    );

    // Invalid dot formats
    assert_eq!(parse_command("dot"), None);
    assert_eq!(parse_command("dot 150"), None);
    assert_eq!(parse_command("dot abc,def"), None);
}

#[test]
fn test_parse_unknown_command() {
    assert_eq!(parse_command("unknown"), None);
    assert_eq!(parse_command(""), None);
    assert_eq!(parse_command("   "), None);
}

// ===================
// Command Execution Tests
// ===================

#[test]
fn test_execute_color_command() {
    let mut buffer = new_buffer();
    let mut color_index = 0;
    let mut size = 5;

    let result = execute_command(
        &Command::Color(5),
        &mut buffer,
        &mut color_index,
        &mut size,
    );

    assert_eq!(color_index, 5);
    assert!(result.is_none());
}

#[test]
fn test_execute_size_command() {
    let mut buffer = new_buffer();
    let mut color_index = 0;
    let mut size = 5;

    execute_command(
        &Command::Size(15),
        &mut buffer,
        &mut color_index,
        &mut size,
    );
    assert_eq!(size, 15);
}

#[test]
fn test_execute_dot_command() {
    let mut buffer = new_buffer();
    let mut color_index = 0; // Black
    let mut size = 1;

    // Draw a dot in the canvas area
    let x = 100;
    let y = CANVAS_TOP + 50;
    execute_command(
        &Command::Dot { x, y },
        &mut buffer,
        &mut color_index,
        &mut size,
    );

    // Verify pixel is now black (color index 0)
    assert_eq!(buffer[y * WIDTH + x], COLOR_PALETTE[0]);
}

#[test]
fn test_execute_dot_with_white_erases() {
    let mut buffer = new_buffer();
    let mut color_index = 0; // Black
    let mut size = 1;

    // First draw a black dot
    let x = 100;
    let y = CANVAS_TOP + 50;
    execute_command(
        &Command::Dot { x, y },
        &mut buffer,
        &mut color_index,
        &mut size,
    );
    assert_eq!(buffer[y * WIDTH + x], COLOR_PALETTE[0]); // Black

    // Now use white (index 1) to erase it
    color_index = 1; // White
    execute_command(
        &Command::Dot { x, y },
        &mut buffer,
        &mut color_index,
        &mut size,
    );
    assert_eq!(buffer[y * WIDTH + x], WHITE);
}

#[test]
fn test_execute_stroke_command() {
    let mut buffer = new_buffer();
    let mut color_index = 2; // Red (index 2 after Black, White)
    let mut size = 1;

    let y = CANVAS_TOP + 100;
    execute_command(
        &Command::Stroke {
            x1: 50,
            y1: y,
            x2: 60,
            y2: y,
        },
        &mut buffer,
        &mut color_index,
        &mut size,
    );

    // Verify pixels along the stroke are red
    for x in 50..=60 {
        assert_eq!(
            buffer[y * WIDTH + x],
            COLOR_PALETTE[2],
            "Pixel at x={} should be red",
            x
        );
    }
}

#[test]
fn test_execute_clear_command() {
    let mut buffer = new_buffer();
    let mut color_index = 0;
    let mut size = 5;

    // Draw something first
    let y = CANVAS_TOP + 100;
    execute_command(
        &Command::Dot { x: 100, y },
        &mut buffer,
        &mut color_index,
        &mut size,
    );
    assert_ne!(buffer[y * WIDTH + 100], WHITE);

    // Clear
    execute_command(
        &Command::Clear,
        &mut buffer,
        &mut color_index,
        &mut size,
    );

    // Verify canvas is cleared
    for y in CANVAS_TOP..CANVAS_BOTTOM {
        for x in 0..WIDTH {
            assert_eq!(
                buffer[y * WIDTH + x],
                WHITE,
                "Pixel at ({},{}) should be white",
                x,
                y
            );
        }
    }
}

#[test]
fn test_execute_state_command() {
    let mut buffer = new_buffer();
    let mut color_index = 5;
    let mut size = 10;

    let result = execute_command(
        &Command::State,
        &mut buffer,
        &mut color_index,
        &mut size,
    );

    assert_eq!(result, Some("color:5 size:10".to_string()));
}

// ===================
// Clear Canvas Tests
// ===================

#[test]
fn test_clear_canvas_only_affects_canvas_area() {
    let mut buffer = new_buffer();

    // Set some pixels in title bar and bottom toolbar
    buffer[10 * WIDTH + 50] = BLACK; // Title bar
    buffer[(CANVAS_BOTTOM + 10) * WIDTH + 50] = BLACK; // Bottom toolbar

    // Clear canvas
    clear_canvas(&mut buffer);

    // Canvas area should be white
    for y in CANVAS_TOP..CANVAS_BOTTOM {
        for x in 0..WIDTH {
            assert_eq!(buffer[y * WIDTH + x], WHITE);
        }
    }

    // Title bar pixel should still be black (not cleared)
    assert_eq!(buffer[10 * WIDTH + 50], BLACK);

    // Bottom toolbar pixel should still be black (not cleared)
    assert_eq!(buffer[(CANVAS_BOTTOM + 10) * WIDTH + 50], BLACK);
}

// ===================
// PNG Export Tests
// ===================

#[test]
fn test_save_canvas_png_creates_file() {
    let mut buffer = new_buffer();

    // Draw something on canvas
    let y = CANVAS_TOP + 50;
    for x in 100..150 {
        buffer[y * WIDTH + x] = RED;
    }

    let path = "/tmp/test_canvas.png";
    let result = save_canvas_png(&buffer, path);

    assert!(result.is_ok(), "save_canvas_png should succeed");
    assert!(std::path::Path::new(path).exists(), "PNG file should exist");

    // Clean up
    std::fs::remove_file(path).ok();
}

#[test]
fn test_save_canvas_png_correct_dimensions() {
    let buffer = new_buffer();
    let path = "/tmp/test_canvas_dimensions.png";

    save_canvas_png(&buffer, path).expect("Should save");

    // Read the image and check dimensions
    let img = image::open(path).expect("Should open");
    assert_eq!(img.width() as usize, WIDTH);
    assert_eq!(img.height() as usize, CANVAS_BOTTOM - CANVAS_TOP);

    // Clean up
    std::fs::remove_file(path).ok();
}

#[test]
fn test_save_canvas_png_pixel_colors() {
    let mut buffer = new_buffer();

    // Draw a red pixel at a known location in canvas coordinates
    let canvas_y = 50; // Y in canvas coordinates (not buffer coordinates)
    let buffer_y = CANVAS_TOP + canvas_y;
    let x = 100;
    buffer[buffer_y * WIDTH + x] = RED;

    let path = "/tmp/test_canvas_colors.png";
    save_canvas_png(&buffer, path).expect("Should save");

    // Read and verify the pixel
    let img = image::open(path).expect("Should open").into_rgb8();
    let pixel = img.get_pixel(x as u32, canvas_y as u32);

    // RED is 0xE04040 -> R=0xE0(224), G=0x40(64), B=0x40(64)
    assert_eq!(pixel.0[0], 0xE0, "Red channel should match");
    assert_eq!(pixel.0[1], 0x40, "Green channel should match");
    assert_eq!(pixel.0[2], 0x40, "Blue channel should match");

    // Clean up
    std::fs::remove_file(path).ok();
}
