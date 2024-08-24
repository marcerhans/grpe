use std::{f64::consts, str::FromStr};

use renderer::VectorRow;

pub enum Model {
    Plane,
    Spiral,
    Test,
}

impl FromStr for Model {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plane" => Ok(Model::Plane),
            "spiral" => Ok(Model::Spiral),
            "test" => Ok(Model::Test),
            _ => Err("Could not convert to string."),
        }
    }
}

impl Model {
    pub fn get_vertices(&self) -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        match self {
            Model::Plane => {
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
            }
            Model::Spiral => {
                // Spiral zooming in.
                const MAX_DEPTH: i32 = 1000;
                for i in 0..MAX_DEPTH {
                    vertices.push(VectorRow::from([
                        i as f64 * (((i as f64) / 16.0) % (consts::PI * 2.0)).cos(),
                        i as f64,
                        i as f64 * (((i as f64) / 16.0) % (consts::PI * 2.0)).sin(),
                    ]));
                }

                const GRID_SIZE: i32 = 200;
                const GRID_SPACING: i32 = 100;
                for i in 0..GRID_SIZE {
                    for j in 0..GRID_SIZE {
                        vertices.push(VectorRow::from([
                            (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (i * GRID_SPACING) as f64,
                            MAX_DEPTH as f64,
                            (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (j * GRID_SPACING) as f64,
                        ]));
                    }
                }
            }
            Model::Test => {
                vertices.push(VectorRow::from([0.0, 0.0, 0.0]));
                // vertices.push(VectorRow::from([10.0, 0.0, 0.0]));
                // vertices.push(VectorRow::from([0.0, 0.0, 1.0]));
                // vertices.push(VectorRow::from([0.0, 0.0, -1.0]));
                // vertices.push(VectorRow::from([1.0, 0.0, 0.0]));
                // vertices.push(VectorRow::from([-1.0, 0.0, 0.0]));

                // vertices.push(VectorRow::from([7.0, 0.0, 4.0]));

                // for vertex in vertices.iter_mut() {
                //     vertex.0.scale(10.0);
                // }
            }
        }

        vertices
    }

    pub fn get_line_draw_order(&self) -> Vec<Vec<usize>> {
        let mut lines = Vec::new();

        match self {
            Model::Plane => {
                lines.push(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]);
                lines.push(vec![27, 28, 29, 30]);
            }
            Model::Spiral => {
                let mut line = vec![];
                for i in 0..1000 {
                    line.push(i);
                }
                lines.push(line);
            }
            Model::Test => {
                // lines.push(vec![0,1]);
                // lines.push(vec![0,2]);
                // lines.push(vec![0,3]);
                // lines.push(vec![0,4]);
            }
        }

        lines
    }
}
