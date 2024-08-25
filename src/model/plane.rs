use std::f64::consts;

use renderer::VectorRow;

const LENGTH: f64 = 15.2;
const WING_SPAN: f64 = 8.6;
const UNIT: f64 = 15.2 / 21.0;

fn add_vertex(
    vertices: &mut Vec<VectorRow<f64, 3>>,
    vertices_counter: &mut usize,
    vertex: VectorRow<f64, 3>,
) {
    vertices.push(vertex);
    *vertices_counter += 1;
}

mod exhaust {
    use super::*;

    pub fn add_vertices(vertices: &mut Vec<VectorRow<f64, 3>>) -> usize {
        let mut count: usize = 0;
        let points = 24.0;
        let radius = UNIT / 2.0;

        for around_x_axis in 0..(points as usize) {
            add_vertex(
                vertices,
                &mut count,
                VectorRow::from([
                    0.0,
                    radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).sin(),
                    radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).cos(),
                ]),
            );
        }

        let radius = radius * 1.5;
        for around_x_axis in 0..(points as usize) {
            add_vertex(
                vertices,
                &mut count,
                VectorRow::from([
                    UNIT,
                    radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).sin(),
                    radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).cos(),
                ]),
            );
        }

        count
    }
}

mod fuselage {
    use super::*;

    pub fn add_vertices(vertices: &mut Vec<VectorRow<f64, 3>>) -> usize {
        let mut count: usize = 0;

        add_vertex(vertices, &mut count, VectorRow::from([LENGTH, 0.0, 0.0]));
        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([LENGTH + 0.5, 0.0, 0.0]),
        );

        count
    }
}

mod rudder {
    use super::*;

    pub fn add_vertices(vertices: &mut Vec<VectorRow<f64, 3>>) -> usize {
        let mut count: usize = 0;

        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([1.0, 0.0, 0.7]),
        );
        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([1.3, 0.0, 3.0 - UNIT]),
        );
        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([1.3, 0.0, 3.0]),
        );
        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([1.3 + UNIT, 0.0, 3.0]),
        );
        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([1.3 + UNIT + UNIT * 4.0, 0.0, 1.0]),
        );

        count
    }
}

mod left_wing {
    use super::*;

    pub fn add_vertices(vertices: &mut Vec<VectorRow<f64, 3>>) -> usize {
        let mut count: usize = 0;

        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([4.0 * UNIT, 2.0 * UNIT, 0.0]),
        );
        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([4.5 * UNIT, (2.0 + 0.5) * UNIT, 0.0]),
        );
        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([4.5 * UNIT, (WING_SPAN / 2.0), 0.0]),
        );
        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([(4.5 + 1.0) * UNIT, (WING_SPAN / 2.0), 0.0]),
        );
        add_vertex(
            vertices,
            &mut count,
            VectorRow::from([(6.5 + 7.0) * UNIT, 2.0 * UNIT, 0.0]),
        );

        count
    }
}

// mod left_canard {
//     use super::*;

//     pub fn add_vertices(vertices: &mut Vec<VectorRow<f64, 3>>) -> usize {
//         let mut count: usize = 0;

//         add_vertex(
//             vertices,
//             &mut count,
//             VectorRow::from([4.0 * UNIT, 2.0 * UNIT, 0.0]),
//         );

//         count
//     }
// }

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];
    let mut vertices_count = 0;

    vertices_count += exhaust::add_vertices(&mut vertices);
    vertices_count += fuselage::add_vertices(&mut vertices);
    vertices_count += rudder::add_vertices(&mut vertices);
    vertices_count += left_wing::add_vertices(&mut vertices);

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
        lines.push(vec![i * 3, i * 3 + 24]);
    }

    // Sidroder
    lines.push(vec![24, 50, 50 + 1, 50 + 2, 50 + 3, 50 + 4]);

    // Left wing
    lines.push(vec![55, 55 + 1, 55 + 2, 55 + 3, 55 + 4]);

    lines
}
