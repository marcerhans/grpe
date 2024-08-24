use std::f64::consts;

use renderer::VectorRow;

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    // let single_step = 15.2 / 12.0;
    // for along_x_axis in 0..12 {
    //     let x_pos = single_step * (along_x_axis as f64);

    // }

    // Exhaust
    {
        let points = 24.0;
        let radius = 0.4;
        for around_x_axis in 0..(points as usize) {
            vertices.push(VectorRow::from([
                0.0,
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).sin(),
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).cos(),
            ]));
        }

        let radius = 0.5;
        for around_x_axis in 0..(points as usize) {
            vertices.push(VectorRow::from([
                0.7,
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).sin(),
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).cos(),
            ]));
        }
    }

    vertices.append(&mut vec![
        // Fuselage
        VectorRow::from([15.2, 0.0, 0.0]),        // Tip
        VectorRow::from([15.7, 0.0, 0.0]),        // Tip + pitot
        VectorRow::from([15.2 / 2.0, 0.0, -1.0]), // Bottom at middle
        // Sidroder
        VectorRow::from([1.0, 0.0, 0.5]),
        VectorRow::from([1.1, 0.0, 3.0]),
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
        vertex.0.scale(16.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut lines = vec![];

    lines.push(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 0,
    ]); // Exhaust1
    lines.push(vec![
        1 + 23,
        2 + 23,
        3 + 23,
        4 + 23,
        5 + 23,
        6 + 23,
        7 + 23,
        8 + 23,
        9 + 23,
        11 + 23,
        11 + 23,
        12 + 23,
        13 + 23,
        14 + 23,
        15 + 23,
        16 + 23,
        17 + 23,
        18 + 23,
        19 + 23,
        20 + 23,
        21 + 23,
        22 + 23,
        23 + 23,
        24 + 23,
        1 + 23,
    ]); // Exhaust2

    for i in 0..8 {
        lines.push(vec![i*3,i*3+24]); // Between exhaust 1 and 2
    }
    // lines.push(vec![27, 28, 29, 30]);

    lines
}
