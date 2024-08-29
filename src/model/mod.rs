use std::{f64::consts, str::FromStr};

use renderer::VectorRow;

mod plane;
mod spiral;

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
                vertices.append(&mut spiral::get_vertices());
            }
            Model::Test => {
                vertices.push(VectorRow::from([0.0, 0.0, 0.0]));
                vertices.push(VectorRow::from([0.0, 0.0, 1.0]));
                vertices.push(VectorRow::from([0.0, 0.0, -1.0]));
                vertices.push(VectorRow::from([1.0, 0.0, 0.0]));
                vertices.push(VectorRow::from([-1.0, 0.0, 0.0]));
                vertices.push(VectorRow::from([0.0, 1.0, 0.0]));
                vertices.push(VectorRow::from([0.0, -1.0, 0.0]));

                // vertices.push(VectorRow::from([7.0, 0.0, 4.0]));

                for vertex in vertices.iter_mut() {
                    vertex.0.scale(10.0);
                }
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
                lines.append(&mut spiral::get_line_draw_order());
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
