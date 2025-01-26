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
            VectorRow::from([2.48, 0.98, 0.0]),// 12
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
            VectorRow::from([8.5, 0.0, 0.6]),  // 21
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

        // Intake.
        vertices.append(&mut vec![
            // Just duplicated from body...
            VectorRow::from([8.8, 0.0, 0.6]),  // 0 (21)
            VectorRow::from([8.8, 0.28, 0.55]),// 1 (22)
            VectorRow::from([8.8, 0.4, 0.5]),  // 2 (23)
            VectorRow::from([8.8, 0.6, 0.4]),  // 3 (24)
            VectorRow::from([8.8, 1.0, 0.25]), // 4 (25)
            VectorRow::from([8.8, 1.0, 0.0]),  // 5 (26)
            VectorRow::from([8.8, 0.28, -0.55]),// 6 (22--)
            VectorRow::from([8.8, 0.6, -0.4]), // 7 (24--)
            VectorRow::from([8.8, 1.0, -0.25]),// 8 (25--)
            VectorRow::from([8.8, 0.0, -0.6]), // 9 (21--)
            //
            VectorRow::from([10.0, 0.5, 0.4]), // 10
            VectorRow::from([11.0, 0.6, 0.35]), // 11
            VectorRow::from([11.0, 0.65, 0.3]), // 12
            VectorRow::from([10.8, 1.0, 0.3]), // 13
            VectorRow::from([10.6, 1.1, 0.3]), // 14
            VectorRow::from([10.4, 1.05, 0.3]), // 15
            //
            VectorRow::from([10.6, 1.0, -0.3]), // 16
            //
            VectorRow::from([10.5, 0.5, -0.3]), // 17
            //
            VectorRow::from([11.1, 0.65, 0.0]), // 18
            VectorRow::from([10.9, 0.65, -0.3]), // 19
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
                start + 10,
                start + 11,
                start + 12,
                start + 13,
                start + 14,
                start + 15,
                start + 4,
            ],
            vec![
                start + 4,
                start + 15,
                start + 14,
                start + 13,
                start + 16,
                start + 8,
            ],
            vec![
                start + 8,
                start + 16,
                start + 19,
                start + 17,
                start + 6,
            ],
            vec![
                start + 13,
                start + 12,
                start + 18,
                start + 19,
                start + 16,
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

mod cockpit {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = vec![];

        // Cockpit.
        let mut misc = vec![
            // Pitot
            VectorRow::from([15.2, 0.0, -0.2]), // 0
            VectorRow::from([16.0, 0.0, -0.2]), // 1
            // Just duplicated from body...
            VectorRow::from([8.5, 0.0, 0.6]),  // 2/14 (21)
            VectorRow::from([8.8, 0.28, 0.55]),// 3 (22)
            VectorRow::from([8.8, 0.4, 0.5]),  // 4 (23)
            VectorRow::from([8.8, 0.6, 0.4]),  // 5 (24)
            VectorRow::from([8.8, 1.0, 0.25]), // 6 (25)
            VectorRow::from([8.8, 1.0, 0.0]),  // 7 (26)
            VectorRow::from([8.8, 0.28, -0.55]),// 8 (22--)
            VectorRow::from([8.8, 0.6, -0.4]), // 9 (24--)
            VectorRow::from([8.8, 1.0, -0.25]),// 10 (25--)
            VectorRow::from([8.5, 0.0, -0.6]), // 11 (21--)
        ];
        // 12 to 23
        misc.append(&mut mirror_y(&misc));

        // Just duplicated from intake...
        let mut duplicated_from_intake = vec![
            VectorRow::from([10.0, 0.5, 0.4]), // 24
            VectorRow::from([11.0, 0.6, 0.35]), // 25
            VectorRow::from([11.0, 0.65, 0.3]), // 26
            VectorRow::from([10.8, 1.0, 0.3]), // 27
            VectorRow::from([10.6, 1.1, 0.3]), // 28
            VectorRow::from([10.4, 1.05, 0.3]), // 29
            //
            VectorRow::from([10.6, 1.0, -0.3]), // 30
            //
            VectorRow::from([10.5, 0.5, -0.3]), // 31
            //
            VectorRow::from([11.1, 0.65, 0.0]), // 32
            VectorRow::from([10.9, 0.65, -0.3]), // 33
        ];
        // 34 to 43
        duplicated_from_intake.append(&mut mirror_y(&duplicated_from_intake));

        let mut gps = vec![
            VectorRow::from([8.8, 0.0, 0.8]), // 44
            VectorRow::from([10.0, 0.0, 0.8]), // 45
            VectorRow::from([10.0, 0.2, 0.76]), // 46
            VectorRow::from([10.0, 0.3, 0.66]), // 47
            VectorRow::from([10.0, 0.35, 0.4]), // 48
            VectorRow::from([8.8, 0.14, 0.76]), // 49
        ];
        // 50 to 55
        gps.append(&mut mirror_y(&gps));
        gps.append(&mut vec![
            VectorRow::from([9.1, 0.0, 0.81]), // 56
            VectorRow::from([9.4, 0.0, 0.81]), // 57
            VectorRow::from([9.7, 0.0, 0.81]), // 58
        ]);

        let mut cone = vec![
            VectorRow::from([11.0, 0.5, 0.32]), // 59
            VectorRow::from([11.0, -0.5, 0.32]), // 60
            VectorRow::from([10.0, 0.35, 0.35]), // 61
            VectorRow::from([10.0, -0.35, 0.35]), // 62
            // Windshield base
            VectorRow::from([11.5, -0.5, 0.27]), // 63
            VectorRow::from([12.0, -0.4, 0.25]), // 64
            VectorRow::from([12.5, -0.25, 0.22]), // 65
            VectorRow::from([12.8, 0.0, 0.2]), // 66
            VectorRow::from([12.5, 0.25, 0.22]), // 67
            VectorRow::from([12.0, 0.4, 0.25]), // 68
            VectorRow::from([11.5, 0.5, 0.27]), // 69
        ];

        let radius_small = 0.5;
        let points = 10;
        for point in 1..=points {
            let radians = (std::f64::consts::PI * 2.0 / (points as f64)) * (point as f64);
            cone.push(VectorRow::from([
                11.5,
                radius_small * radians.cos(),
                0.375 + radius_small * radians.sin(),
            ]));
        }
        cone[11+8][0] += 0.0;
        cone[11+9][0] += 0.1;
        cone[11+0][0] += 0.15;
        cone[11+1][0] += 0.19;
        cone[11+2][0] += 0.19;
        cone[11+3][0] += 0.15;
        cone[11+4][0] += 0.1;
        cone[11+5][0] += 0.0;

        cone.append(&mut vec![
            // Line on windshield.
            VectorRow::from([10.0, 0.0, 0.8]),  // 80
            VectorRow::from([11.3, 0.0, 0.94]), // 81
            VectorRow::from([11.6, 0.0, 0.88]), // 82
            VectorRow::from([11.8, 0.0, 0.77]), // 83
            VectorRow::from([12.0, 0.0, 0.66]), // 84
            VectorRow::from([12.2, 0.0, 0.55]), // 85
            VectorRow::from([12.4, 0.0, 0.44]), // 86
            VectorRow::from([12.8, 0.0, 0.22]), // 87
        ]);

        cone.append(&mut vec![
            // TOP
            VectorRow::from([13.5, 0.35, 0.09]), // 88
            VectorRow::from([14.0, 0.3, 0.0]), // 89
            VectorRow::from([13.5, -0.35, 0.09]), // 90
            VectorRow::from([14.0, -0.3, 0.0]), // 91
            // SIDES
            VectorRow::from([14.0, 0.4, -0.2]), // 92
            VectorRow::from([13.5, 0.5, -0.2]), // 93
            VectorRow::from([12.0, 0.65, -0.2]), // 94
            VectorRow::from([14.0, -0.4, -0.2]), // 95
            VectorRow::from([13.5, -0.5, -0.2]), // 96
            VectorRow::from([12.0, -0.65, -0.2]), // 97
            // BOTTOM
            VectorRow::from([14.0, 0.0, -0.45]), // 98
            VectorRow::from([13.5, 0.0, -0.5]), // 99
            VectorRow::from([12.0, 0.0, -0.55]), // 100
            VectorRow::from([10.0, 0.0, -0.6]), // 101
        ]);

        // Duplicate and mirror.
        // vertices.append(&mut mirror_y(&vertices));

        vertices.append(&mut misc);
        vertices.append(&mut duplicated_from_intake);
        vertices.append(&mut gps);
        vertices.append(&mut cone);

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        line_draw_order.append(&mut vec![
            vec![
                start + 0,
                start + 1,
            ],
            vec![
                start + 61,
                start + 59,
                start + 25,
                start + 24,
                start + 3,
                start + 2,
                start + 15,
                start + 34,
                start + 35,
                start + 60,
                start + 62,
            ],
            // gps
            vec![
                start + 3,
                start + 15,
                start + 55,
                start + 44,
                start + 49,
            ],
            vec![
                start + 48,
                start + 47,
                start + 46,
                start + 45,
                start + 52,
                start + 53,
                start + 54,
            ],
            vec![
                start + 46,
                start + 49,
                start + 44,
                start + 55,
                start + 52,
                start + 51,
            ],
            vec![
                start + 56,
            ],
            vec![
                start + 57,
            ],
            vec![
                start + 58,
            ],
            vec![
                start + 49,
                start + 46,
                start + 47,
                start + 48,
                start + 3,
            ],
            vec![
                start + 15,
                start + 54,
                start + 53,
                start + 52,
                start + 55,
            ],
            // Windshield base
            vec![
                start + 62,
                start + 60,
                start + 63,
                start + 64,
                start + 65,
                start + 66,
                start + 67,
                start + 68,
                start + 69,
                start + 59,
                start + 61,
            ],
            // bar
            vec![
                start + 70,
                start + 71,
            ],
            vec![
                start + 71,
                start + 72,
            ],
            vec![
                start + 72,
                start + 73,
            ],
            vec![
                start + 73,
                start + 74,
            ],
            vec![
                start + 74,
                start + 75,
            ],
            vec![
                start + 70,
                start + 79,
            ],
            vec![
                start + 79,
                start + 78,
            ],
            // Line
            vec![
                start + 80,
                start + 81,
            ],
            vec![
                start + 81,
                start + 82,
            ],
            vec![
                start + 82,
                start + 83,
            ],
            vec![
                start + 83,
                start + 84,
            ],
            vec![
                start + 84,
                start + 85,
            ],
            vec![
                start + 85,
                start + 86,
            ],
            vec![
                start + 86,
                start + 87,
            ],
            vec![
                start + 41,
                start + 20,
                start + 11,
                start + 8,
                start + 31,
            ],
        ]);

        let mut nose = vec![
            // TOP
            vec![
                start + 89,
                start + 88,
                start + 69,
                start + 68,
                start + 67,
                start + 66,
                start + 65,
                start + 64,
                start + 63,
                start + 90,
                start + 91,
                start + 0,
            ],
            // SIDES
            vec![
                start + 0,
                start + 92,
                start + 93,
                start + 94,
                start + 32,
                start + 26,
                start + 25,
                start + 59,
                start + 69,
                start + 88,
                start + 89,
            ],
            vec![
                start + 91,
                start + 90,
                start + 63,
                start + 60,
                start + 35,
                start + 36,
                start + 42,
                start + 97,
                start + 96,
                start + 95,
                start + 0,
            ],
            // BOTTOM
            vec![
                start + 0,
                start + 98,
                start + 99,
                start + 100,
                start + 101,
                start + 11,
            ],
        ];

        // Duplicate and mirror.
        // let mut line_draw_order_mirrored = line_draw_order.clone().to_vec();
        // for order in &mut line_draw_order_mirrored {
        //     for ele in order.iter_mut() {
        //         *ele += get_vertices().len() / 2;
        //     }
        //     order.reverse();
        // }
        // line_draw_order.append(&mut line_draw_order_mirrored);

        line_draw_order.append(&mut nose);

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
            VectorRow::from([2.8, -0.1, 2.0]),  // 1
            VectorRow::from([2.8, -0.1, 2.2]),  // 2
            VectorRow::from([1.1, -0.1, 2.2]),  // 3
            VectorRow::from([1.1, 0.1, 2.0]),   // 4
            VectorRow::from([2.8, 0.1, 2.0]),   // 5
            VectorRow::from([2.8, 0.1, 2.2]),   // 6
            VectorRow::from([1.1, 0.1, 2.2]),   // 7
            VectorRow::from([2.95, 0.00, 2.1]), // 8
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
    vertices.append(&mut cockpit::get_vertices());
    vertices.append(&mut exhaust::get_vertices());
    vertices.append(&mut rudder::get_vertices());
    vertices.append(&mut wings::get_vertices());
    vertices.append(&mut canards::get_vertices());

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

    let mut cockpit= cockpit::get_line_draw_order(index_start);
    index_start += cockpit::get_vertices().len();
    line_draw_order.append(&mut cockpit);

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
