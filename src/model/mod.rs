use std::{f64::consts, str::FromStr};

use renderer::VectorRow;

mod plane;

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
                vertices.append(&mut plane::get_vertices());
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
                vertices.push(VectorRow::from([40.0, 0.0, 0.0]));
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
                lines.append(&mut plane::get_line_draw_order());
            }
            Model::Spiral => {
                let mut line = vec![];
                for i in 0..1000 {
                    line.push(i);
                }
                lines.push(line);
            }
            Model::Test => {
                lines.push(vec![0,1]);
                // lines.push(vec![0,2]);
                // lines.push(vec![0,3]);
                // lines.push(vec![0,4]);
            }
        }

        lines
    }
}
