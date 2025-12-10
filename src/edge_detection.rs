use crate::capture::Screenshot;

const EDGE_THRESHOLD: i32 = 5;

#[derive(Debug, Clone, Copy)]
pub struct Edges {
    pub left: u32,
    pub right: u32,
    pub up: u32,
    pub down: u32,
}

pub fn find_edges(screenshot: &Screenshot, cursor_x: u32, cursor_y: u32) -> Edges {
    Edges {
        left: scan_horizontal(screenshot, cursor_x, cursor_y, -1).unwrap_or(0),
        right: scan_horizontal(screenshot, cursor_x, cursor_y, 1).unwrap_or(screenshot.width - 1),
        up: scan_vertical(screenshot, cursor_x, cursor_y, -1).unwrap_or(0),
        down: scan_vertical(screenshot, cursor_x, cursor_y, 1).unwrap_or(screenshot.height - 1),
    }
}

fn scan_horizontal(screenshot: &Screenshot, start_x: u32, y: u32, direction: i32) -> Option<u32> {
    let mut x = start_x as i32;
    let mut prev_lum = screenshot.get_luminance(start_x, y) as i32;

    loop {
        x += direction;
        if x < 0 || x >= screenshot.width as i32 {
            return None;
        }

        let lum = screenshot.get_luminance(x as u32, y) as i32;
        if (lum - prev_lum).abs() > EDGE_THRESHOLD {
            return Some(if direction < 0 {
                (x + 1) as u32
            } else {
                (x - 1) as u32
            });
        }
        prev_lum = lum;
    }
}

fn scan_vertical(screenshot: &Screenshot, x: u32, start_y: u32, direction: i32) -> Option<u32> {
    let mut y = start_y as i32;
    let mut prev_lum = screenshot.get_luminance(x, start_y) as i32;

    loop {
        y += direction;
        if y < 0 || y >= screenshot.height as i32 {
            return None;
        }

        let lum = screenshot.get_luminance(x, y as u32) as i32;
        if (lum - prev_lum).abs() > EDGE_THRESHOLD {
            return Some(if direction < 0 {
                (y + 1) as u32
            } else {
                (y - 1) as u32
            });
        }
        prev_lum = lum;
    }
}

/// Snap threshold in pixels - how close to an edge before snapping occurs
const SNAP_THRESHOLD: u32 = 200;

/// Find the nearest vertical edge from a given x position, preferring the inward direction.
/// `prefer_direction`: 1 = prefer rightward (for left edge), -1 = prefer leftward (for right edge)
pub fn snap_to_nearest_edge_x(
    screenshot: &Screenshot,
    x: u32,
    y: u32,
    prefer_direction: i32,
) -> u32 {
    // First try the preferred (inward) direction
    let preferred_edge =
        scan_horizontal_limited(screenshot, x, y, prefer_direction, SNAP_THRESHOLD);
    if let Some(edge) = preferred_edge {
        return edge;
    }

    // Fallback to the opposite direction
    let fallback_edge =
        scan_horizontal_limited(screenshot, x, y, -prefer_direction, SNAP_THRESHOLD);
    fallback_edge.unwrap_or(x)
}

/// Find the nearest horizontal edge from a given y position, preferring the inward direction.
/// `prefer_direction`: 1 = prefer downward (for top edge), -1 = prefer upward (for bottom edge)
pub fn snap_to_nearest_edge_y(
    screenshot: &Screenshot,
    x: u32,
    y: u32,
    prefer_direction: i32,
) -> u32 {
    // First try the preferred (inward) direction
    let preferred_edge = scan_vertical_limited(screenshot, x, y, prefer_direction, SNAP_THRESHOLD);
    if let Some(edge) = preferred_edge {
        return edge;
    }

    // Fallback to the opposite direction
    let fallback_edge = scan_vertical_limited(screenshot, x, y, -prefer_direction, SNAP_THRESHOLD);
    fallback_edge.unwrap_or(y)
}

/// Scan horizontally for an edge within a limited distance
fn scan_horizontal_limited(
    screenshot: &Screenshot,
    start_x: u32,
    y: u32,
    direction: i32,
    max_distance: u32,
) -> Option<u32> {
    let mut x = start_x as i32;
    let mut prev_lum = screenshot.get_luminance(start_x, y) as i32;
    let mut distance = 0u32;

    loop {
        x += direction;
        distance += 1;

        if distance > max_distance || x < 0 || x >= screenshot.width as i32 {
            return None;
        }

        let lum = screenshot.get_luminance(x as u32, y) as i32;
        if (lum - prev_lum).abs() > EDGE_THRESHOLD {
            // Return the position just before the edge (inside the element)
            return Some(if direction < 0 {
                (x + 1) as u32
            } else {
                (x - 1) as u32
            });
        }
        prev_lum = lum;
    }
}

/// Scan vertically for an edge within a limited distance
fn scan_vertical_limited(
    screenshot: &Screenshot,
    x: u32,
    start_y: u32,
    direction: i32,
    max_distance: u32,
) -> Option<u32> {
    let mut y = start_y as i32;
    let mut prev_lum = screenshot.get_luminance(x, start_y) as i32;
    let mut distance = 0u32;

    loop {
        y += direction;
        distance += 1;

        if distance > max_distance || y < 0 || y >= screenshot.height as i32 {
            return None;
        }

        let lum = screenshot.get_luminance(x, y as u32) as i32;
        if (lum - prev_lum).abs() > EDGE_THRESHOLD {
            // Return the position just before the edge (inside the element)
            return Some(if direction < 0 {
                (y + 1) as u32
            } else {
                (y - 1) as u32
            });
        }
        prev_lum = lum;
    }
}
