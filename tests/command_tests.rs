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

#[test]
fn test_parse_edge() {
    assert_eq!(parse_command("edge 0"), Some(Command::Edge(Some(0))));
    assert_eq!(parse_command("edge 5"), Some(Command::Edge(Some(5))));
    assert_eq!(parse_command("edge 13"), Some(Command::Edge(Some(13))));
    assert_eq!(parse_command("edge none"), Some(Command::Edge(None)));

    // Invalid
    assert_eq!(parse_command("edge 14"), None);
    assert_eq!(parse_command("edge"), None);
    assert_eq!(parse_command("edge abc"), None);
}

#[test]
fn test_parse_fill() {
    assert_eq!(parse_command("fill 0"), Some(Command::Fill(Some(0))));
    assert_eq!(parse_command("fill 5"), Some(Command::Fill(Some(5))));
    assert_eq!(parse_command("fill 13"), Some(Command::Fill(Some(13))));
    assert_eq!(parse_command("fill none"), Some(Command::Fill(None)));

    // Invalid
    assert_eq!(parse_command("fill 14"), None);
    assert_eq!(parse_command("fill"), None);
    assert_eq!(parse_command("fill abc"), None);
}

// ===================
// Shape Command Parsing Tests
// ===================

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_command("line 100,200 300,400"),
        Some(Command::Line {
            x1: 100,
            y1: 200,
            x2: 300,
            y2: 400
        })
    );

    // Invalid formats
    assert_eq!(parse_command("line 100,200"), None);
    assert_eq!(parse_command("line"), None);
    assert_eq!(parse_command("line abc,def 100,200"), None);
}

#[test]
fn test_parse_square() {
    assert_eq!(
        parse_command("square 100,200 50"),
        Some(Command::Square {
            x: 100,
            y: 200,
            size: 50
        })
    );

    // Invalid formats
    assert_eq!(parse_command("square 100,200"), None);
    assert_eq!(parse_command("square"), None);
    assert_eq!(parse_command("square abc,def 50"), None);
}

#[test]
fn test_parse_rect() {
    assert_eq!(
        parse_command("rect 100,200 300,400"),
        Some(Command::Rect {
            x1: 100,
            y1: 200,
            x2: 300,
            y2: 400
        })
    );

    // Invalid formats
    assert_eq!(parse_command("rect 100,200"), None);
    assert_eq!(parse_command("rect"), None);
}

#[test]
fn test_parse_circle() {
    assert_eq!(
        parse_command("circle 200,300 50"),
        Some(Command::Circle {
            x: 200,
            y: 300,
            r: 50
        })
    );

    // Invalid formats
    assert_eq!(parse_command("circle 200,300"), None);
    assert_eq!(parse_command("circle"), None);
}

#[test]
fn test_parse_oval() {
    assert_eq!(
        parse_command("oval 200,300 50,30"),
        Some(Command::Oval {
            x: 200,
            y: 300,
            rx: 50,
            ry: 30
        })
    );

    // Invalid formats
    assert_eq!(parse_command("oval 200,300"), None);
    assert_eq!(parse_command("oval 200,300 50"), None);
    assert_eq!(parse_command("oval"), None);
}

#[test]
fn test_parse_triangle() {
    assert_eq!(
        parse_command("triangle 100,200 300,400"),
        Some(Command::Triangle {
            x1: 100,
            y1: 200,
            x2: 300,
            y2: 400
        })
    );

    // Invalid formats
    assert_eq!(parse_command("triangle 100,200"), None);
    assert_eq!(parse_command("triangle"), None);
}

// ===================
// Command Execution Tests
// ===================

#[test]
fn test_execute_color_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0);
    let mut fill_color_index: Option<usize> = None;
    let mut size = 5;

    let result = execute_command(
        &Command::Color(5),
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    assert_eq!(edge_color_index, Some(5));
    assert!(result.is_none());
}

#[test]
fn test_execute_edge_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0);
    let mut fill_color_index: Option<usize> = None;
    let mut size = 5;

    // Set edge to color 7
    execute_command(
        &Command::Edge(Some(7)),
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );
    assert_eq!(edge_color_index, Some(7));

    // Set edge to transparent
    execute_command(
        &Command::Edge(None),
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );
    assert_eq!(edge_color_index, None);
}

