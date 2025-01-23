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

        // Body.
        vertices.append(&mut vec![
            VectorRow::from([0.6, 0.0, 0.45]), // 0
            VectorRow::from([2.0, 0.0, 0.6]),  // 1
            VectorRow::from([2.0, 0.4, 0.5]),  // 2
            VectorRow::from([2.0, 0.6, 0.3]),  // 3
            VectorRow::from([2.0, 0.75, 0.0]), // 4
            VectorRow::from([0.6, 0.45, 0.0]), // 5
            VectorRow::from([0.6, 0.4, 0.3]),  // 6
            VectorRow::from([0.6, 0.2, 0.4]),  // 7
            //
            VectorRow::from([4.0, 0.0, 0.6]),  // 8
            VectorRow::from([4.0, 0.4, 0.5]),  // 9
            VectorRow::from([4.0, 0.65, 0.25]),// 10
            VectorRow::from([4.0, 1.0, 0.15]), // 11
            VectorRow::from([2.5, 0.95, 0.0]), // 12
            //
            VectorRow::from([6.0, 0.0, 0.6]),  // 13
            VectorRow::from([6.0, 0.4, 0.5]),  // 14
            VectorRow::from([6.0, 0.65, 0.25]),// 15
            VectorRow::from([6.0, 1.0, 0.15]), // 16
            //
            VectorRow::from([8.0, 0.0, 0.6]), // 17
            VectorRow::from([8.0, 0.4, 0.5]), // 18
            VectorRow::from([8.0, 0.6, 0.35]),// 19
            VectorRow::from([8.0, 1.0, 0.1]), // 20
            //
            VectorRow::from([8.8, 0.0, 0.6]),  // 21
            VectorRow::from([8.8, 0.28, 0.55]),// 22
            VectorRow::from([8.8, 0.4, 0.5]),  // 23
            VectorRow::from([8.8, 0.6, 0.4]),  // 24
            VectorRow::from([8.8, 1.0, 0.25]), // 25
            VectorRow::from([8.8, 1.0, 0.0]),  // 26
        ]);

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));
        vertices.append(&mut mirror_z(&vertices));

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        line_draw_order.append(&mut vec![
            vec![
                start + 0,
                start + 1,
                start + 2,
                start + 3,
                start + 6,
                start + 7,
            ],
            vec![
                start + 6,
                start + 3,
                start + 4,
                start + 5,
            ],
            vec![
                start + 1,
                start + 8,
                start + 9,
                start + 10,
                start + 3,
                start + 2,
            ],
            vec![
                start + 3,
                start + 10,
                start + 11,
                start + 12,
                start + 4,
            ],
            vec![
                start + 8,
                start + 13,
                start + 14,
                start + 15,
                start + 10,
                start + 9,
            ],
            vec![
                start + 10,
                start + 15,
                start + 16,
                start + 11,
            ],
            vec![
                start + 13,
                start + 17,
                start + 18,
                start + 19,
                start + 15,
                start + 14,
            ],
            vec![
                start + 15,
                start + 19,
                start + 20,
                start + 16,
            ],
            vec![
                start + 17,
                start + 21,
                start + 22,
                start + 23,
                start + 24,
                start + 19,
                start + 18,
            ],
            vec![
                start + 19,
                start + 24,
                start + 25,
                start + 20,
            ],
            vec![
                start + 20,
                start + 25,
                start + 26,
            ],
        ]);

        // Duplicate and mirror.
        let mut line_draw_order_mirrored = line_draw_order.clone().to_vec();
        for order in &mut line_draw_order_mirrored {
            for ele in order.iter_mut() {
                *ele += get_vertices().len() / 4;
            }
            order.reverse();
        }
        line_draw_order.append(&mut line_draw_order_mirrored);

        let mut line_draw_order_mirrored = line_draw_order.clone().to_vec();
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

mod intake {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = vec![];

            // Canard
            // VectorRow::from([8.68, 1.0, 0.35]),  // 0
            // VectorRow::from([10.41, 1.1, 0.04]), // 1
            // VectorRow::from([8.41, 2.345, 0.45]),// 3
            // VectorRow::from([7.93, 2.345, 0.5]), // 4
            // VectorRow::from([8.53, 1.25, 0.4]),  // 5
            // VectorRow::from([8.53, 1.1, 0.35]),  // 6

