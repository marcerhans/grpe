use std::f64::consts;

use renderer::VectorRow;

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    // let single_step = 15.2 / 12.0;
    // for along_x_axis in 0..12 {
    //     let x_pos = single_step * (along_x_axis as f64);

    // }

    // Exhaust
    for around_x_axis in 0..24 {
        vertices.push(
            VectorRow::from([0.0,  0.4 * (around_x_axis as f64 * (2.0 * consts::PI / 24.0)).sin(), 0.4 * (around_x_axis as f64 * (2.0 * consts::PI / 24.0)).cos()])
        );
    }

    // Prime
    vertices.append(&mut vec![
        // Fuselage
        VectorRow::from([15.2, 0.0, 0.0]), // Tip
        VectorRow::from([15.7, 0.0, 0.0]), // Tip + pitot
        VectorRow::from([15.2 / 2.0, 0.0, -1.0]), // Bottom at middle
        // Sidroder
        VectorRow::from([0.9, 0.0, 0.5]),
        VectorRow::from([1.0, 0.0, 3.0]),
        VectorRow::from([1.7, 0.0, 3.0]),
        VectorRow::from([1.7 + 2.5, 0.0, 0.5]),
        // // Left wing
        // VectorRow::from([1.2, 0.0, 4.3]), // Wing span is 8.6 (/ 2 = 4.3)
        // VectorRow::from([2.0, 0.0, 4.3]),
        // VectorRow::from([15.2 / 2.0, 0.0, 1.0]),
        // // Left canard wing
        // VectorRow::from([15.2 / 2.0, 0.0, 4.3 / 1.75]), // Wing span is 8.6 (/ 2 = 4.3)
        // VectorRow::from([15.2 / 2.0 + 0.4, 0.0, 4.3 / 1.75]),
        // VectorRow::from([15.2 / 2.0 + 2.0, 0.0, 1.0]),
    ]);

    for vertex in vertices.iter_mut() {
        vertex.0.scale(10.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut lines = vec![];
    lines.push(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]);
    lines.push(vec![27, 28, 29, 30]);
    lines
}