#[test]
fn test_execute_fill_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0);
    let mut fill_color_index: Option<usize> = None;
    let mut size = 5;

    // Set fill to color 3
    execute_command(
        &Command::Fill(Some(3)),
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );
    assert_eq!(fill_color_index, Some(3));

    // Set fill to transparent
    execute_command(
        &Command::Fill(None),
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );
    assert_eq!(fill_color_index, None);
}

#[test]
fn test_execute_size_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0);
    let mut fill_color_index: Option<usize> = None;
    let mut size = 5;

    execute_command(
        &Command::Size(15),
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );
    assert_eq!(size, 15);
}

#[test]
fn test_execute_dot_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    // Draw a dot in the canvas area
    let x = 100;
    let y = CANVAS_TOP + 50;
    execute_command(
        &Command::Dot { x, y },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Verify pixel is now black (color index 0)
    assert_eq!(buffer[y * WIDTH + x], COLOR_PALETTE[0]);
}

#[test]
fn test_execute_dot_with_white_erases() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    // First draw a black dot
    let x = 100;
    let y = CANVAS_TOP + 50;
    execute_command(
        &Command::Dot { x, y },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );
    assert_eq!(buffer[y * WIDTH + x], COLOR_PALETTE[0]); // Black

    // Now use white (index 1) to erase it
    edge_color_index = Some(1); // White
    execute_command(
        &Command::Dot { x, y },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );
    assert_eq!(buffer[y * WIDTH + x], WHITE);
}

