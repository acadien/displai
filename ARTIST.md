# ARTIST.md

Guidelines for AI agents using displai's drawing interface.

---

## Canvas Coordinates

- **Window size**: 800x600 pixels
- **Canvas area**: y=30 to y=540 (510px drawable height)
- **Coordinate format**: `x,y` (origin at top-left)
- **Valid x range**: 0-799
- **Valid y range**: 30-539 (within canvas)

---

## Color Palette Reference

| Index | Color | Hex |
|-------|-------|-----|
| 0 | Black | `#000000` |
| 1 | White | `#FFFFFF` |
| 2 | Red | `#E04040` |
| 3 | Red-Orange | `#E07040` |
| 4 | Orange | `#E0A040` |
| 5 | Yellow | `#E0E040` |
| 6 | Yellow-Green | `#A0E040` |
| 7 | Green | `#40E040` |
| 8 | Cyan-Green | `#40E0A0` |
| 9 | Cyan | `#40E0E0` |
| 10 | Blue | `#4080E0` |
| 11 | Blue-Violet | `#4040E0` |
| 12 | Violet | `#8040E0` |
| 13 | Magenta | `#E040E0` |

**Transparency**: Use `edge none` or `fill none` for transparent edges/fills.

---

## Drawing Best Practices

### Use Shapes for Solid Areas
Instead of drawing many points, use filled shapes:
```bash
# Bad: many points to fill a rectangle
points 100,100 101,100 102,100 ...

# Good: single filled rectangle
fill 2
rect 100,100 200,150
```

### Set Fill Before Drawing Shapes
Fill color must be set BEFORE the shape command:
```bash
fill 7          # Set green fill
edge 0          # Set black edge
circle 400,300 50   # Draw filled circle with black outline
```

### Use Polyline for Connected Paths
For drawing continuous lines (like handwriting or curves):
```bash
polyline 100,100 150,120 200,110 250,130 300,100
```

### Use Points for Scattered Dots
For multiple unconnected dots:
```bash
points 100,100 200,150 300,200 400,250
```

### Per-Point Attributes for Multi-Color Drawings
Draw a rainbow line without changing global state:
```bash
polyline 100,200:2 200,200:4 300,200:5 400,200:7 500,200:10
```

---

## Shape Commands Quick Reference

| Command | Example | Description |
|---------|---------|-------------|
| `line` | `line 100,100 300,200` | Line between two points |
| `square` | `square 100,100 50` | 50x50 square at (100,100) |
| `rect` | `rect 100,100 200,150` | Rectangle from corner to corner |
| `circle` | `circle 400,300 50` | Circle centered at (400,300), radius 50 |
| `oval` | `oval 400,300 80,40` | Oval centered at (400,300), radii 80x40 |
| `triangle` | `triangle 100,200 200,100` | Triangle in bounding box |

**Edge vs Fill behavior**:
- Edge color draws the outline
- Fill color fills the interior
- Set either to `none` for no edge/fill
- Lines don't have fill (only edge)

---

## Efficient Drawing Patterns

### Layer Order Matters
Draw background elements first, foreground last:
```bash
# Draw sky
fill 9
rect 0,30 800,300

# Draw ground
fill 7
rect 0,300 800,540

# Draw sun (on top)
fill 5
edge 4
circle 650,100 40
```

### Reset with Clear
Use `clear` to reset the canvas to white:
```bash
clear
```

### Batch Commands for Performance
Send multiple shapes in sequence - each command executes immediately:
```bash
fill 2
circle 100,300 30
circle 200,300 30
circle 300,300 30
```

---

## Socket Communication

### Basic Command Format
```bash
printf "command\n" | nc -U -q 1 /tmp/displai.sock
```

**Important**: Always include the newline (`\n`).

### Multi-Line Mode
Send multiple commands in one connection (only first command returns response):
```bash
printf "edge 2\nfill 5\ncircle 400,300 50\n" | nc -U -q 1 /tmp/displai.sock
```

### Check Current State
```bash
echo "state" | nc -U /tmp/displai.sock
# Returns: edge:0 fill:none size:1
```

### Save Canvas
```bash
echo "snapshot" | nc -U /tmp/displai.sock
# Returns: saved canvas.png
```

---

## Common Patterns

### Draw a House
```bash
# Walls
fill 4
edge 0
rect 200,350 400,500

# Roof
fill 2
triangle 200,350 400,250

# Door
fill 3
rect 270,400 330,500

# Windows
fill 9
square 210,370 40
square 350,370 40
```

### Draw a Tree
```bash
# Trunk
fill 3
edge 0
rect 380,400 420,500

# Leaves
fill 7
circle 400,350 60
```

### Draw a Smiley Face
```bash
# Face
fill 5
edge 0
circle 400,300 80

# Eyes
fill 0
circle 370,280 10
circle 430,280 10

# Mouth (arc approximation with polyline)
edge 0
fill none
polyline 350,330 370,350 400,360 430,350 450,330
```
