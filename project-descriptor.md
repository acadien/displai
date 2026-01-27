# displai

A whiteboard-style drawing application built with Rust and minifb, featuring AI interaction via Claude.

---

## Current Version: 0.1

### Features
- 800x600 pixel canvas with custom title bar and bottom toolbar
- **Drawing**: Click and drag to draw continuous strokes
- **13-color palette**: Black, Red, Orange, Yellow, Green, Cyan, Blue, Violet, Magenta, and more
- **Brush sizes**: Adjustable from 1-20 pixels via +/- buttons
- **Eraser tool**: Toggle eraser mode to remove strokes
- **Snapshot**: Save canvas to PNG file

### Claude Integration
Control the canvas programmatically via Unix socket or stdin:

```bash
# Connect via Unix socket
echo "state" | nc -U /tmp/displai.sock

# Available commands:
snapshot              # Save canvas.png, returns "saved canvas.png"
color <0-12>          # Select color from palette
eraser on|off         # Toggle eraser mode
size <1-20>           # Set brush size
stroke x1,y1 x2,y2    # Draw line between points
dot x,y               # Draw single dot
clear                 # Clear canvas to white
state                 # Returns "color:N eraser:on|off size:N"
```

### Technical Stack
- **Rust** with minifb (minimal framebuffer library)
- Direct pixel buffer manipulation (`Vec<u32>`, RGB format `0xRRGGBB`)
- Bresenham's line algorithm for smooth strokes
- Unix domain socket at `/tmp/displai.sock` for external control

---

## Future Versions

### Version 0.2 (Planned)
- Undo/redo functionality
- Shape tools (rectangle, circle, line)

### Version 0.3 (Planned)
- Collaboration features (multi-user drawing)
- Layer support

### Version 0.4 (Planned)
- Enhanced AI interaction (autonomous drawing, image understanding)
