# displai

Ever wished you could interact with an AI through ... drawing? Well look no further. This vibe coded slop generates a drawing window that your agent can interact with through a simple unix socket.

## Getting Started

### Install

```bash
git clone https://github.com/acadien/displai.git
cd displai
cargo build --release
```

### Run

```bash
cargo run --release
```

A drawing window opens. Draw with your mouse. That's it.

### Connect an AI agent

displai listens on a Unix socket at `/tmp/displai.sock`. Any agent (or script) can send commands:

```bash
echo "state" | nc -U /tmp/displai.sock
# → edge:0 fill:none size:1

echo "stroke 100,100 300,200" | nc -U /tmp/displai.sock
echo "snapshot" | nc -U /tmp/displai.sock
# → saves canvas.png
```

**Available commands:**

| Command | Description |
|---|---|
| `snapshot` | Save canvas to `canvas.png` |
| `state` | Get current edge color, fill color, and size |
| `clear` | Clear canvas to white |

**Color & Brush:**

| Command | Description |
|---|---|
| `color <0-13>` | Set edge color (legacy, same as `edge`) |
| `edge <0-13\|none>` | Set edge/stroke color (`none` = transparent) |
| `fill <0-13\|none>` | Set fill color (`none` = no fill) |
| `size <1-20>` | Set brush size |

**Drawing:**

| Command | Description |
|---|---|
| `dot x,y` | Draw single dot at position |
| `stroke x1,y1 x2,y2` | Draw brush stroke between points |
| `points x,y [x,y ...]` | Draw multiple dots |
| `polyline x,y x,y [x,y ...]` | Draw connected line segments |

**Shapes** (use current edge/fill colors):

| Command | Description |
|---|---|
| `line x1,y1 x2,y2` | Draw line between two points |
| `square x,y size` | Draw square at top-left corner |
| `rect x1,y1 x2,y2` | Draw rectangle with corners at points |
| `circle x,y r` | Draw circle at center with radius |
| `oval x,y rx,ry` | Draw oval at center with x/y radii |
| `triangle x1,y1 x2,y2` | Draw triangle in bounding box |

**Per-point attributes:**

For `points` and `polyline`, you can specify color and size per point:
- `x,y` - use current edge color and brush size
- `x,y:color` - override color (0-13)
- `x,y:color:size` - override both color and size

### Use with Claude Code

Start displai, then ask Claude things like:

- **"What did I draw?"** - Claude snapshots the canvas and describes it
- **"Draw a blue circle in the top right"** - Claude sends stroke commands to draw
- **"Change the cat's color to orange"** - Claude snapshots, finds the element, erases and redraws it
- **"Add a tree next to the house"** - Claude reads the canvas and draws new elements in context
- **"Draw a chart of population growth"** - Claude generates data visualizations directly on canvas

The agent sees the canvas via `snapshot` and draws via `stroke`/`dot` commands. It's a shared whiteboard between you and the AI.

## License

Licensed under the Business Source License 1.1 (BSL). See [LICENSE](LICENSE) for the full text.

**TL;DR:** You can view, modify, and use this code for any non-commercial purpose. Commercial use requires a license from the author. On **January 27, 2029**, the license automatically converts to Apache 2.0, making it fully open source.

## Examples

<img width="2751" height="1001" alt="Screenshot From 2026-01-26 17-05-23" src="https://github.com/user-attachments/assets/a66a0de9-017e-498c-af89-be03c1221e7c" />

<img width="2751" height="1013" alt="Screenshot From 2026-01-26 16-53-32" src="https://github.com/user-attachments/assets/e651f9eb-9ff0-4da5-90a1-f47bf8bf2485" />

<img width="2751" height="1013" alt="Screenshot From 2026-01-26 16-48-08" src="https://github.com/user-attachments/assets/abe072e4-180c-4c2a-a7d6-27bf115eb8be" />

<img width="2751" height="1013" alt="Screenshot From 2026-01-26 16-42-15" src="https://github.com/user-attachments/assets/ad0cc0ad-4dca-433f-8ac6-41e0c7963cc8" />

<img width="2751" height="1013" alt="Screenshot From 2026-01-26 16-27-16" src="https://github.com/user-attachments/assets/e24713dc-6e9f-4d9b-9c34-ab0efd032c8a" />
