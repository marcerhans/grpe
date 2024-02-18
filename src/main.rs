use std::env;

use linear_algebra::{matrix::{self, macros::matrix, Matrix},utility};
use terminal::Terminal;

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
    matrix: Matrix,
}

impl Drawable for Cube {
    fn draw() {
        todo!()
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let terminal_width = usize::from_str_radix(args.get(1).unwrap(), 10);
    // let terminal_height = usize::from_str_radix(args.get(2).unwrap(), 10);
    // let terminal = Terminal::new(terminal_width.unwrap(), terminal_height.unwrap());

    let terminal_width = 8;
    let terminal_height = 8;
    let terminal = Terminal::new(terminal_width, terminal_height);

    let camera = matrix![
        [0.0,   0.0,    -4.0],   // Point
    ];

    let canvas = matrix![
        [0.0,   0.0,    -2.0],   // Point
        [1.0,   0.0,    0.0],   // Vector (t1)
        [0.0,   1.0,    0.0],   // Vector (t2)
    ];
    let canvas = canvas.transpose();

    let mut cube_points = matrix![
        // [0.0,   0.0,    0.0],   // Point 1
        // [4.0,   0.0,    0.0],   // Point 2
        // [4.0,   4.0,    0.0],   // Point 3
        // [0.0,   4.0,    0.0],   // Point 4
        [1.0,   1.0,    0.0],   // Point 1
        // [2.0,   2.0,    0.0],   // Point 1
        // [3.0,   3.0,    0.0],   // Point 1
        // [4.0,   4.0,    0.0],   // Point 1
        // [5.0,   5.0,    0.0],   // Point 1
        // [6.0,   6.0,    0.0],   // Point 2
        // [7.0,   7.0,    0.0],   // Point 3
        // [8.0,   8.0,    0.0],   // Point 4
    ];

    // loop {
        // Rasterize
        let mut points_to_draw: Matrix = Matrix::zeros(0, 0);

        for index in 0..cube_points.rows() {
            let cube_point = cube_points.row(index);
            let mut cube_vector = &camera.row(0) - &cube_point;

            let cube_camera_line = Matrix::from_row_matrices(&[
                &cube_point,
                &cube_vector,
            ]);
            let cube_camera_line = cube_camera_line.transpose();

            let mut eq_system = Matrix::from_column_matrices(&[
                &canvas.column(1),
                &canvas.column(2),
                &-&cube_camera_line.column(1),
                &(&cube_camera_line.column(0) - &canvas.column(0)),
            ]);

            utility::gauss_elimination(&mut eq_system);

            let cube_point_scalar = eq_system[(2,3)];
            cube_vector.scalar(cube_point_scalar);
            let point_on_canvas = &cube_point + &cube_vector;
            points_to_draw.push_row(point_on_canvas);
        }

        log::info!("{:?}", points_to_draw);

        // Draw
        terminal.draw(&points_to_draw);

        // Update position(s) of points
        cube_points[(0,0)] += 1.0;

        std::thread::sleep(std::time::Duration::from_millis(100));
    // }

    // let degree: f64 = 1.0;
    // let rotation_matrix = matrix![
    //     [1.0,   0.0,            0.0],
    //     [0.0,   degree.cos(),   -degree.sin()],
    //     [0.0,   degree.sin(),   degree.sin()],
    // ];



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
