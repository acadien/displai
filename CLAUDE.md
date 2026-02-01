# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo run                # Build and run
cargo run --release      # Build and run with optimizations
cargo check              # Quick compilation check (no binary output)
cargo fmt                # Format code
cargo clippy             # Lint
cargo test               # Run all tests
cargo test test_name     # Run a specific test
cargo test --test drawing_tests  # Run a specific test file
```

## Project Structure

```
src/
  lib.rs      # Core logic (public functions, constants)
  main.rs     # Entry point, calls displai::run()
tests/
  drawing_tests.rs  # Pixel and line drawing tests
  button_tests.rs   # Button hit detection tests
  ui_tests.rs       # Title bar and UI rendering tests
  command_tests.rs  # Command parsing and execution tests
```

## Testing

**All code changes must include tests that validate the behavior.**

### Testing Strategy

Since minifb uses a direct pixel buffer, we test rendering logic without opening a window:

1. **Unit tests** - Test pure functions (`draw_line`, `set_pixel`, `is_in_*_button`, etc.) by passing a buffer and verifying pixel values
2. **Integration tests** - Test UI behavior by simulating inputs and checking buffer state
3. **Visual regression** (optional) - Use `image` crate to save buffers as PNGs for comparison

### Writing Tests

Tests live in the `tests/` directory. Each test file imports from the library:

```rust
use displai::*;

fn new_buffer() -> Vec<u32> {
    vec![WHITE; WIDTH * HEIGHT]
}

#[test]
fn test_example() {
    let mut buffer = new_buffer();
    set_pixel(&mut buffer, 100, 100, BLACK);
    assert_eq!(buffer[100 * WIDTH + 100], BLACK);
}
```

### Test Organization

- `drawing_tests.rs` - Tests for `set_pixel`, `draw_line`, boundary conditions
- `button_tests.rs` - Tests for `is_in_close_button`, `is_in_color_button`
- `ui_tests.rs` - Tests for `draw_title_bar`, `draw_button`, rendering
- `command_tests.rs` - Tests for `parse_command`, `execute_command`, PNG export

### Test Requirements

- Test buffer modifications, not window behavior
- Verify boundary conditions (edges, title bar area)
- Test button hit detection functions
- When adding new UI elements, add corresponding `is_in_*` tests
- Ensure new logic is valid so that future changes preserve reliability

### Pre-Commit Workflow (MANDATORY)

Before any code changes can be committed:

1. **Add/modify tests** - New behavior must have corresponding tests that validate it works
2. **Do not disable old tests** - Existing tests can only be disabled after explicit permission from the user
3. **Run tests** - Execute `cargo test` and verify all tests pass
4. **Only then commit** - Code cannot be committed until tests pass

## Architecture

**displai** is a whiteboard-style drawing application built with Rust and minifb, featuring AI interaction via Claude through a Unix socket interface.

### Current Implementation (v0.1)

- `src/lib.rs` - Core logic, all public functions and constants
- `src/main.rs` - Entry point, calls `displai::run()`
- 800x600 window with custom title bar (30px) and bottom toolbar (60px)
- Drawable canvas area: 800x510 pixels (from y=30 to y=540)
- Direct pixel buffer manipulation using a `Vec<u32>` (linear array, index = `y * WIDTH + x`)
- RGB pixel format: `0xRRGGBB`
- Bresenham's line algorithm for continuous drawing
- minifb handles window creation and input events

### UI Elements

- **Title bar**: Gray bar at top with close button
- **Close button (X)**: Red button in top-right corner, exits application
- **Bottom toolbar**: Two rows containing:
  - Row 1: 13 color palette buttons
  - Row 2: Eraser button, brush size display, +/- size buttons

### Rendering Pattern

Immediate mode rendering with a simple game loop:
1. Redraw title bar and buttons each frame
2. Handle mouse input (left-click to draw, button clicks)
3. Update pixel buffer with pen strokes
4. Render buffer to window via `update_with_buffer()`

### Key Constants (in lib.rs)

- `WIDTH`/`HEIGHT`: Canvas dimensions (800x600)
- `TITLE_BAR_HEIGHT`: 30 pixels
- `BOTTOM_TOOLBAR_HEIGHT`: 60 pixels
- `BUTTON_SIZE`: 24 pixels
- `COLOR_PALETTE`: 14 colors (Black, White, Red, Red-Orange, Orange, Yellow, Yellow-Green, Green, Cyan-Green, Cyan, Blue, Blue-Violet, Violet, Magenta)
- `MIN_BRUSH_SIZE`/`MAX_BRUSH_SIZE`: 1-20 pixels
- `SOCKET_PATH`: `/tmp/displai.sock`

### Command Protocol

Control via Unix socket (`/tmp/displai.sock`) or stdin:

```
snapshot              -> saves canvas.png, returns "saved canvas.png"
color <0-13>          -> select edge color from palette (0=Black, 1=White acts as eraser)
size <1-20>           -> set brush size
stroke x1,y1 x2,y2    -> draw brush stroke between points
dot x,y               -> draw single dot at position
clear                 -> clear canvas to white
state                 -> returns "edge:N fill:N|none size:N"

# Shape commands (use current edge/fill colors and brush size)
line x1,y1 x2,y2      -> draw line between two points
square x,y size       -> draw square at top-left corner with side length
rect x1,y1 x2,y2      -> draw rectangle with corners at points
circle x,y r          -> draw circle at center with radius
oval x,y rx,ry        -> draw oval at center with x/y radii
triangle x1,y1 x2,y2  -> draw triangle in bounding box
```

## Roadmap

### Version 0.1
- ✅ 13-color palette
- ✅ Brush sizes 1-20
- ✅ Eraser tool
- ✅ Command protocol (stdin + Unix socket)
- ✅ PNG snapshot export

### Version 0.2 (Current)
- ✅ 14-color palette (Black, White + 12 colors)
- ✅ Edge/fill color system with transparent option
- ✅ Shape tools (line, square, rectangle, circle, oval, triangle)
- ✅ Shape socket commands
- ✅ Clear canvas button

### Version 0.3 (Planned)
- Undo/redo functionality
- Polygon support
- Collaboration features (multi-user drawing)
- Layer support

### Version 0.4 (Planned)
- Enhanced AI interaction (autonomous drawing, image understanding)
