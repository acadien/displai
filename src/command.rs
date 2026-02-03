//! Command parsing and execution for the displai application.
//!
//! This module handles:
//! - Command enum definitions
//! - Parsing commands from text input
//! - Executing commands and modifying application state

use crate::drawing::{clear_canvas, draw_brush_line, draw_circle, draw_shape_with_fill};
use crate::{
    ToolMode, CANVAS_BOTTOM, CANVAS_TOP, COLOR_PALETTE, MAX_BRUSH_SIZE, MIN_BRUSH_SIZE, WIDTH,
};

/// A point with optional color and size overrides
#[derive(Debug, Clone, PartialEq)]
pub struct AttributedPoint {
    pub x: usize,
    pub y: usize,
    pub color: Option<usize>, // None = use current edge color
    pub size: Option<usize>,  // None = use current brush size
}

/// Commands that can be sent via stdin
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Snapshot,
    Color(usize),        // Legacy: sets edge color
    Edge(Option<usize>), // Set edge color (None = transparent)
    Fill(Option<usize>), // Set fill color (None = transparent)
    Size(usize),
    Stroke {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    },
    Dot {
        x: usize,
        y: usize,
    },
    Clear,
    State,
    // Shape commands
    Line {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    },
    Square {
        x: usize,
        y: usize,
        size: usize,
    },
    Rect {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    },
    Circle {
        x: usize,
        y: usize,
        r: usize,
    },
    Oval {
        x: usize,
        y: usize,
        rx: usize,
        ry: usize,
    },
    Triangle {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    },
    // Batch commands for performance (with optional per-point color/size attributes)
    Polyline(Vec<AttributedPoint>), // Connected line segments
    Points(Vec<AttributedPoint>),   // Multiple dots
}

/// Parse a point with optional color and size attributes
/// Format: x,y or x,y:color or x,y:color:size
pub fn parse_attributed_point(s: &str) -> Option<AttributedPoint> {
    // Split on colon first to separate coords from attributes
    let parts: Vec<&str> = s.split(':').collect();

    // Parse x,y from first part
    let coords: Vec<&str> = parts[0].split(',').collect();
    if coords.len() != 2 {
        return None;
    }
    let x = coords[0].parse().ok()?;
    let y = coords[1].parse().ok()?;

    // Parse optional color from second part
    let color = if parts.len() >= 2 {
        let c = parts[1].parse::<usize>().ok()?;
        if c < COLOR_PALETTE.len() {
            Some(c)
        } else {
            return None; // Invalid color index
        }
    } else {
        None
    };

    // Parse optional size from third part
    let size = if parts.len() >= 3 {
        let s = parts[2].parse::<usize>().ok()?;
        Some(s.clamp(MIN_BRUSH_SIZE, MAX_BRUSH_SIZE))
    } else {
        None
    };

    Some(AttributedPoint { x, y, color, size })
}

/// Parse a space-separated list of attributed points
fn parse_attributed_list(args: &str) -> Option<Vec<AttributedPoint>> {
    args.split_whitespace()
        .map(parse_attributed_point)
        .collect()
}

