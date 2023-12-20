use linear_algebra::matrix::*;
use terminal::*;

trait Drawable {
    type Output;

    fn draw(&self) -> Self::Output;
}

fn main() {
    // Set up cube
    let _cube = Matrix::new(vec![
        vec![0, 0],
        vec![1, 0],
        vec![1, 1],
        vec![0, 1],
    ]);

    // Draw cube
}