#[test]
fn test_execute_stroke_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(2); // Red (index 2 after Black, White)
    let mut fill_color_index: Option<usize> = None;
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
        &mut edge_color_index,
        &mut fill_color_index,
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
    let mut edge_color_index: Option<usize> = Some(0);
    let mut fill_color_index: Option<usize> = None;
    let mut size = 5;

    // Draw something first
    let y = CANVAS_TOP + 100;
    execute_command(
        &Command::Dot { x: 100, y },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );
    assert_ne!(buffer[y * WIDTH + 100], WHITE);

    // Clear
    execute_command(
        &Command::Clear,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
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
    let mut edge_color_index: Option<usize> = Some(5);
    let mut fill_color_index: Option<usize> = Some(3);
    let mut size = 10;

    let result = execute_command(
        &Command::State,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    assert_eq!(result, Some("edge:5 fill:3 size:10".to_string()));
}

#[test]
fn test_execute_state_command_no_fill() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(5);
    let mut fill_color_index: Option<usize> = None;
    let mut size = 10;

    let result = execute_command(
        &Command::State,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    assert_eq!(result, Some("edge:5 fill:none size:10".to_string()));
}

#[test]
fn test_execute_state_command_transparent_edge() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = None; // Transparent edge
    let mut fill_color_index: Option<usize> = Some(3);
    let mut size = 10;

    let result = execute_command(
        &Command::State,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    assert_eq!(result, Some("edge:none fill:3 size:10".to_string()));
}

#[test]
fn test_execute_dot_with_transparent_edge_does_nothing() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = None; // Transparent edge
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let x = 100;
    let y = CANVAS_TOP + 50;

    // Store original pixel value
    let original = buffer[y * WIDTH + x];

    execute_command(
        &Command::Dot { x, y },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Pixel should be unchanged when edge is transparent
    assert_eq!(buffer[y * WIDTH + x], original);
}

// ===================
// Shape Command Execution Tests
// ===================

#[test]
fn test_execute_line_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(2); // Red
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let y = CANVAS_TOP + 100;
    execute_command(
        &Command::Line {
            x1: 100,
            y1: y,
            x2: 150,
            y2: y,
        },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Verify pixels along the line are red
    for x in 100..=150 {
        assert_eq!(
            buffer[y * WIDTH + x],
            COLOR_PALETTE[2],
            "Pixel at x={} should be red",
            x
        );
    }
}

#[test]
fn test_execute_rect_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let x1 = 100;
    let y1 = CANVAS_TOP + 50;
    let x2 = 200;
    let y2 = CANVAS_TOP + 150;

    execute_command(
        &Command::Rect { x1, y1, x2, y2 },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Verify top edge
    assert_eq!(buffer[y1 * WIDTH + 150], BLACK, "Top edge should be black");
    // Verify left edge
    assert_eq!(
        buffer[(y1 + 50) * WIDTH + x1],
        BLACK,
        "Left edge should be black"
    );
    // Interior should still be white (no fill)
    assert_eq!(
        buffer[(y1 + 50) * WIDTH + 150],
        WHITE,
        "Interior should be white"
    );
}

#[test]
fn test_execute_rect_with_fill() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = Some(2); // Red fill
    let mut size = 1;

    let x1 = 100;
    let y1 = CANVAS_TOP + 50;
    let x2 = 200;
    let y2 = CANVAS_TOP + 150;

    execute_command(
        &Command::Rect { x1, y1, x2, y2 },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Interior should be red (fill)
    assert_eq!(
        buffer[(y1 + 50) * WIDTH + 150],
        COLOR_PALETTE[2],
        "Interior should be red fill"
    );
}

#[test]
fn test_execute_square_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let x = 100;
    let y = CANVAS_TOP + 50;
    let sq_size = 50;

    execute_command(
        &Command::Square {
            x,
            y,
            size: sq_size,
        },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Verify corners are drawn
    assert_eq!(buffer[y * WIDTH + x], BLACK, "Top-left corner");
    assert_eq!(buffer[y * WIDTH + (x + sq_size)], BLACK, "Top-right corner");
    assert_eq!(
        buffer[(y + sq_size) * WIDTH + x],
        BLACK,
        "Bottom-left corner"
    );
}

#[test]
fn test_execute_circle_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let cx = 200;
    let cy = CANVAS_TOP + 100;
    let r = 30;

    execute_command(
        &Command::Circle { x: cx, y: cy, r },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Center should still be white (no fill)
    assert_eq!(buffer[cy * WIDTH + cx], WHITE, "Center should be white");
    // Top of circle should be black
    assert_eq!(
        buffer[(cy - r) * WIDTH + cx],
        BLACK,
        "Top of circle should be black"
    );
}

#[test]
fn test_execute_oval_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let cx = 200;
    let cy = CANVAS_TOP + 100;
    let rx = 50;
    let ry = 30;

    execute_command(
        &Command::Oval {
            x: cx,
            y: cy,
            rx,
            ry,
        },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Center should still be white (no fill)
    assert_eq!(buffer[cy * WIDTH + cx], WHITE, "Center should be white");
}

#[test]
fn test_execute_triangle_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let x1 = 100;
    let y1 = CANVAS_TOP + 50;
    let x2 = 200;
    let y2 = CANVAS_TOP + 150;

    execute_command(
        &Command::Triangle { x1, y1, x2, y2 },
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Triangle with y1 < y2 points DOWN (apex at bottom, base at top)
    // Base corners at top: (x1, y1) and (x2, y1)
    assert_eq!(buffer[y1 * WIDTH + x1], BLACK, "Top-left base corner");
    assert_eq!(buffer[y1 * WIDTH + x2], BLACK, "Top-right base corner");

    // Apex at bottom center: ((x1+x2)/2, y2)
    let apex_x = (x1 + x2) / 2;
    assert_eq!(buffer[y2 * WIDTH + apex_x], BLACK, "Apex should be black");
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
// Polyline and Points Command Tests
// ===================

#[test]
fn test_parse_polyline() {
    let cmd = parse_command("polyline 10,20 30,40 50,60");
    match cmd {
        Some(Command::Polyline(points)) => {
            assert_eq!(points.len(), 3);
            assert_eq!(points[0].x, 10);
            assert_eq!(points[0].y, 20);
            assert_eq!(points[0].color, None);
            assert_eq!(points[0].size, None);
            assert_eq!(points[1].x, 30);
            assert_eq!(points[1].y, 40);
            assert_eq!(points[2].x, 50);
            assert_eq!(points[2].y, 60);
        }
        _ => panic!("Expected Polyline command"),
    }
}

#[test]
fn test_parse_polyline_minimum_points() {
    // Polyline requires at least 2 points
    let cmd = parse_command("polyline 10,20 30,40");
    assert!(matches!(cmd, Some(Command::Polyline(points)) if points.len() == 2));

    // Single point should fail
    assert_eq!(parse_command("polyline 10,20"), None);

    // No points should fail
    assert_eq!(parse_command("polyline"), None);
}

#[test]
fn test_parse_polyline_invalid() {
    assert_eq!(parse_command("polyline abc,def 30,40"), None);
    assert_eq!(parse_command("polyline 10,20 abc,def"), None);
}

#[test]
fn test_parse_points() {
    let cmd = parse_command("points 10,20 30,40");
    match cmd {
        Some(Command::Points(points)) => {
            assert_eq!(points.len(), 2);
            assert_eq!(points[0].x, 10);
            assert_eq!(points[0].y, 20);
            assert_eq!(points[0].color, None);
            assert_eq!(points[0].size, None);
            assert_eq!(points[1].x, 30);
            assert_eq!(points[1].y, 40);
        }
        _ => panic!("Expected Points command"),
    }
}

#[test]
fn test_parse_points_single() {
    // Points should work with a single point
    let cmd = parse_command("points 100,200");
    assert!(matches!(cmd, Some(Command::Points(points)) if points.len() == 1));
}

#[test]
fn test_parse_points_invalid() {
    assert_eq!(parse_command("points"), None);
    assert_eq!(parse_command("points abc,def"), None);
}

#[test]
fn test_execute_polyline_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(2); // Red
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let y = CANVAS_TOP + 100;
    // Draw a horizontal polyline
    let cmd = Command::Polyline(vec![
        AttributedPoint { x: 100, y, color: None, size: None },
        AttributedPoint { x: 150, y, color: None, size: None },
        AttributedPoint { x: 200, y, color: None, size: None },
    ]);
    execute_command(
        &cmd,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Verify pixels along the path are red
    for x in 100..=200 {
        assert_eq!(
            buffer[y * WIDTH + x],
            COLOR_PALETTE[2],
            "Pixel at x={} should be red",
            x
        );
    }
}

#[test]
fn test_execute_polyline_with_transparent_edge() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = None; // Transparent
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let y = CANVAS_TOP + 100;
    let original = buffer[y * WIDTH + 150];

    let cmd = Command::Polyline(vec![
        AttributedPoint { x: 100, y, color: None, size: None },
        AttributedPoint { x: 200, y, color: None, size: None },
    ]);
    execute_command(
        &cmd,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Buffer should be unchanged with transparent edge
    assert_eq!(buffer[y * WIDTH + 150], original);
}

#[test]
fn test_execute_points_command() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let y = CANVAS_TOP + 100;
    let cmd = Command::Points(vec![
        AttributedPoint { x: 100, y, color: None, size: None },
        AttributedPoint { x: 150, y, color: None, size: None },
        AttributedPoint { x: 200, y, color: None, size: None },
    ]);
    execute_command(
        &cmd,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Verify each point is drawn
    assert_eq!(buffer[y * WIDTH + 100], BLACK, "Point at 100");
    assert_eq!(buffer[y * WIDTH + 150], BLACK, "Point at 150");
    assert_eq!(buffer[y * WIDTH + 200], BLACK, "Point at 200");

    // Points between should still be white (not connected)
    assert_eq!(buffer[y * WIDTH + 125], WHITE, "Gap at 125");
}

#[test]
fn test_execute_points_with_transparent_edge() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = None; // Transparent
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let y = CANVAS_TOP + 100;
    let original = buffer[y * WIDTH + 100];

    let cmd = Command::Points(vec![
        AttributedPoint { x: 100, y, color: None, size: None },
        AttributedPoint { x: 150, y, color: None, size: None },
    ]);
    execute_command(
        &cmd,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Buffer should be unchanged with transparent edge
    assert_eq!(buffer[y * WIDTH + 100], original);
}

#[test]
fn test_execute_points_with_brush_size() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = None;
    let mut size = 5; // Larger brush

    let x = 200;
    let y = CANVAS_TOP + 100;
    let cmd = Command::Points(vec![
        AttributedPoint { x, y, color: None, size: None },
    ]);
    execute_command(
        &cmd,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Center should be black
    assert_eq!(buffer[y * WIDTH + x], BLACK);
    // Nearby pixels should also be black due to brush size
    assert_eq!(buffer[y * WIDTH + (x + 2)], BLACK);
    assert_eq!(buffer[(y + 2) * WIDTH + x], BLACK);
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

// ===================
// Attributed Point Parsing Tests
// ===================

#[test]
fn test_parse_attributed_point_xy_only() {
    let pt = parse_attributed_point("100,200");
    assert!(pt.is_some());
    let pt = pt.unwrap();
    assert_eq!(pt.x, 100);
    assert_eq!(pt.y, 200);
    assert_eq!(pt.color, None);
    assert_eq!(pt.size, None);
}

#[test]
fn test_parse_attributed_point_with_color() {
    let pt = parse_attributed_point("100,200:5");
    assert!(pt.is_some());
    let pt = pt.unwrap();
    assert_eq!(pt.x, 100);
    assert_eq!(pt.y, 200);
    assert_eq!(pt.color, Some(5));
    assert_eq!(pt.size, None);
}

#[test]
fn test_parse_attributed_point_with_color_and_size() {
    let pt = parse_attributed_point("100,200:5:3");
    assert!(pt.is_some());
    let pt = pt.unwrap();
    assert_eq!(pt.x, 100);
    assert_eq!(pt.y, 200);
    assert_eq!(pt.color, Some(5));
    assert_eq!(pt.size, Some(3));
}

#[test]
fn test_parse_attributed_point_invalid_color() {
    // Color index 99 is out of range (0-13)
    let pt = parse_attributed_point("100,200:99");
    assert!(pt.is_none());

    // Color index 14 is out of range
    let pt = parse_attributed_point("100,200:14");
    assert!(pt.is_none());
}

#[test]
fn test_parse_attributed_point_size_clamped() {
    // Size should be clamped to MIN_BRUSH_SIZE..=MAX_BRUSH_SIZE (1-20)
    let pt = parse_attributed_point("100,200:0:0");
    assert!(pt.is_some());
    let pt = pt.unwrap();
    assert_eq!(pt.size, Some(1)); // Clamped to minimum

    let pt = parse_attributed_point("100,200:0:100");
    assert!(pt.is_some());
    let pt = pt.unwrap();
    assert_eq!(pt.size, Some(20)); // Clamped to maximum
}

#[test]
fn test_parse_polyline_with_attributes() {
    let cmd = parse_command("polyline 100,50:2 200,100:5 300,150:0");
    match cmd {
        Some(Command::Polyline(points)) => {
            assert_eq!(points.len(), 3);
            assert_eq!(points[0].x, 100);
            assert_eq!(points[0].y, 50);
            assert_eq!(points[0].color, Some(2));
            assert_eq!(points[1].color, Some(5));
            assert_eq!(points[2].color, Some(0));
        }
        _ => panic!("Expected Polyline command"),
    }
}

#[test]
fn test_parse_polyline_with_color_and_size() {
    let cmd = parse_command("polyline 100,50:2:5 200,100:5:3 300,150:0:1");
    match cmd {
        Some(Command::Polyline(points)) => {
            assert_eq!(points.len(), 3);
            assert_eq!(points[0].color, Some(2));
            assert_eq!(points[0].size, Some(5));
            assert_eq!(points[1].color, Some(5));
            assert_eq!(points[1].size, Some(3));
            assert_eq!(points[2].color, Some(0));
            assert_eq!(points[2].size, Some(1));
        }
        _ => panic!("Expected Polyline command"),
    }
}

#[test]
fn test_parse_points_mixed_attributes() {
    // Mixed: some use defaults, some override
    let cmd = parse_command("points 100,50 200,100:5 300,150:2:8");
    match cmd {
        Some(Command::Points(points)) => {
            assert_eq!(points.len(), 3);
            // First point: no overrides
            assert_eq!(points[0].color, None);
            assert_eq!(points[0].size, None);
            // Second point: color only
            assert_eq!(points[1].color, Some(5));
            assert_eq!(points[1].size, None);
            // Third point: color and size
            assert_eq!(points[2].color, Some(2));
            assert_eq!(points[2].size, Some(8));
        }
        _ => panic!("Expected Points command"),
    }
}

// ===================
// Attributed Point Execution Tests
// ===================

#[test]
fn test_execute_points_with_per_point_color() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black (default)
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let y = CANVAS_TOP + 100;
    // Draw points with different colors
    let cmd = Command::Points(vec![
        AttributedPoint { x: 100, y, color: Some(2), size: None }, // Red
        AttributedPoint { x: 150, y, color: Some(7), size: None }, // Green
        AttributedPoint { x: 200, y, color: None, size: None },    // Uses default (Black)
    ]);
    execute_command(
        &cmd,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Verify each point has the correct color
    assert_eq!(buffer[y * WIDTH + 100], COLOR_PALETTE[2], "Point at 100 should be red");
    assert_eq!(buffer[y * WIDTH + 150], COLOR_PALETTE[7], "Point at 150 should be green");
    assert_eq!(buffer[y * WIDTH + 200], COLOR_PALETTE[0], "Point at 200 should be black (default)");
}

#[test]
fn test_execute_points_mixed_attributes() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black (default)
    let mut fill_color_index: Option<usize> = None;
    let mut size = 3; // Default size

    let y = CANVAS_TOP + 100;
    // Draw points: first uses defaults, second overrides color, third overrides both
    let cmd = Command::Points(vec![
        AttributedPoint { x: 100, y, color: None, size: None },       // Uses defaults
        AttributedPoint { x: 200, y, color: Some(2), size: None },    // Red, default size
        AttributedPoint { x: 300, y, color: Some(5), size: Some(8) }, // Yellow, size 8
    ]);
    execute_command(
        &cmd,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Verify colors
    assert_eq!(buffer[y * WIDTH + 100], COLOR_PALETTE[0], "Point 1 should be black");
    assert_eq!(buffer[y * WIDTH + 200], COLOR_PALETTE[2], "Point 2 should be red");
    assert_eq!(buffer[y * WIDTH + 300], COLOR_PALETTE[5], "Point 3 should be yellow");

    // Verify size for point 3 - nearby pixels should also be colored
    // Size 8 means radius of 7, so pixels within 7 should be colored
    assert_eq!(buffer[y * WIDTH + 305], COLOR_PALETTE[5], "Point 3 should have size 8");
}

#[test]
fn test_execute_polyline_with_per_segment_color() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black (default)
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1;

    let y = CANVAS_TOP + 100;
    // Draw a rainbow line: segment colors are determined by the END point
    let cmd = Command::Polyline(vec![
        AttributedPoint { x: 100, y, color: None, size: None },    // Start point (no segment yet)
        AttributedPoint { x: 200, y, color: Some(2), size: None }, // Red segment (100->200)
        AttributedPoint { x: 300, y, color: Some(7), size: None }, // Green segment (200->300)
    ]);
    execute_command(
        &cmd,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // First segment (100->200) should be red (color from second point)
    assert_eq!(buffer[y * WIDTH + 150], COLOR_PALETTE[2], "Middle of first segment should be red");

    // Second segment (200->300) should be green (color from third point)
    assert_eq!(buffer[y * WIDTH + 250], COLOR_PALETTE[7], "Middle of second segment should be green");
}

#[test]
fn test_execute_polyline_with_per_segment_size() {
    let mut buffer = new_buffer();
    let mut edge_color_index: Option<usize> = Some(0); // Black
    let mut fill_color_index: Option<usize> = None;
    let mut size = 1; // Default size

    let y = CANVAS_TOP + 100;
    // Draw polyline with varying brush sizes
    let cmd = Command::Polyline(vec![
        AttributedPoint { x: 100, y, color: None, size: None },
        AttributedPoint { x: 150, y, color: None, size: Some(1) },  // Thin segment
        AttributedPoint { x: 200, y, color: None, size: Some(10) }, // Thick segment
    ]);
    execute_command(
        &cmd,
        &mut buffer,
        &mut edge_color_index,
        &mut fill_color_index,
        &mut size,
    );

    // Thin segment should only affect the center line
    assert_eq!(buffer[y * WIDTH + 125], BLACK, "Thin segment center");
    assert_eq!(buffer[(y + 5) * WIDTH + 125], WHITE, "Thin segment should not extend far");

    // Thick segment should affect nearby pixels
    assert_eq!(buffer[y * WIDTH + 175], BLACK, "Thick segment center");
    assert_eq!(buffer[(y + 5) * WIDTH + 175], BLACK, "Thick segment should extend");
}
