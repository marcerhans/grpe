/// Modelled with image of brazillian plane. Top down image. FOV: 15.
use std::f64::consts;

use renderer::VectorRow;

const LENGTH: f64 = 15.2;
const WING_SPAN: f64 = 8.6;
const HEIGHT: f64 = 4.5;

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

mod body {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = vec![];

        // Exhaust.
        let radius_small = 0.3;
        let radius_large = 0.45;
        let points = 10;
        for point in 1..=points {
            let radians = (std::f64::consts::PI * 2.0 / (points as f64)) * (point as f64);
            vertices.push(VectorRow::from([
                0.0,
                radius_small * radians.cos(),
                radius_small * radians.sin(),
            ]));
        }

        for point in 1..=points {
            let radians = (std::f64::consts::PI * 2.0 / (points as f64)) * (point as f64);
            vertices.push(VectorRow::from([
                0.6,
                radius_large * radians.cos(),
                radius_large * radians.sin(),
            ]));
        }

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        // Exhaust.
        let points = 10;
        for point in 0..(points - 1) {
            line_draw_order.append(&mut vec![vec![
                start + point + 1,
                start + points + point + 1,
                start + points + point,
                start + point,
            ]]);
        }
        line_draw_order.append(&mut vec![vec![
            start + 0,
            start + points + 0,
            start + points + (points - 1),
            start + (points - 1),
        ]]);

        for point in 0..(points - 1) {
            line_draw_order.append(&mut vec![vec![
                start + point,
                start + points + point,
                start + points + point + 1,
                start + point + 1,
            ]]);
        }
        line_draw_order.append(&mut vec![vec![
            start + (points - 1),
            start + points + (points - 1),
            start + points + 0,
            start + 0,
        ]]);

        // line_draw_order = add_lines(start + 23, line_draw_order);

        line_draw_order
    }
}

mod wings {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = vec![];

        // Main part.
        vertices.append(&mut vec![
            VectorRow::from([3.82, 1.01, -0.05]), // 0
            VectorRow::from([3.82, 1.01, 0.05]),  // 1
            VectorRow::from([5.5, 0.98, -0.05]),  // 2
            VectorRow::from([5.5, 0.98, 0.05]),   // 3
            VectorRow::from([7.0, 0.98, -0.05]),  // 4
            VectorRow::from([7.0, 0.98, 0.05]),   // 5
            VectorRow::from([8.9, 0.98, 0.0]),    // 6
            VectorRow::from([8.08, 1.33, 0.0]),   // 7
            VectorRow::from([6.8, 2.25, 0.0]),    // 8
            VectorRow::from([6.33, 2.25, -0.05]), // 9
            VectorRow::from([6.33, 2.25, 0.05]),  // 10
            VectorRow::from([4.22, 4.25, -0.05]), // 11
            VectorRow::from([4.22, 4.25, 0.05]),  // 12
            VectorRow::from([4.5, 4.25, 0.0]),    // 13
            VectorRow::from([4.36, 4.35, 0.0]),   // 14
            VectorRow::from([3.55, 4.35, 0.0]),   // 15
            VectorRow::from([3.64, 2.8, -0.05]),  // 16
            VectorRow::from([3.64, 2.8, 0.05]),   // 17
            VectorRow::from([3.72, 2.8, -0.05]),  // 18
            VectorRow::from([3.72, 2.8, 0.05]),   // 19
        ]);

        // Thick inner flap.
        vertices.append(&mut vec![
            VectorRow::from([2.68, 1.05, 0.0]),  // 0
            VectorRow::from([3.8, 1.01, -0.05]), // 1
            VectorRow::from([3.8, 1.01, 0.05]),  // 2
            VectorRow::from([3.7, 2.75, -0.05]), // 3
            VectorRow::from([3.7, 2.75, 0.05]),  // 4
            VectorRow::from([3.12, 2.75, 0.0]),  // 5
            VectorRow::from([3.07, 1.5, 0.0]),   // 6
        ]);

        // Thin outer flap.
        vertices.append(&mut vec![
            VectorRow::from([3.12, 2.78, 0.0]),   // 0
            VectorRow::from([3.62, 2.8, -0.05]),  // 1
            VectorRow::from([3.62, 2.8, 0.05]),   // 2
            VectorRow::from([3.53, 4.34, -0.05]), // 3
            VectorRow::from([3.53, 4.34, 0.05]),  // 4
            VectorRow::from([3.23, 4.34, 0.05]),  // 5
        ]);

        // Inner front flap.
        vertices.append(&mut vec![
            VectorRow::from([6.78, 2.27, 0.0]),   // 0
            VectorRow::from([6.35, 2.27, -0.05]), // 1
            VectorRow::from([6.35, 2.27, 0.05]),  // 2
            VectorRow::from([5.34, 3.21, -0.05]), // 3
            VectorRow::from([5.34, 3.21, 0.05]),  // 4
            VectorRow::from([5.61, 3.21, 0.0]),   // 5
        ]);

