use std::f64::consts;

use renderer::VectorRow;

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    // Spiral zooming in.
    const MAX_DEPTH: i32 = 1000;
    for i in 0..MAX_DEPTH {
        vertices.push(VectorRow::from([
            i as f64 * (((i as f64) / 16.0) % (consts::PI * 2.0)).cos(),
            i as f64,
            i as f64 * (((i as f64) / 16.0) % (consts::PI * 2.0)).sin(),
        ]));
    }

    const GRID_SIZE: i32 = 200;
    const GRID_SPACING: i32 = 100;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            vertices.push(VectorRow::from([
                (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (i * GRID_SPACING) as f64,
                MAX_DEPTH as f64,
                (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (j * GRID_SPACING) as f64,
            ]));
        }
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut lines = vec![];

    let mut line = vec![];
    for i in 0..1000 {
        line.push(i);
    }
    lines.push(line);
    lines.push(vec![999, 0]);

    lines
}