use std::str::FromStr;

use renderer::VectorRow;

mod cube;
mod plane;
mod spiral;
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
            Model::Star => {
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

// mod object {
//     use renderer::VectorRow;

//     pub enum Axis {
//         X,
//         Y,
//         Z,
//     }

//     impl Axis {
//         fn index(&self) -> usize {
//             match self {
//                 Axis::X => 0,
//                 Axis::Y => 1,
//                 Axis::Z => 2,
//             }
//         }
//     }

//     pub struct Object {
//         index_start: usize,
//         index_end: usize,
//         vertices: Vec<VectorRow<f64, 3>>,
//     }

//     impl Object {
//         pub fn new(start_index: usize) -> Self {
//             Self {
//                 index_start: start_index,
//                 index_end: start_index,
//                 vertices: vec![],
//             }
//         }

//         // pub fn with_vertices(start_index: usize, vertices: Vec<VectorRow<f64, 3>>) -> Self {
//         //     Self {
//         //         index_start: start_index,
//         //         index_end: start_index + vertices.len(),
//         //         vertices,
//         //     }
//         // }

//         // pub fn add_vertices(&mut self, vertices: &mut Vec<VectorRow<f64, 3>>) {
//         //     self.vertices.append(vertices);
//         //     self.index_end += self.vertices.len();
//         // }

//         // pub fn combine(&mut self, mut other: Object) {
//         //     for (index, vertex) in other.iter_mut() {
//         //         if let Some(pos) = self.vertices.iter().position(|x| *x == *vertex) {
//         //             index = pos;
//         //         }
//         //     }
//         // }

//         pub fn iter(&self) -> ObjectIter {
//             ObjectIter::new(self)
//         }

//         pub fn mirror(mut self, axis: Axis) -> Self {
//             let mirror_index = axis.index();

//             for vertex in self.vertices.iter_mut() {
//                 vertex[mirror_index] = vertex[mirror_index] * -1.0;
//             }

//             self
//         }
//     }

//     pub struct ObjectIter<'a> {
//         index: usize,
//         index_vec: usize,
//         object: &'a Object,
//     }

//     impl<'a> ObjectIter<'a> {
//         fn new(object: &'a Object) -> Self {
//             Self {
//                 index: object.index_start,
//                 index_vec: 0,
//                 object,
//             }
//         }
//     }

//     impl<'a> Iterator for ObjectIter<'a> {
//         type Item = (usize, &'a VectorRow<f64, 3>);

//         fn next(&mut self) -> Option<Self::Item> {
//             self.index += 1;

//             if self.index == self.object.index_end {
//                 None
//             } else {
//                 Some((self.index, &self.object.vertices[self.index_vec]))
//             }
//         }
//     }
// }
