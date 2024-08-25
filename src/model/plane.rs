use std::f64::consts;

use renderer::VectorRow;

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    let length = 15.2;
    let wing_span = 8.6;
    let unit = 15.2 / 21.0;

    {
        // Exhaust
        let points = 24.0;
        let radius = 0.4;
        for around_x_axis in 0..(points as usize) {
            vertices.push(VectorRow::from([
                0.0,
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).sin(),
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).cos(),
            ]));
        }

        let radius = 0.6;
        for around_x_axis in 0..(points as usize) {
            vertices.push(VectorRow::from([
                unit,
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).sin(),
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).cos(),
            ]));
        }
    }

    {
        // Fuselage
        vertices.append(&mut vec![
            VectorRow::from([length, 0.0, 0.0]),        // Tip
            VectorRow::from([length + 0.5, 0.0, 0.0]),  // Tip + pitot
        ]);
    }

    {
        // Sidroder
        vertices.append(&mut vec![
            VectorRow::from([1.0, 0.0, 0.7]),
            VectorRow::from([1.3, 0.0, 3.0 - unit]),
            VectorRow::from([1.3, 0.0, 3.0]),
            VectorRow::from([1.3 + unit, 0.0, 3.0]),
            VectorRow::from([1.3 + unit + unit * 4.0, 0.0, 1.0]),
        ]);
    }

    {
        // Left wing
        vertices.append(&mut vec![
            VectorRow::from([4.0 * unit, 2.0 * unit, 0.0]),
            VectorRow::from([4.5 * unit, (2.0 + 0.5) * unit, 0.0]),
            VectorRow::from([4.5 * unit, (wing_span / 2.0), 0.0]),
            VectorRow::from([(4.5 + 1.0) * unit, (wing_span / 2.0), 0.0]),
            VectorRow::from([(6.5 + 7.0) * unit, 2.0 * unit, 0.0]),
        ]);
    }

        // // Left canard wing
        // VectorRow::from([15.2 / 2.0, 0.0, 4.3 / 1.75]), // Wing span is 8.6 (/ 2 = 4.3)
        // VectorRow::from([15.2 / 2.0 + 0.4, 0.0, 4.3 / 1.75]),
        // VectorRow::from([15.2 / 2.0 + 2.0, 0.0, 1.0]),

    for vertex in vertices.iter_mut() {
        vertex[0] = vertex[0] - 15.7 / 2.0; // Center plane
        vertex.0.scale(16.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut lines = vec![];

    // Exhaust1
    lines.push(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 0,
    ]);

    // Exhaust2
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
    ]);

    // Between exhaust 1 and 2
    for i in 0..8 {
        lines.push(vec![i*3,i*3+24]);
    }

    // Sidroder
    lines.push(vec![
        24,
        50,
        50 + 1,
        50 + 2,
        50 + 3,
        50 + 4,
    ]);

    // Left wing
    lines.push(vec![
        24 + 6,
        55,
        55+1,
        55+2,
        55+3,
        55+4,
    ]);

    lines
}
