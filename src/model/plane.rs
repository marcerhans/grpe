/// Modelled with image of brazillian plane. Top down image. FOV: 15.
use std::f64::consts;

use renderer::VectorRow;

const LENGTH: f64 = 15.2;
const WING_SPAN: f64 = 8.6;
const HEIGHT: f64 = 4.5;

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

        let radius = 0.3;
        for around_x_axis in 0..(points as usize) {
            vertices.push(VectorRow::from([
                0.0,
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).sin(),
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).cos(),
            ]));
        }

        let radius = 0.42;
        for around_x_axis in 0..(points as usize) {
            vertices.push(VectorRow::from([
                0.62,
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

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        let points = 24;
        let mut small = vec![vec![]];
        let mut large = vec![vec![]];

        for i in 0..points {
            small[0].push(start + i);
            large[0].push(start + i + points);

            if i % 2 == 0 {
                line_draw_order.push(vec![start + i, start + points + (i + 2) % points]);
            }
        }

        small[0].push(start);
        large[0].push(start + points);

        line_draw_order.append(&mut small);
        line_draw_order.append(&mut large);

        line_draw_order
    }
}

mod fuselage {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        // Outline
        vertices.append(&mut vec![
            VectorRow::from([0.62, 0.42, 0.0]),
            VectorRow::from([0.92, 0.48, 0.0]),
            VectorRow::from([0.98, 0.54, 0.0]),
            VectorRow::from([2.0, 0.75, 0.0]),
            VectorRow::from([2.15, 0.86, 0.0]),
            VectorRow::from([2.65, 0.98, 0.0]),
            VectorRow::from([5.65, 0.97, 0.0]),
            VectorRow::from([8.65, 0.94, 0.0]),
            VectorRow::from([10.0, 0.98, 0.0]),
            VectorRow::from([10.525, 1.06, 0.0]),
            VectorRow::from([10.7, 0.95, 0.0]),
            VectorRow::from([10.87, 0.90, 0.0]),
            VectorRow::from([10.92, 0.84, 0.0]),
            VectorRow::from([10.98, 0.62, 0.0]),
            VectorRow::from([11.1, 0.55, 0.0]),
            VectorRow::from([11.1, 0.53, 0.0]),
            VectorRow::from([10.88, 0.51, 0.0]),
            VectorRow::from([10.86, 0.51, 0.0]),
            VectorRow::from([10.86, 0.47, 0.0]),
        ]);

        vertices.append(&mut mirror_y(&vertices));

        vertices.append(&mut vec![VectorRow::from([15.2, 0.0, 0.0])]);

        // Extra points to draw details
        vertices.append(&mut vec![VectorRow::from([15.9, 0.0, 0.0])]);

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        // Outline
        let mut order = vec![];
        for i in 0..19 {
            order.push(start + i);
        }
        // order.push(start + 28);

        let mut mirror_order = vec![];
        for i in 19..38 {
            mirror_order.push(start + i);
        }
        // mirror_order.push(start + 28);

        line_draw_order.push(order);
        line_draw_order.push(mirror_order);

        line_draw_order
    }
}

mod rudder {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        // Outline
        vertices.append(&mut vec![
            VectorRow::from([0.95, 0.0, 0.60]),
            VectorRow::from([1.15, 0.0, 2.2]),
            VectorRow::from([1.15, 0.0, 2.8]),
            VectorRow::from([1.82, 0.0, 2.8]),
            VectorRow::from([4.3, 0.0, 1.0]),
        ]);

        // Extra points to draw details
        vertices.append(&mut vec![
            // Pitot tube
            VectorRow::from([3.4, 0.0, 1.5]),
            VectorRow::from([4.3, 0.0, 1.5]),
            // Blocky thing
            VectorRow::from([1.05, 0.0, 2.2]),
            VectorRow::from([1.15, 0.0, 2.25]),
            VectorRow::from([3.05, 0.0, 2.25]),
            VectorRow::from([3.2, 0.0, 2.2]),
            VectorRow::from([3.05, 0.0, 2.15]),
            VectorRow::from([1.15, 0.0, 2.15]),
        ]);

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        // Outline
        line_draw_order.append(&mut vec![vec![
            start + 0,
            start + 1,
            start + 2,
            start + 3,
            start + 4,
        ]]);

        // Extra points to draw details
        line_draw_order.append(&mut vec![
            // Pitot tube
            vec![start + 5, start + 6],
            vec![
                start + 7,
                start + 8,
                start + 9,
                start + 10,
                start + 11,
                start + 12,
            ],
        ]);

        line_draw_order
    }
}

