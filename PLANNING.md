# Planning

This document consolidates all future feature plans for displai.

---

## Version 0.2 Remaining Items

Small UI fixes:

### 1. Clear Button Icon
Change the red X clear button to a "C" icon for clarity.

### 2. Brush Button Icon
Make the brush button look more like a brush/pen/pencil (currently looks like a short line).

### ~~3. Documentation Update~~
~~Update CLAUDE.md and README.md to reflect current state (v0.2 features, updated command list).~~
**COMPLETED** - All documentation updated to v0.2.1 (README.md, VERSIONS.md, CLAUDE.md, ARTIST.md created).

---

## Version 0.3 (Planned)

### Undo/Redo
- Implement undo stack for drawing operations
- Add redo functionality
- Socket commands: `undo`, `redo`

### Polygon Support
- GUI tool for drawing arbitrary polygons
- Click to add vertices, double-click to close
- Socket command: `polygon x1,y1 x2,y2 x3,y3 ...`

### Collaboration Features
- Multi-user drawing support
- Shared canvas state

### Layer Support
- Multiple drawing layers
- Layer visibility toggle
- Layer ordering

---

## Version 0.4 (Planned)

### Enhanced AI Interaction
- Autonomous drawing capabilities
- Image understanding integration
- Better coordination between AI and user drawing

---

## Future Features

Ideas from drawing sessions and usage:

### Drawing Enhancements
- **Custom RGB colors** - `edge #RRGGBB` for arbitrary colors beyond palette
- **Bezier curves** - `bezier x1,y1 cx,cy x2,y2` for smooth curves
- **Arc command** - `arc x,y r start_angle end_angle` for partial circles
- **Polygon fill** - `polygon x1,y1 x2,y2 x3,y3 ...` for arbitrary filled shapes
- **Better polyline joins** - Mitered or rounded joins instead of simple overlap

---

## Open Questions

1. **Shape preview** - Should rubber-banding (live preview while dragging) be added?
2. **Text tool** - Add ability to draw text on canvas?
3. **Selection tool** - Add ability to select and move drawn elements?
4. **Export formats** - Support additional export formats beyond PNG?
