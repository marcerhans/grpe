use std::f64::consts;

use renderer::VectorRow;

const LENGTH: f64 = 15.2;
const WING_SPAN: f64 = 8.6;
const HEIGHT: f64 = 4.5;
const UNIT: f64 = 15.2 / 21.0;

// const LENGTH_PX: f64 = 650.0; // Length i pixels based on an image (without pitot tube).
fn mirror_y(vertices: &Vec<VectorRow<f64, 3>>) -> Vec<VectorRow<f64, 3>> {
    // Duplicate and mirror.
    let mut mirror = vertices.clone();

    for vertex in &mut mirror {
        vertex[1] = vertex[1] * -1.0;
    }

    mirror
}

mod exhaust {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();
        let points = 24.0;
        let radius = UNIT / 2.25;

        for around_x_axis in 0..(points as usize) {
            vertices.push(VectorRow::from([
                0.0,
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).sin(),
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).cos(),
            ]));
        }

        let radius = (UNIT / 2.0) * 1.5;
        for around_x_axis in 0..(points as usize) {
            vertices.push(VectorRow::from([
                UNIT,
                radius
                    * (consts::PI / 10.0 + around_x_axis as f64 * (2.0 * consts::PI / points))
                        .sin(),
                radius
                    * (consts::PI / 10.0 + around_x_axis as f64 * (2.0 * consts::PI / points))
                        .cos(),
            ]));
        }

        vertices
    }
}

mod fuselage {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        vertices.append(&mut vec![
            VectorRow::from([LENGTH, 0.0, 0.0]),
            VectorRow::from([LENGTH + 0.5, 0.0, 0.0]),
        ]);

        vertices
    }
}

mod rudder {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        vertices.append(&mut vec![
            VectorRow::from([1.1, 0.0, 0.65]),
            VectorRow::from([1.3, 0.0, 3.0 - UNIT]),
            VectorRow::from([1.3, 0.0, 3.0]),
            VectorRow::from([1.3 + UNIT, 0.0, 3.0]),
            VectorRow::from([1.3 + UNIT + UNIT * 4.0, 0.0, 1.0]),
        ]);

        vertices
    }
}

mod wings {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        // Outline
        vertices.append(&mut vec![
            VectorRow::from([2.88, 1.0, 0.0]),
            VectorRow::from([3.22, 1.42, 0.0]),
            VectorRow::from([3.24, 2.6, 0.0]),
            VectorRow::from([3.3, 4.0, 0.0]),
            VectorRow::from([3.55, 4.0, 0.0]),
            VectorRow::from([4.3, 4.0, 0.0]),
            VectorRow::from([5.9, 2.92, 0.0]),
            VectorRow::from([5.7, 2.92, 0.0]),
            VectorRow::from([8.2, 1.15, 0.0]),
            VectorRow::from([8.9, 0.9, 0.0]),
        ]);

        // Extra points to draw details
        vertices.append(&mut vec![
            VectorRow::from([3.82, 0.95, 0.0]),
            VectorRow::from([3.69, 2.52, 0.0]),
            VectorRow::from([3.64, 2.52, 0.0]),
            VectorRow::from([5.45, 2.92, 0.0]),
        ]);

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![vec![]];

        // Outline
        line_draw_order.append(&mut vec![vec![
            start + 0,
            start + 1,
            start + 2,
            start + 3,
            start + 4,
            start + 5,
            start + 6,
            start + 7,
            start + 8,
            start + 9,
        ]]);

        // Extra points to draw details
        line_draw_order.append(&mut vec![
            vec![start + 0, start + 10, start + 11, start + 2],
            vec![start + 12, start + 4],
            vec![start + 13, start + 7],
        ]);

        // Mirror
        let start = start + 14;
        line_draw_order.append(&mut vec![vec![
            start + 0,
            start + 1,
            start + 2,
            start + 3,
            start + 4,
            start + 5,
            start + 6,
            start + 7,
            start + 8,
            start + 9,
        ]]);

        // Extra points to draw details
        line_draw_order.append(&mut vec![
            vec![start + 0, start + 10, start + 11, start + 2],
            vec![start + 12, start + 4],
            vec![start + 13, start + 7],
        ]);

        line_draw_order
    }
}

mod canards {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        // Outline
        vertices.append(&mut vec![
            VectorRow::from([8.63, 0.9, 0.0]),
            VectorRow::from([8.63, 1.07, 0.0]),
            VectorRow::from([8.0, 2.05, 0.0]),
            VectorRow::from([8.45, 2.05, 0.0]),
            VectorRow::from([10.5, 0.9, 0.0]),
        ]);

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![vec![]];

        // Outline
        line_draw_order.append(&mut vec![vec![
            start + 0,
            start + 1,
            start + 2,
            start + 3,
            start + 4,
            start + 0,
        ]]);

        // Mirror
        let start = start + 5;
        line_draw_order.append(&mut vec![vec![
            start + 0,
            start + 1,
            start + 2,
            start + 3,
            start + 4,
            start + 0,
        ]]);

        line_draw_order
    }
}

mod intake {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        // Outline
        vertices.append(&mut vec![
        ]);

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![vec![]];

        // Outline
        line_draw_order.append(&mut vec![vec![
        ]]);

        line_draw_order
    }
}

mod cockpit {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        vertices.append(&mut vec![
            VectorRow::from([(6.5 + 7.0 + 0.5) * UNIT, 0.2, 1.0]),
            VectorRow::from([(6.5 + 7.0 + 0.5 * 5.0) * UNIT, 0.3, 1.0]),
            VectorRow::from([(6.5 + 7.0 + 0.5 * 5.0 + 0.5 * 7.5) * UNIT, 0.3, 1.0]),
            VectorRow::from([
                (6.5 + 7.0 + 0.5 * 5.0 + 0.5 * 7.5 + 0.5 * 4.0) * UNIT,
                0.3,
                1.0,
            ]),
        ]);

        let mut mirror = Vec::new();

        for vertex in &vertices {
            let mut v = vertex.clone();
            v[1] = -v[1];
            mirror.push(v);
        }

        vertices.append(&mut mirror);

        vertices
    }
}

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    // Length for reference
    vertices.append(&mut vec![
        VectorRow::from([0.0, 0.0, 0.0]),
        VectorRow::from([LENGTH, 0.0, 0.0]),
    ]);

    // vertices.append(&mut exhaust::get_vertices());
    // vertices.append(&mut fuselage::get_vertices());
    // vertices.append(&mut rudder::get_vertices());

    vertices.append(&mut wings::get_vertices());
    vertices.append(&mut canards::get_vertices());
    vertices.append(&mut intake::get_vertices());

    // vertices.append(&mut cockpit::get_vertices());

    // Scale and center
    for vertex in vertices.iter_mut() {
        vertex[0] = vertex[0] - 15.7 / 2.0; // Center plane
        vertex[1] = vertex[1] + 0.05; // Whilst using draft image to center it.
        vertex.0.scale(21.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut line_draw_order = vec![];

    let mut index_start = 2;

    let mut wings = wings::get_line_draw_order(index_start);
    index_start += wings::get_vertices().len();
    line_draw_order.append(&mut wings);

    let mut canards = canards::get_line_draw_order(index_start);
    index_start += canards::get_vertices().len();
    line_draw_order.append(&mut canards);

    let mut intake = intake::get_line_draw_order(index_start);
    index_start += intake::get_vertices().len();
    line_draw_order.append(&mut intake);

    line_draw_order
}