mod wings {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        // Outline
        vertices.append(&mut vec![
            VectorRow::from([2.68, 1.0, 0.0]),
            VectorRow::from([3.07, 1.4, 0.0]),
            VectorRow::from([3.24, 4.3, 0.0]),
            VectorRow::from([4.3, 4.3, 0.0]),
            VectorRow::from([5.9, 3.15, 0.0]),
            VectorRow::from([5.65, 3.15, 0.0]),
            VectorRow::from([8.1, 1.23, 0.0]),
            VectorRow::from([8.8, 0.94, 0.0]),
        ]);

        // Extra points to draw details
        // vertices.append(&mut vec![
        //     VectorRow::from([3.82, 0.95, 0.0]),
        //     VectorRow::from([3.69, 2.52, 0.0]),
        //     VectorRow::from([3.64, 2.52, 0.0]),
        //     VectorRow::from([5.45, 2.92, 0.0]),
        // ]);

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

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
        ]]);

        // Extra points to draw details
        // line_draw_order.append(&mut vec![
        //     vec![start + 0, start + 10, start + 11, start + 2],
        //     vec![start + 12, start + 4],
        //     vec![start + 13, start + 7],
        // ]);

        // Mirror
        let start = start + 8;
        line_draw_order.append(&mut vec![vec![
            start + 0,
            start + 1,
            start + 2,
            start + 3,
            start + 4,
            start + 5,
            start + 6,
            start + 7,
        ]]);

        // Extra points to draw details
        // line_draw_order.append(&mut vec![
        //     vec![start + 0, start + 10, start + 11, start + 2],
        //     vec![start + 12, start + 4],
        //     vec![start + 13, start + 7],
        // ]);

        line_draw_order
    }
}

mod canards {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        // Outline
        vertices.append(&mut vec![
            VectorRow::from([8.68, 0.98, 0.0]),
            VectorRow::from([8.52, 1.05, 0.0]),
            VectorRow::from([8.51, 1.3, 0.0]),
            VectorRow::from([7.93, 2.28, 0.0]),
            VectorRow::from([8.42, 2.28, 0.0]),
            VectorRow::from([10.45, 1.07, 0.0]),
            VectorRow::from([10.2, 1.0, 0.0]),
            VectorRow::from([9.5, 0.97, 0.0]),
        ]);

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

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
            start + 0,
        ]]);

        // Mirror
        let start = start + 8;
        line_draw_order.append(&mut vec![vec![
            start + 0,
            start + 1,
            start + 2,
            start + 3,
            start + 4,
            start + 5,
            start + 6,
            start + 7,
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
        vertices.append(&mut vec![]);

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        // Outline
        line_draw_order.append(&mut vec![vec![]]);

        line_draw_order
    }
}

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    // Length for reference
    vertices.append(&mut vec![
        VectorRow::from([0.0, 0.0, 0.0]),
        VectorRow::from([LENGTH, 0.0, 0.0]),
        VectorRow::from([4.0, WING_SPAN / 2.0, 0.0]),
        VectorRow::from([4.0, -WING_SPAN / 2.0, 0.0]),
    ]);

    vertices.append(&mut exhaust::get_vertices());
    vertices.append(&mut fuselage::get_vertices());
    // vertices.append(&mut rudder::get_vertices());
    vertices.append(&mut wings::get_vertices());
    vertices.append(&mut canards::get_vertices());
    // vertices.append(&mut intake::get_vertices());

    // vertices.append(&mut cockpit::get_vertices());

    // Scale and center
    for vertex in vertices.iter_mut() {
        vertex[0] = vertex[0] - 15.7 / 2.0; // Center plane
        // vertex.0.scale(20.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut line_draw_order = vec![];

    let mut index_start = 4;
    line_draw_order.append(&mut vec![
        vec![0, 1],
        vec![2, 3],
    ]);

    let mut exhaust = exhaust::get_line_draw_order(index_start);
    index_start += exhaust::get_vertices().len();
    line_draw_order.append(&mut exhaust);

    let mut fuselage = fuselage::get_line_draw_order(index_start);
    index_start += fuselage::get_vertices().len();
    line_draw_order.append(&mut fuselage);

    // let mut rudder = rudder::get_line_draw_order(index_start);
    // index_start += rudder::get_vertices().len();
    // line_draw_order.append(&mut rudder);

    let mut wings = wings::get_line_draw_order(index_start);
    index_start += wings::get_vertices().len();
    line_draw_order.append(&mut wings);

    let mut canards = canards::get_line_draw_order(index_start);
    index_start += canards::get_vertices().len();
    line_draw_order.append(&mut canards);

    // let mut intake = intake::get_line_draw_order(index_start);
    // index_start += intake::get_vertices().len();
    // line_draw_order.append(&mut intake);

    line_draw_order
}
