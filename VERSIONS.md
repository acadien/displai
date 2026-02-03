# Version History

## Version 0.2.1 (Current)

Released: February 2026

### Features
- **Batch drawing commands** - `polyline` and `points` for efficient multi-point drawing
- **Per-point attributes** - Override color and size for individual points: `x,y[:color[:size]]`
- **AttributedPoint struct** - Internal representation for points with optional color/size overrides

### New Socket Commands
- `polyline x1,y1 x2,y2 [x3,y3 ...]` - Draw connected line segments
- `points x1,y1 [x2,y2 ...]` - Draw multiple dots at specified positions

### Per-Point Attribute Syntax
Each point can optionally specify color and size overrides:
- `100,200` - Use current edge color and brush size
- `100,200:5` - Use color index 5, current brush size
- `100,200:5:3` - Use color index 5, brush size 3

---

## Version 0.2

Released: February 2026

### Features
- **14-color palette** - Black, White, Red, Red-Orange, Orange, Yellow, Yellow-Green, Green, Cyan-Green, Cyan, Blue, Blue-Violet, Violet, Magenta
- **Edge/fill color system** - Separate edge and fill colors with transparent option
- **Shape tools (GUI)** - Line, Square, Rectangle, Circle, Oval, Triangle with click-drag interaction
- **Shape socket commands** - `line`, `square`, `rect`, `circle`, `oval`, `triangle`
- **Color socket commands** - `edge` and `fill` commands for setting colors via socket
- **Clear canvas button** - Red button in toolbar to clear canvas
- **Transparent color button** - Checkerboard button for transparent edge/fill

### Changes from v0.1
- Removed eraser tool (use white color instead)
- Changed `color` command to set edge color
- Added `edge` and `fill` commands
- Updated `state` output format: `edge:N|none fill:N|none size:N`

---

## Version 0.1

Released: January 2026

### Features
- 13-color palette
- Brush sizes 1-20
- Eraser tool
- Command protocol via Unix socket (`/tmp/displai.sock`)
- PNG snapshot export
- Basic drawing with mouse

### Socket Commands
- `snapshot` - Save canvas to PNG
- `color <0-12>` - Select color
- `eraser on|off` - Toggle eraser
- `size <1-20>` - Set brush size
- `stroke x1,y1 x2,y2` - Draw line
- `dot x,y` - Draw dot
- `clear` - Clear canvas
- `state` - Get current state