        // Intake.
        vertices.append(&mut vec![
            // Just duplicated from body...
            VectorRow::from([8.8, 0.0, 0.6]),  // 0 (21)
            VectorRow::from([8.8, 0.28, 0.55]),// 1 (22)
            VectorRow::from([8.8, 0.4, 0.5]),  // 2 (23)
            VectorRow::from([8.8, 0.6, 0.4]),  // 3 (24)
            VectorRow::from([8.8, 1.0, 0.25]), // 4 (25)
            VectorRow::from([8.8, 1.0, 0.0]),  // 5 (26)
            VectorRow::from([8.8, 1.0, -0.25]),// 6 (25++)
            //
            VectorRow::from([10.0, 0.31, 0.4]), // 7
            VectorRow::from([11.0, 0.35, 0.3]), // 8
            VectorRow::from([10.75, 1.0, 0.3]), // 9
            //
            VectorRow::from([10.5, 1.0, -0.3]), // 10
            // VectorRow::from([11.0, 0.7, 0.4]), // 2
        ]);

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        line_draw_order.append(&mut vec![
            vec![
                start + 1,
                start + 7,
                start + 8,
                start + 9,
                start + 4,
            ],
            vec![
                start + 4,
                start + 9,
                start + 10,
                start + 6,
            ]
        ]);

        // Duplicate and mirror.
        let mut line_draw_order_mirrored = line_draw_order.clone().to_vec();
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

mod exhaust {
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

        vertices
    }

    pub fn get_line_draw_order(mut start: usize) -> Vec<Vec<usize>> {
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

        line_draw_order
    }
}

mod wings {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = vec![];

