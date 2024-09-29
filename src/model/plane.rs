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

fn mirror_z(vertices: &Vec<VectorRow<f64, 3>>) -> Vec<VectorRow<f64, 3>> {
    // Duplicate and mirror.
    let mut mirror = vertices.clone();

    for vertex in &mut mirror {
        vertex[2] = vertex[2] * -1.0;
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

        vertices.append(&mut vec![
            // Outline
            VectorRow::from([2.68, 1.0, 0.0]), // +
            VectorRow::from([3.07, 1.45, 0.0]),
            VectorRow::from([3.15, 2.2, 0.0]),  // +
            VectorRow::from([3.19, 3.17, 0.0]), // +
            VectorRow::from([3.24, 4.3, 0.0]),
            VectorRow::from([4.3, 4.3, 0.0]), // +
            VectorRow::from([5.95, 3.17, 0.0]),
            VectorRow::from([5.65, 3.17, 0.0]), // +
            VectorRow::from([6.85, 2.2, 0.0]),  // +
            VectorRow::from([8.15, 1.26, 0.0]),
            VectorRow::from([8.8, 0.98, 0.0]),
            // +Z
            VectorRow::from([4.3, 3.17, 0.02]),  // +
            VectorRow::from([4.3, 2.2, 0.04]),   // +
            VectorRow::from([4.3, 1.0, 0.1]), // +
            VectorRow::from([5.65, 2.2, 0.04]),  // +
            VectorRow::from([5.65, 1.0, 0.1]),  // +
            VectorRow::from([6.85, 1.0, 0.07]), // +
            // -Z
            VectorRow::from([4.3, 3.17, -0.02]),  // +
            VectorRow::from([4.3, 2.2, -0.04]),   // +
            VectorRow::from([4.3, 1.0, -0.1]), // +
            VectorRow::from([5.65, 2.2, -0.04]),  // +
            VectorRow::from([5.65, 1.0, -0.1]),  // +
            VectorRow::from([6.85, 1.0, -0.07]), // +
        ]);

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        let next = |x: &mut usize| {
            let prev = *x;
            *x += 1;
            prev
        };

        let add_lines = |start: usize, mut ldo: Vec<Vec<usize>>| -> Vec<Vec<usize>> {
            let mut count = 0;

            ldo.append(&mut vec![
                // Outline
                vec![
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                ],
                // +Z
                vec![start + 0, start + 13, start + 15, start + 16, start + 10],
                vec![start + 2, start + 12, start + 14, start + 8],
                vec![start + 3, start + 11, start + 7],
                vec![start + 5, start + 11, start + 12, start + 13],
                vec![start + 7, start + 14, start + 15],
                vec![start + 8, start + 16],
                // -Z
                vec![start + 0, start + 19, start + 21, start + 22, start + 10],
                vec![start + 2, start + 18, start + 20, start + 8],
                vec![start + 3, start + 17, start + 7],
                vec![start + 5, start + 17, start + 18, start + 19],
                vec![start + 7, start + 20, start + 21],
                vec![start + 8, start + 22],
            ]);

            ldo
        };

        // Left
        line_draw_order = add_lines(start, line_draw_order);

        // Right (Mirror)
        line_draw_order = add_lines(start + 23, line_draw_order);

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
            VectorRow::from([8.54, 1.08, 0.0]),
            VectorRow::from([8.54, 1.28, 0.0]), // +
            VectorRow::from([8.25, 1.76, 0.0]), // +
            VectorRow::from([7.93, 2.32, 0.0]),
            VectorRow::from([8.42, 2.32, 0.0]),  // +
            VectorRow::from([9.33, 1.76, 0.0]),  // +
            VectorRow::from([10.12, 1.28, 0.0]), // +
            VectorRow::from([10.43, 1.09, 0.0]),
            // +Z
            VectorRow::from([8.54, 1.76, 0.01]), // +
            VectorRow::from([9.33, 1.28, 0.01]), // +
            VectorRow::from([9.33, 1.0, 0.04]),  // +
            VectorRow::from([10.12, 1.02, 0.01]), // +
            // -Z
            VectorRow::from([8.54, 1.76, -0.01]), // +
            VectorRow::from([9.33, 1.28, -0.01]), // +
            VectorRow::from([9.33, 1.0, -0.04]),  // +
            VectorRow::from([10.12, 1.02, -0.01]), // +
        ]);

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        let next = |x: &mut usize| {
            let prev = *x;
            *x += 1;
            prev
        };

        let add_lines = |start: usize, mut ldo: Vec<Vec<usize>>| -> Vec<Vec<usize>> {
            let mut count = 0;

            ldo.append(&mut vec![
                // Outline
                vec![
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                    start + next(&mut count),
                ],
                // +Z
                vec![start + 0, start + 11, start + 12, start + 8],
                vec![start + 2, start + 10, start + 7],
                vec![start + 3, start + 9, start + 6],
                vec![start + 5, start + 9, start + 2],
                vec![start + 6, start + 11],
                vec![start + 7, start + 12],
                // -Z
                vec![start + 0, start + 15, start + 16, start + 8],
                vec![start + 2, start + 14, start + 7],
                vec![start + 3, start + 13, start + 6],
                vec![start + 5, start + 13, start + 2],
                vec![start + 6, start + 15],
                vec![start + 7, start + 16],
            ]);

            ldo
        };

        // Outline
        line_draw_order = add_lines(start, line_draw_order);

        // Mirror
        line_draw_order = add_lines(start + 17, line_draw_order);

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
    // vertices.append(&mut vec![
    //     VectorRow::from([0.0, 0.0, 0.0]),
    //     VectorRow::from([LENGTH, 0.0, 0.0]),
    //     VectorRow::from([4.0, WING_SPAN / 2.0, 0.0]),
    //     VectorRow::from([4.0, -WING_SPAN / 2.0, 0.0]),
    // ]);

    // vertices.append(&mut exhaust::get_vertices());
    // vertices.append(&mut fuselage::get_vertices());
    // vertices.append(&mut rudder::get_vertices());
    vertices.append(&mut wings::get_vertices());
    vertices.append(&mut canards::get_vertices());
    // vertices.append(&mut intake::get_vertices());

    // vertices.append(&mut cockpit::get_vertices());

    // Scale and center
    for vertex in vertices.iter_mut() {
        vertex[0] = vertex[0] - 15.7 / 2.0; // Center plane
        vertex.0.scale(150.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut line_draw_order = vec![];

    let mut index_start = 0;
    // line_draw_order.append(&mut vec![
    //     vec![0, 1],
    //     vec![2, 3],
    // ]);

    // let mut exhaust = exhaust::get_line_draw_order(index_start);
    // index_start += exhaust::get_vertices().len();
    // line_draw_order.append(&mut exhaust);

    // let mut fuselage = fuselage::get_line_draw_order(index_start);
    // index_start += fuselage::get_vertices().len();
    // line_draw_order.append(&mut fuselage);

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
