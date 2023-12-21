use std::ops::{Deref, IndexMut};

use linear_algebra::matrix::*;
use terminal::*;

trait Drawable {
    type Output;
    fn draw(&self) -> Self::Output;
}

impl Drawable for Matrix<usize> {
    type Output = ();

    fn draw(&self) -> Self::Output {
        let mut frame = Vec::<char>::with_capacity(self.);
        // frame.chars()[100] = "a";

        // for row in self.inner().iter() {
        //     frame
        //     for column_item in row.iter() {

        //     }
        // }
    }
}

fn main() {
    // Set up cube
    let cube = Matrix::new(vec![
        vec![0, 0],
        vec![4, 0],
        vec![4, 4],
        vec![0, 4],
    ]);

    // Draw cube
    cube.unwrap().draw();
}
