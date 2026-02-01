# Version 0.2 Project Plan: Shape Tools

## Overview

Add shape drawing tools via both GUI (toolbar buttons) and socket commands. Replace eraser with color-based approach. Shapes initially use a single color; edge/fill separation comes later.

---

## Phase 1: Color Palette Refactor

### 1.1 Remove Eraser, Add Black and White Colors
- Remove `eraser_mode` state variable
- Remove eraser button from toolbar
- Reorder `COLOR_PALETTE` to start with Black (`0x000000`) and White (`0xFFFFFF`)
- New palette order: Black, White, Red, Red-Orange, Orange, Yellow, Yellow-Green, Green, Cyan-Green, Cyan, Blue, Blue-Violet, Violet, Magenta (14 colors)

### 1.2 Update Command Protocol
- Remove `eraser on|off` command (no longer needed; use white color instead)
- Update `state` command output format:
  - Old: `color:N eraser:on|off size:N`
  - New: `color:N size:N`
- Update `color` command to use new indices (0=Black, 1=White, 2-13=colors)
- Document that selecting white (index 1) acts as eraser

### 1.3 Update Tests
- Remove eraser-related tests
- Add tests for white color drawing (eraser behavior)
- Update `state` command tests

---

## Phase 2: Shape Tool Infrastructure

### 2.1 Add Tool Mode State
- Create `ToolMode` enum: `Brush`, `Line`, `Square`, `Rectangle`, `Circle`, `Oval`, `Triangle`
- Add `current_tool: ToolMode` state variable (default: `Brush`)
- Brush mode uses existing freehand drawing logic

### 2.2 Add Shape Tool Buttons to Toolbar
- Redesign Row 2 layout:
  ```
  [Brush] [Line] [Sq] [Rect] [Circ] [Oval] [Tri] | Size: 3 [+][-]
  ```
- Add button rendering for each shape tool
- Add `is_in_*_tool_button()` hit detection functions
- Highlight currently selected tool

### 2.3 Shape Drawing Interaction Model
- Click-drag to define shape bounds
- Store `drag_start: Option<(usize, usize)>` for shape drawing
- On mouse down: record start point, enter dragging state
- On mouse up: draw final shape, clear dragging state
- Preview shape while dragging (optional, can defer)

### 2.4 Update Tests
- Add tests for tool button hit detection
- Add tests for tool mode switching
- Add tests for drag state management

---

## Phase 3: Basic Shape Implementation (Drawing Functions)

### 3.1 Line Tool
- `draw_line()` already exists, reuse it
- Line tool uses click-drag: start point to end point

### 3.2 Square Tool
- Add `draw_square(buffer, x, y, size, color, brush_size)`
- Draw four equal-length lines forming square outline
- Drag defines one corner; square expands from that point
- Side length = max of drag width/height (constrained to equal sides)

### 3.3 Rectangle Tool
- Add `draw_rectangle(buffer, x1, y1, x2, y2, color, brush_size)`
- Draw four lines forming rectangle outline
- Respect brush size for line thickness

### 3.4 Circle Tool
- Add `draw_circle(buffer, cx, cy, radius, color, brush_size)`
- Use midpoint circle algorithm
- Center at drag start, radius = distance to drag end

### 3.5 Oval Tool
- Add `draw_oval(buffer, cx, cy, rx, ry, color, brush_size)`
- Use midpoint ellipse algorithm
- Bounding box defined by drag start/end

### 3.6 Triangle Tool
- Add `draw_triangle(buffer, x1, y1, x2, y2, color, brush_size)`
- Equilateral or isoceles triangle fitting in bounding box
- Base at bottom, apex at top center

### 3.7 Update Tests
- Add tests for each shape drawing function
- Test boundary conditions (shapes at canvas edges)
- Test various brush sizes

---

## Phase 4: Socket Commands for Shapes

### 4.1 Add Shape Commands
- `line x1,y1 x2,y2` - draw line between two points
- `square x,y size` - draw square at top-left corner with side length
- `rect x1,y1 x2,y2` - draw rectangle with corners at points
- `circle x,y r` - draw circle at center with radius
- `oval x,y rx,ry` - draw oval at center with x/y radii
- `triangle x1,y1 x2,y2` - draw triangle in bounding box

### 4.2 Update Command Parser
- Add parsing for new shape commands
- Validate coordinates are within canvas bounds
- Use current color and brush size

### 4.3 Update Documentation
- Update CLAUDE.md command protocol section
- Add shape command examples

### 4.4 Update Tests
- Add command parsing tests for each shape
- Add execution tests verifying shapes are drawn correctly

---

## Phase 6: Edge and Fill Colors (Future)

### 6.1 Dual Color State
- Add `edge_color: u32` and `fill_color: Option<u32>` state
- `None` fill means transparent (outline only)
- Add special "transparent" color constant

### 6.2 UI for Color Selection
- Click color = set edge color
- Right-click or Shift+click = set fill color
- Visual indicator showing current edge/fill colors
- Transparent option in palette or separate button

### 6.3 Update Shape Drawing
- Modify shape functions to accept edge and fill colors
- Draw fill first, then edge on top
- Implement flood fill or scanline fill for shapes

### 6.4 Update Socket Commands
- Extend commands: `rect x1,y1 x2,y2 [edge_color] [fill_color]`
- Add `edge <0-14|transparent>` command
- Add `fill <0-14|transparent>` command
- Update `state` command output

### 6.5 Update Tests
- Test filled shapes
- Test transparent edge/fill
- Test color selection UI

---

## Phase 5: Hodge Podge

### 5.1: Change the red X button for clearing the canvas to a big C
### 5.2: Make the brush button more clearly a brush, it looks just like a short line now, maybe change it to a pen or pencil?
### 5.3: Update the claude.md and the readme.md to reflect the current state of the project

---

## Implementation Order

1. **Phase 1** - Color palette refactor (removes eraser complexity) ✅ COMPLETE
2. **Phase 2** - Shape tool infrastructure (buttons, tool modes) ✅ COMPLETE
3. **Phase 3** - Basic shapes (line, square, rect, circle, oval, triangle) ✅ COMPLETE
4. **Phase 6** - Edge/fill colors ✅ COMPLETE
5. **Phase 4** - Socket commands for shapes ✅ COMPLETE
6. **Phase 5** - Random fixes

---

## Files to Modify

- `src/lib.rs` - Core shape logic, state, constants, command parsing
- `src/main.rs` - Possibly no changes (all logic in lib)
- `tests/drawing_tests.rs` - Shape drawing tests
- `tests/button_tests.rs` - Tool button hit detection tests
- `tests/command_tests.rs` - Shape command parsing tests
- `tests/ui_tests.rs` - Toolbar rendering tests
- `CLAUDE.md` - Update command protocol documentation

---

## Open Questions

1. Should shape preview (rubber-banding) while dragging be in v0.2 or deferred?
2. Triangle style: equilateral, isoceles, or right triangle?
3. Should shapes respect the title bar / toolbar boundaries, or allow drawing anywhere?
