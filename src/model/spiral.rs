use std::f64::consts;

use renderer::VectorRow;

const MAX_DEPTH: i32 = 1000;
const GRID_SIZE: i32 = 200;
const GRID_SPACING: i32 = 100;

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    // Spiral zooming in.
    for i in 0..MAX_DEPTH {
        vertices.push(VectorRow::from([
            i as f64 * (((i as f64) / 16.0) % (consts::PI * 2.0)).cos(),
            i as f64,
            i as f64 * (((i as f64) / 16.0) % (consts::PI * 2.0)).sin(),
        ]));
    }

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

    // Particles (points).
    for i in 1..(GRID_SIZE * GRID_SIZE) {
        lines.push(vec![i as usize]);
    }

    // Particles (lines).
    for i in 1..MAX_DEPTH {
        lines.push(vec![i as usize - 1, i as usize]);
    }

    lines
}