        // Main part.
        vertices.append(&mut vec![
            VectorRow::from([3.82, 1.01, -0.1]),  // 0
            VectorRow::from([3.82, 1.01, 0.1]),   // 1
            VectorRow::from([5.5, 0.98, -0.15]),  // 2
            VectorRow::from([5.5, 0.98, 0.15]),   // 3
            VectorRow::from([7.0, 0.98, -0.1]),   // 4
            VectorRow::from([7.0, 0.98, 0.1]),    // 5
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
            VectorRow::from([2.68, 1.05, 0.0]), // 0
            VectorRow::from([3.8, 1.01, -0.1]), // 1
            VectorRow::from([3.8, 1.01, 0.1]),  // 2
            VectorRow::from([3.7, 2.75, -0.1]), // 3
            VectorRow::from([3.7, 2.75, 0.1]),  // 4
            VectorRow::from([3.12, 2.75, 0.0]), // 5
            VectorRow::from([3.07, 1.5, 0.0]),  // 6
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

mod canards {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = vec![];

        // Main part.
        vertices.append(&mut vec![
            VectorRow::from([8.68, 1.0, 0.35]),  // 0
            VectorRow::from([10.41, 1.1, 0.04]), // 1
            VectorRow::from([8.41, 2.345, 0.45]),// 3
            VectorRow::from([7.93, 2.345, 0.5]), // 4
            VectorRow::from([8.53, 1.25, 0.4]),  // 5
            VectorRow::from([8.53, 1.1, 0.35]),  // 6
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
                start + 0,
                start + 1,
                start + 2,
                start + 3,
                start + 4,
                start + 5,
            ],
            vec![
                start + 5,
                start + 5,
                start + 3,
                start + 2,
                start + 1,
                start + 0,
            ],
        ]);

        // Mirror for right canard.
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

mod rudder {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = vec![];

        // Main part.
        vertices.append(&mut vec![
            VectorRow::from([1.0, 0.0, 0.5]),  // 0
            VectorRow::from([4.0, 0.0, 0.6]),  // 1
            VectorRow::from([1.8, 0.0, 2.5]),  // 2
            VectorRow::from([1.2, 0.0, 2.5]),  // 3
            VectorRow::from([1.2, 0.0, 2.0]),  // 4
            VectorRow::from([1.18, 0.0, 1.8]), // 5
            VectorRow::from([1.55, 0.0, 1.8]), // 6
            VectorRow::from([1.6, 0.0, 0.8]),  // 7
            VectorRow::from([1.1, 0.0, 0.8]),  // 8
        ]);

        // Rudder
        vertices.append(&mut vec![
            VectorRow::from([1.1, 0.0, 0.8]),  // 0
            VectorRow::from([1.6, 0.0, 0.8]),  // 1
            VectorRow::from([1.55, 0.0, 1.8]), // 2
            VectorRow::from([1.18, 0.0, 1.8]), // 3
        ]);

        // Pitot tube
        vertices.append(&mut vec![
            VectorRow::from([3.2, 0.01, 1.2]), // 0
            VectorRow::from([4.0, 0.01, 1.2]), // 1
        ]);

        // EW
        vertices.append(&mut vec![
            VectorRow::from([1.1, -0.1, 2.0]),  // 0
            VectorRow::from([3.0, -0.1, 2.0]),  // 1
            VectorRow::from([3.0, -0.1, 2.2]),  // 2
            VectorRow::from([1.1, -0.1, 2.2]),  // 3
            VectorRow::from([1.1, 0.1, 2.0]),   // 4
            VectorRow::from([3.0, 0.1, 2.0]),   // 5
            VectorRow::from([3.0, 0.1, 2.2]),   // 6
            VectorRow::from([1.1, 0.1, 2.2]),   // 7
            VectorRow::from([3.15, 0.00, 2.1]), // 8
            VectorRow::from([0.95, 0.00, 2.1]), // 9
        ]);

        // Duplicate and mirror.
        vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(mut start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        // Main part.
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
        ]]);
        start += 9;

        // Rudder
        line_draw_order.append(&mut vec![vec![start + 0, start + 1, start + 2, start + 3]]);
        start += 4;

        // Pitot tube
        line_draw_order.append(&mut vec![vec![start + 0, start + 1]]);
        start += 2;

        // Mirror.
        let mut line_draw_order_mirrored = line_draw_order.clone();
        for order in &mut line_draw_order_mirrored {
            for ele in order.iter_mut() {
                *ele += get_vertices().len() / 2;
            }
            order.reverse();
        }
        line_draw_order.append(&mut line_draw_order_mirrored);

        // EW
        line_draw_order.append(&mut vec![
            vec![start + 0, start + 1, start + 2, start + 3],
            vec![start + 7, start + 6, start + 5, start + 4],
            vec![start + 4, start + 5, start + 1, start + 0],
            vec![start + 3, start + 2, start + 6, start + 7],
            vec![start + 1, start + 8, start + 2],
            vec![start + 2, start + 8, start + 6],
            vec![start + 6, start + 8, start + 5],
            vec![start + 5, start + 8, start + 1],
            vec![start + 3, start + 9, start + 0],
            vec![start + 0, start + 9, start + 4],
            vec![start + 4, start + 9, start + 7],
            vec![start + 7, start + 9, start + 3],
        ]);

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
    vertices.append(&mut intake::get_vertices());
    vertices.append(&mut exhaust::get_vertices());
    vertices.append(&mut rudder::get_vertices());
    vertices.append(&mut wings::get_vertices());
    vertices.append(&mut canards::get_vertices());

    // vertices.append(&mut cockpit::get_vertices());

    // // Backdrop
    // const GRID_SIZE: i32 = 200;
    // const GRID_SPACING: i32 = 2;
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

    let mut intake = intake::get_line_draw_order(index_start);
    index_start += intake::get_vertices().len();
    line_draw_order.append(&mut intake);

    let mut exhaust = exhaust::get_line_draw_order(index_start);
    index_start += exhaust::get_vertices().len();
    line_draw_order.append(&mut exhaust);

    let mut rudder = rudder::get_line_draw_order(index_start);
    index_start += rudder::get_vertices().len();
    line_draw_order.append(&mut rudder);

    let mut wings = wings::get_line_draw_order(index_start);
    index_start += wings::get_vertices().len();
    line_draw_order.append(&mut wings);

    let mut canards = canards::get_line_draw_order(index_start);
    index_start += canards::get_vertices().len();
    line_draw_order.append(&mut canards);

    // // Backdrop
    // for i in index_start..(index_start + 200*200) {
    //     line_draw_order.push(vec![i]);
    // }

    line_draw_order
}