        // Outer front flap.
        vertices.append(&mut vec![
            VectorRow::from([5.85, 3.24, 0.0]),   // 0
            VectorRow::from([5.34, 3.24, -0.05]), // 1
            VectorRow::from([5.34, 3.24, 0.05]),  // 2
            VectorRow::from([4.24, 4.23, -0.05]), // 3
            VectorRow::from([4.24, 4.23, 0.05]),  // 4
            VectorRow::from([4.5, 4.23, 0.00]),   // 5
        ]);

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(mut start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        // Main part.
        line_draw_order.append(&mut vec![
            vec![
                start + 18,
                start + 16,
                start + 15,
                start + 14,
                start + 13,
                start + 11,
                start + 9,
                start + 8,
                start + 7,
                start + 6,
                start + 4,
                start + 2,
                start + 0,
            ],
            vec![
                start + 1,
                start + 3,
                start + 5,
                start + 6,
                start + 7,
                start + 8,
                start + 10,
                start + 12,
                start + 13,
                start + 14,
                start + 15,
                start + 17,
                start + 19,
            ],
            vec![start + 8, start + 9, start + 10],
            // vec![start + 9, start + 11, start + 12, start + 10], // Causing odd clipping?
            vec![start + 11, start + 13, start + 12],
        ]);
        start += 20;

        // Thick inner flap.
        line_draw_order.append(&mut vec![
            vec![start + 0, start + 2, start + 4, start + 5, start + 6],
            vec![start + 6, start + 5, start + 3, start + 1, start + 0],
            vec![start + 0, start + 1, start + 2],
            vec![start + 1, start + 3, start + 4, start + 2],
            vec![start + 3, start + 5, start + 4],
        ]);
        start += 7;

        // Thin outer flap.
        line_draw_order.append(&mut vec![
            vec![start + 0, start + 2, start + 4, start + 5],
            vec![start + 5, start + 3, start + 1, start + 0],
            vec![start + 0, start + 1, start + 2],
            vec![start + 1, start + 3, start + 4, start + 2],
            vec![start + 3, start + 5, start + 4],
        ]);
        start += 6;

        // Inner front flap.
        line_draw_order.append(&mut vec![
            vec![start + 5, start + 4, start + 2, start + 0],
            vec![start + 0, start + 1, start + 3, start + 5],
            vec![start + 2, start + 1, start + 0],
            vec![start + 2, start + 4, start + 3, start + 1],
            vec![start + 4, start + 5, start + 3],
        ]);
        start += 6;

        // Outer front flap.
        line_draw_order.append(&mut vec![
            vec![start + 5, start + 4, start + 2, start + 0],
            vec![start + 0, start + 1, start + 3, start + 5],
            vec![start + 2, start + 1, start + 0],
            vec![start + 2, start + 4, start + 3, start + 1],
            vec![start + 4, start + 5, start + 3],
        ]);

        // Mirror for right wing.
        let mut line_draw_order_mirrored = line_draw_order.clone();
        for order in &mut line_draw_order_mirrored {
            for ele in order.iter_mut() {
                *ele += get_vertices().len() / 2;
            }
            order.reverse();
        }
        line_draw_order.append(&mut line_draw_order_mirrored);

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

    vertices.append(&mut body::get_vertices());
    // vertices.append(&mut fuselage::get_vertices());
    // vertices.append(&mut rudder::get_vertices());
    vertices.append(&mut wings::get_vertices());
    // vertices.append(&mut canards::get_vertices());
    // vertices.append(&mut intake::get_vertices());

    // vertices.append(&mut cockpit::get_vertices());

    // Backdrop
    // const GRID_SIZE: i32 = 200;
    // const GRID_SPACING: i32 = 10;
    // for i in 0..GRID_SIZE {
    //     for j in 0..GRID_SIZE {
    //         vertices.push(VectorRow::from([
    //             (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (i * GRID_SPACING) as f64,
    //             (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (j * GRID_SPACING) as f64,
    //             -10 as f64,
    //         ]));
    //     }
    // }

    // Scale and center
    for vertex in vertices.iter_mut() {
        vertex[0] = vertex[0] - LENGTH / 2.0; // Center plane
        vertex[1] -= 0.08;
        vertex.0.scale(32.6);
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
    // index_start += 4;

    let mut body = body::get_line_draw_order(index_start);
    index_start += body::get_vertices().len();
    line_draw_order.append(&mut body);

    // let mut fuselage = fuselage::get_line_draw_order(index_start);
    // index_start += fuselage::get_vertices().len();
    // line_draw_order.append(&mut fuselage);

    // let mut rudder = rudder::get_line_draw_order(index_start);
    // index_start += rudder::get_vertices().len();
    // line_draw_order.append(&mut rudder);

    let mut wings = wings::get_line_draw_order(index_start);
    index_start += wings::get_vertices().len();
    line_draw_order.append(&mut wings);

    // let mut canards = canards::get_line_draw_order(index_start);
    // index_start += canards::get_vertices().len();
    // line_draw_order.append(&mut canards);

    // let mut intake = intake::get_line_draw_order(index_start);
    // index_start += intake::get_vertices().len();
    // line_draw_order.append(&mut intake);

    // for i in index_start..(index_start + 200*200) {
    //     line_draw_order.push(vec![i]);
    // }

    line_draw_order
}
