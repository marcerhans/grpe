use std::env;

use linear_algebra::{matrix,utility};

// use linear_algebra::matrix2;

/// # TODO:
/// ## Needed types.
/// - Objects (matrices)
/// - Viewport (2D-matrix to project objects on)
/// - Draw-handler/trait or whatever. Some common interface for showing the viewport. In this case, a terminal (text representation).
///
/// No "World" matrix should exist. That would only set unnecessary limits. Objects can exist on their own.

// struct ViewPort {
//     resolution: (usize, usize),
// }

// use linear_algebra::matrix::*;
// use terminal::*;

// // trait Drawable {
// //     type Output;

// //     fn draw(&self, config: &Self::) -> Self::Output;
// // }

// // impl Drawable for Matrix<usize> {
// //     type Output = Canvas;

// //     fn draw(&self, canvas: &Self::Canvas) -> Self::Output {
// //     }
// // }

// struct Canvas {
//     buffer: Vec<char>,
//     width: usize,
//     height: usize,
// }

// impl Canvas {
//     fn new(width: usize, height: usize) -> Self {
//         Self {
//             buffer: vec![' '; width * height],
//             width: width,
//             height: height,
//         }
//     }

//     fn draw(&self) {
//         for (index, c) in self.buffer.iter().enumerate() {
//             let index_column = index % self.width;
//             let index_row = index / self.width;
//             let index_row_max = self.buffer.len() / self.width;

//             if index_column == 0 {
//                 println!();
//             }

//             if index_column == self.width / 2 {
//                 print!("|");
//             } else if index_row_max / 2 == index_row {
//                 print!("-");
//             } else {
//                 print!("{c}");
//             }
//         }
//     }
// }

// mod vector {
//     use super::*;

//     pub struct Vector {
//         inner: Matrix<usize>,
//     }

//     impl Vector {
//         fn new(vec: Vec<usize>) -> Self {
//             Self {
//                 inner: Matrix::new(vec![vec; 1]).unwrap(),
//             }
//         }
//     }
// }

// struct Plane {
//     normal: Matrix<usize>,
// }

// impl Plane {
//     fn new(normal: Vec<usize>) -> Self {
//         Self {
//             normal: Matrix::new_vector(normal).unwrap(),
//         }
//     }
// }

// mod what {
//     struct Line {
//         origin: [isize; 3],
//         vectors: [isize; 3],
//     }

//     struct Plane {
//         origin: [isize; 3],
//         vectors: [[isize; 3]; 2],
//     }

//     impl Plane {
//         /// Set the Plane and the Line to equal eachother, and solve
//         /// the resulting equation system.
//         /// 
//         /// |plane_x_0|          |x|          |x|   |line_x_0|         |x|
//         /// |plane_y_0| + plane_t|y| + plane_s|y| = |line_y_0| + line_t|y|
//         /// |plane_z_0|          |z|          |z|   |line_z_0|         |z|
//         fn intersects_with_line(&self, line: Line) -> Option<[isize; 3]> {
//             let plane_origin = self.origin;
//             let plane_t_vec = self.vectors[0];
//             let plane_s_vec = self.vectors[1];

//             let line_origin = line.origin;
//             let line_t_vec = line.vectors;

//             let origin = [
//                 line_origin[0] - plane_origin[0],
//                 line_origin[1] - plane_origin[1],
//                 line_origin[2] - plane_origin[2],
//             ];

//             let t_vec = [
//                 plane_t_vec[0] - line_t_vec[0],
//                 plane_t_vec[1] - line_t_vec[1],
//                 plane_t_vec[2] - line_t_vec[2],
//             ];

//             let s_vec = [
//                 plane_s_vec[0],
//                 plane_s_vec[1],
//                 plane_s_vec[2],
//             ];

//             // We now have the following system that has to be solved:
//             //
//             //        |x|          |x|         |x|   |x_sum0|    
//             // plane_t|y| + plane_s|y| - line_t|y| = |y_sum1|
//             //        |z|          |z|         |z|   |z_sum2|    
//             Plane::gauss_elimination();

//             None
//         }

//         /// Returns None if a unique solution could be found.
//         fn gauss_elimination() -> Option<[isize;3]> {
//             // Forward

//             // Reverse
//             todo!()
//         }
//     }

//     #[cfg(test)]
//     mod tests {
//         use super::*;

//         #[test]
//         fn main() {
//             let P = Plane {
//                 origin: [0, 0, 4],
//                 vectors: [[1, 0, 0], [0, 1, 0]],
//             };

//             let L = Line {
//                 origin: [2, 0, 0],
//                 vectors: [[2, 0, 4]],
//             };
//         }
//     }
// }

trait Canvas {
    fn draw();
}

trait Drawable {
    fn draw();
}

struct Cube {
    matrix: matrix::Matrix,
}

impl Drawable for Cube {
    fn draw() {
        todo!()
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let option = args.get(1);

    // println!("{}", option.unwrap());

    // let cube = Cube {
    //     matrix: matrix::macros::matrix![

    //     ]
    // }; 

    let camera = matrix::macros::matrix![
        [0.0,   0.0,    4.0],   // Point
    ];

    let canvas = matrix::macros::matrix![
        [0.0,   0.0,    2.0],   // Point
        [1.0,   0.0,    0.0],   // Vector (t1)
        [0.0,   1.0,    0.0],   // Vector (t2)
    ];
    let canvas = canvas.transpose();

    let cube_points = matrix::macros::matrix![
        [0.0,   0.0,    0.0],   // Point 1
        [4.0,   0.0,    0.0],   // Point 2
        [4.0,   4.0,    0.0],   // Point 3
        [0.0,   4.0,    0.0],   // Point 4
    ];
    
    let cube_direction_vectors = matrix::Matrix::from_row_matrices(&[
        &camera.slice(0..1,0..3) - &cube_points.slice(0..1,0..3), // Vector 1
        &camera.slice(0..1,0..3) - &cube_points.slice(1..2,0..3), // Vector 2
        &camera.slice(0..1,0..3) - &cube_points.slice(2..3,0..3), // Vector 3
        &camera.slice(0..1,0..3) - &cube_points.slice(3..4,0..3), // Vector 4
    ]);

    let cube_camera_line_1 = matrix::Matrix::from_row_matrices(&[
        cube_points.slice(0..1,0..3),
        cube_direction_vectors.slice(0..1, 0..3),
    ]);
    let cube_camera_line_1 = cube_camera_line_1.transpose();

    let mut eq_system = matrix::Matrix::from_column_matrices(&[
        canvas.slice(0..3, 1..2),
        canvas.slice(0..3, 2..3),
        -&cube_camera_line_1.slice(0..3, 1..2),
        &cube_camera_line_1.slice(0..3, 0..1) - &canvas.slice(0..3, 0..1),
    ]);


    utility::gauss_elimination(&mut eq_system);
    println!("{:?}", eq_system);

    // Set up cube
    // let cube = Matrix::new(vec![
    //     vec![0, 0],
    //     vec![4, 0],
    //     vec![0, 4],
    // ]);

    // // Draw cube
    // cube.unwrap().draw(&(16,9));

    // let rows = 10;
    // let cols = 10;

    // let canvas = Canvas::new(cols, rows);

    // let terminal = Terminal::new(16, 9);

    // terminal.transform_buffer(canva)

    // canvas.draw();
    // println!();

    // Cube described in matrix.

    // Project cube onto 2d-plane (screen)

    // Adjust positions in order to fit in terminal

    // Display in terminal
    // let plane = Plane::new(vec![0, 0, 1]);
}
