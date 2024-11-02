use std::{f64::consts, str::FromStr};

use renderer::VectorRow;

mod plane;
mod spiral;
mod cube;
mod star;
mod test;

pub enum Model {
    Plane,
    Spiral,
    Cube,
    Star,
    Test,
}

impl FromStr for Model {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plane" => Ok(Model::Plane),
            "spiral" => Ok(Model::Spiral),
            "cube" => Ok(Model::Cube),
            "star" => Ok(Model::Star),
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
            Model::Cube => {
                vertices.append(&mut cube::get_vertices());
            }
            Model::Star=> {
                vertices.append(&mut star::get_vertices());
            }
            Model::Test => {
                vertices.append(&mut test::get_vertices());
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
            Model::Cube => {
                lines.append(&mut cube::get_line_draw_order());
            }
            Model::Star => {
                lines.append(&mut star::get_line_draw_order());
            }
            Model::Test => {
                lines.append(&mut test::get_line_draw_order());
            }
        }

        lines
    }
}
