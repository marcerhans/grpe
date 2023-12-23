use std::ops::{Deref, IndexMut};

use linear_algebra::matrix::*;
use terminal::*;

trait Drawable {
    type Output;
    type Config;
    fn draw(&self, config: &Self::Config) -> Self::Output;
}

impl Drawable for Matrix<usize> {
    type Output = ();
    type Config = (usize, usize);

    fn draw(&self, config: &Self::Config) -> Self::Output {
        let dims = self.dimensions();
        let mut frame: Vec<char> = vec![' '; config.0 * config.1];

        for row in self.inner() {
            frame[row[0] + row[1] * config.0] = '#';
        }

        for (index, c) in frame.iter().rev().enumerate() {
            print!("{}", c);

            if index % dims.0 == 0 {
                println!();
            }
        }
    }
}

fn main() {
    // Set up cube
    let cube = Matrix::new(vec![
        vec![0, 0],
        vec![4, 0],
        vec![0, 4],
    ]);

    // Draw cube
    cube.unwrap().draw(&(10, 10));
}