/// Parse a command string into a Command enum
pub fn parse_command(input: &str) -> Option<Command> {
    let input = input.trim();
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    match parts[0] {
        "snapshot" => Some(Command::Snapshot),
        "clear" => Some(Command::Clear),
        "state" => Some(Command::State),
        "color" => {
            if parts.len() >= 2 {
                parts[1]
                    .parse::<usize>()
                    .ok()
                    .filter(|&i| i < COLOR_PALETTE.len())
                    .map(Command::Color)
            } else {
                None
            }
        }
        "edge" => {
            if parts.len() >= 2 {
                if parts[1] == "none" {
                    Some(Command::Edge(None))
                } else {
                    parts[1]
                        .parse::<usize>()
                        .ok()
                        .filter(|&i| i < COLOR_PALETTE.len())
                        .map(|i| Command::Edge(Some(i)))
                }
            } else {
                None
            }
        }
        "fill" => {
            if parts.len() >= 2 {
                if parts[1] == "none" {
                    Some(Command::Fill(None))
                } else {
                    parts[1]
                        .parse::<usize>()
                        .ok()
                        .filter(|&i| i < COLOR_PALETTE.len())
                        .map(|i| Command::Fill(Some(i)))
                }
            } else {
                None
            }
        }
        "size" => {
            if parts.len() >= 2 {
                parts[1]
                    .parse::<usize>()
                    .ok()
                    .filter(|&s| (MIN_BRUSH_SIZE..=MAX_BRUSH_SIZE).contains(&s))
                    .map(Command::Size)
            } else {
                None
            }
        }
        "stroke" => {
            // stroke x1,y1 x2,y2
            if parts.len() >= 3 {
                let p1: Vec<&str> = parts[1].split(',').collect();
                let p2: Vec<&str> = parts[2].split(',').collect();
                if p1.len() == 2 && p2.len() == 2 {
                    let x1 = p1[0].parse::<usize>().ok()?;
                    let y1 = p1[1].parse::<usize>().ok()?;
                    let x2 = p2[0].parse::<usize>().ok()?;
                    let y2 = p2[1].parse::<usize>().ok()?;
                    Some(Command::Stroke { x1, y1, x2, y2 })
                } else {
                    None
                }
            } else {
                None
            }
        }
        "dot" => {
            // dot x,y
            if parts.len() >= 2 {
                let coords: Vec<&str> = parts[1].split(',').collect();
                if coords.len() == 2 {
                    let x = coords[0].parse::<usize>().ok()?;
                    let y = coords[1].parse::<usize>().ok()?;
                    Some(Command::Dot { x, y })
                } else {
                    None
                }
            } else {
                None
            }
        }
        "line" => {
            // line x1,y1 x2,y2
            if parts.len() >= 3 {
                let p1: Vec<&str> = parts[1].split(',').collect();
                let p2: Vec<&str> = parts[2].split(',').collect();
                if p1.len() == 2 && p2.len() == 2 {
                    let x1 = p1[0].parse::<usize>().ok()?;
                    let y1 = p1[1].parse::<usize>().ok()?;
                    let x2 = p2[0].parse::<usize>().ok()?;
                    let y2 = p2[1].parse::<usize>().ok()?;
                    Some(Command::Line { x1, y1, x2, y2 })
                } else {
                    None
                }
            } else {
                None
            }
        }
        "square" => {
            // square x,y size
            if parts.len() >= 3 {
                let coords: Vec<&str> = parts[1].split(',').collect();
                if coords.len() == 2 {
                    let x = coords[0].parse::<usize>().ok()?;
                    let y = coords[1].parse::<usize>().ok()?;
                    let size = parts[2].parse::<usize>().ok()?;
                    Some(Command::Square { x, y, size })
                } else {
                    None
                }
            } else {
                None
            }
        }
        "rect" => {
            // rect x1,y1 x2,y2
            if parts.len() >= 3 {
                let p1: Vec<&str> = parts[1].split(',').collect();
                let p2: Vec<&str> = parts[2].split(',').collect();
                if p1.len() == 2 && p2.len() == 2 {
                    let x1 = p1[0].parse::<usize>().ok()?;
                    let y1 = p1[1].parse::<usize>().ok()?;
                    let x2 = p2[0].parse::<usize>().ok()?;
                    let y2 = p2[1].parse::<usize>().ok()?;
                    Some(Command::Rect { x1, y1, x2, y2 })
                } else {
                    None
                }
            } else {
                None
            }
        }
        "circle" => {
            // circle x,y r
            if parts.len() >= 3 {
                let coords: Vec<&str> = parts[1].split(',').collect();
                if coords.len() == 2 {
                    let x = coords[0].parse::<usize>().ok()?;
                    let y = coords[1].parse::<usize>().ok()?;
                    let r = parts[2].parse::<usize>().ok()?;
                    Some(Command::Circle { x, y, r })
                } else {
                    None
                }
            } else {
                None
            }
        }
        "oval" => {
            // oval x,y rx,ry
            if parts.len() >= 3 {
                let coords: Vec<&str> = parts[1].split(',').collect();
                let radii: Vec<&str> = parts[2].split(',').collect();
                if coords.len() == 2 && radii.len() == 2 {
                    let x = coords[0].parse::<usize>().ok()?;
                    let y = coords[1].parse::<usize>().ok()?;
                    let rx = radii[0].parse::<usize>().ok()?;
                    let ry = radii[1].parse::<usize>().ok()?;
                    Some(Command::Oval { x, y, rx, ry })
                } else {
                    None
                }
            } else {
                None
            }
        }
        "triangle" => {
            // triangle x1,y1 x2,y2
            if parts.len() >= 3 {
                let p1: Vec<&str> = parts[1].split(',').collect();
                let p2: Vec<&str> = parts[2].split(',').collect();
                if p1.len() == 2 && p2.len() == 2 {
                    let x1 = p1[0].parse::<usize>().ok()?;
                    let y1 = p1[1].parse::<usize>().ok()?;
                    let x2 = p2[0].parse::<usize>().ok()?;
                    let y2 = p2[1].parse::<usize>().ok()?;
                    Some(Command::Triangle { x1, y1, x2, y2 })
                } else {
                    None
                }
            } else {
                None
            }
        }
        "polyline" => {
            // polyline x1,y1[:c[:s]] x2,y2[:c[:s]] x3,y3[:c[:s]] ...
            if parts.len() >= 3 {
                let args = parts[1..].join(" ");
                let points = parse_attributed_list(&args)?;
                if points.len() >= 2 {
                    Some(Command::Polyline(points))
                } else {
                    None
                }
            } else {
                None
            }
        }
        "points" => {
            // points x1,y1[:c[:s]] x2,y2[:c[:s]] x3,y3[:c[:s]] ...
            if parts.len() >= 2 {
                let args = parts[1..].join(" ");
                let points = parse_attributed_list(&args)?;
                if !points.is_empty() {
                    Some(Command::Points(points))
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Execute a command, modifying the buffer and/or state
/// Returns an optional response string to print to stdout
pub fn execute_command(
    cmd: &Command,
    buffer: &mut [u32],
    edge_color_index: &mut Option<usize>,
    fill_color_index: &mut Option<usize>,
    brush_size: &mut usize,
) -> Option<String> {
    match cmd {
        Command::Snapshot => {
            if let Err(e) = save_canvas_png(buffer, "canvas.png") {
                Some(format!("error: {}", e))
            } else {
                Some("saved canvas.png".to_string())
            }
        }
        Command::Color(index) => {
            *edge_color_index = Some(*index);
            None
        }
        Command::Edge(color_opt) => {
            *edge_color_index = *color_opt;
            None
        }
        Command::Fill(color_opt) => {
            *fill_color_index = *color_opt;
            None
        }
        Command::Size(size) => {
            *brush_size = *size;
            None
        }
        Command::Stroke { x1, y1, x2, y2 } => {
            if let Some(idx) = *edge_color_index {
                let color = COLOR_PALETTE[idx];
                draw_brush_line(buffer, *x1, *y1, *x2, *y2, color, *brush_size);
            }
            None
        }
        Command::Dot { x, y } => {
            if let Some(idx) = *edge_color_index {
                let color = COLOR_PALETTE[idx];
                draw_circle(buffer, *x, *y, *brush_size, color);
            }
            None
        }
        Command::Clear => {
            clear_canvas(buffer);
            None
        }
        Command::State => {
            let edge_str = match edge_color_index {
                Some(i) => i.to_string(),
                None => "none".to_string(),
            };
            let fill_str = match fill_color_index {
                Some(i) => i.to_string(),
                None => "none".to_string(),
            };
            Some(format!(
                "edge:{} fill:{} size:{}",
                edge_str, fill_str, *brush_size
            ))
        }
        Command::Line { x1, y1, x2, y2 } => {
            let edge_color = edge_color_index.map(|i| COLOR_PALETTE[i]);
            let fill_color = fill_color_index.map(|i| COLOR_PALETTE[i]);
            draw_shape_with_fill(
                buffer,
                ToolMode::Line,
                *x1,
                *y1,
                *x2,
                *y2,
                edge_color,
                fill_color,
                *brush_size,
            );
            None
        }
        Command::Square { x, y, size } => {
            let edge_color = edge_color_index.map(|i| COLOR_PALETTE[i]);
            let fill_color = fill_color_index.map(|i| COLOR_PALETTE[i]);
            // Convert top-left + size to bounding box coordinates
            let x2 = x + size;
            let y2 = y + size;
            draw_shape_with_fill(
                buffer,
                ToolMode::Square,
                *x,
                *y,
                x2,
                y2,
                edge_color,
                fill_color,
                *brush_size,
            );
            None
        }
        Command::Rect { x1, y1, x2, y2 } => {
            let edge_color = edge_color_index.map(|i| COLOR_PALETTE[i]);
            let fill_color = fill_color_index.map(|i| COLOR_PALETTE[i]);
            draw_shape_with_fill(
                buffer,
                ToolMode::Rectangle,
                *x1,
                *y1,
                *x2,
                *y2,
                edge_color,
                fill_color,
                *brush_size,
            );
            None
        }
        Command::Circle { x, y, r } => {
            let edge_color = edge_color_index.map(|i| COLOR_PALETTE[i]);
            let fill_color = fill_color_index.map(|i| COLOR_PALETTE[i]);
            // Convert center + radius to bounding box coordinates
            let x1 = x.saturating_sub(*r);
            let y1 = y.saturating_sub(*r);
            let x2 = x + r;
            let y2 = y + r;
            draw_shape_with_fill(
                buffer,
                ToolMode::Circle,
                x1,
                y1,
                x2,
                y2,
                edge_color,
                fill_color,
                *brush_size,
            );
            None
        }
        Command::Oval { x, y, rx, ry } => {
            let edge_color = edge_color_index.map(|i| COLOR_PALETTE[i]);
            let fill_color = fill_color_index.map(|i| COLOR_PALETTE[i]);
            // Convert center + radii to bounding box coordinates
            let x1 = x.saturating_sub(*rx);
            let y1 = y.saturating_sub(*ry);
            let x2 = x + rx;
            let y2 = y + ry;
            draw_shape_with_fill(
                buffer,
                ToolMode::Oval,
                x1,
                y1,
                x2,
                y2,
                edge_color,
                fill_color,
                *brush_size,
            );
            None
        }
        Command::Triangle { x1, y1, x2, y2 } => {
            let edge_color = edge_color_index.map(|i| COLOR_PALETTE[i]);
            let fill_color = fill_color_index.map(|i| COLOR_PALETTE[i]);
            draw_shape_with_fill(
                buffer,
                ToolMode::Triangle,
                *x1,
                *y1,
                *x2,
                *y2,
                edge_color,
                fill_color,
                *brush_size,
            );
            None
        }
        Command::Polyline(points) => {
            for window in points.windows(2) {
                // Use the END point's attributes for this segment
                let color_idx = window[1].color.or(*edge_color_index);
                if let Some(idx) = color_idx {
                    let color = COLOR_PALETTE[idx];
                    let size = window[1].size.unwrap_or(*brush_size);
                    draw_brush_line(
                        buffer,
                        window[0].x,
                        window[0].y,
                        window[1].x,
                        window[1].y,
                        color,
                        size,
                    );
                }
            }
            None
        }
        Command::Points(points) => {
            for pt in points {
                let color_idx = pt.color.or(*edge_color_index);
                if let Some(idx) = color_idx {
                    let color = COLOR_PALETTE[idx];
                    let size = pt.size.unwrap_or(*brush_size);
                    draw_circle(buffer, pt.x, pt.y, size, color);
                }
            }
            None
        }
    }
}

/// Save the canvas portion of the buffer to a PNG file
pub fn save_canvas_png(buffer: &[u32], path: &str) -> Result<(), String> {
    use image::{ImageBuffer, Rgb};

    let canvas_height = CANVAS_BOTTOM - CANVAS_TOP;
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(WIDTH as u32, canvas_height as u32);

    for y in 0..canvas_height {
        for x in 0..WIDTH {
            let pixel = buffer[(y + CANVAS_TOP) * WIDTH + x];
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
            img.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
        }
    }

    img.save(path).map_err(|e| e.to_string())
}
