/// Common traits and structures.
use linear_algebra::{matrix::{Matrix, MatrixDataTrait}, utility::gauss_elimination};

// pub trait PointTrait<T: MatrixDataTrait> {
//     fn new(data: [T; 3]) -> Self;

//     fn x(&self) -> &T;
//     fn y(&self) -> &T;
//     fn z(&self) -> &T;

//     fn x_mut(&mut self) -> &mut T;
//     fn y_mut(&mut self) -> &mut T;
//     fn z_mut(&mut self) -> &mut T;
// }

// impl<T: MatrixDataTrait> PointTrait<T> for Matrix<T> {
//     fn new(data: [T; 3]) -> Self {
//         Matrix::from_array([data; 1])
//     }

//     fn x(&self) -> &T {
//         self.index(0, 0)
//     }

//     fn y(&self) -> &T {
//         self.index(0, 1)
//     }

//     fn z(&self) -> &T {
//         self.index(0, 2)
//     }

//     fn x_mut(&mut self) -> &mut T {
//         self.index_mut(0, 0)
//     }

//     fn y_mut(&mut self) -> &mut T {
//         self.index_mut(0, 1)
//     }

//     fn z_mut(&mut self) -> &mut T {
//         self.index_mut(0, 2)
//     }
// }

// pub trait PlaneTrait<T> {
//     fn point(&self) -> &[T];
//     fn parameter_a(&self) -> &[T];
//     fn parameter_b(&self) -> &[T];
// }

// impl<T: MatrixDataTrait> PlaneTrait<T> for Matrix<T> {
//     fn point(&self) -> &[T] {
//         &self.data()[0..3]
//     }

//     fn parameter_a(&self) -> &[T] {
//         &self.data()[3..6]
//     }

//     fn parameter_b(&self) -> &[T] {
//         &self.data()[6..9]
//     }
// }

pub trait DimensionsTrait<T> {
    fn width(&self) -> T;
    fn height(&self) -> T;
}

impl DimensionsTrait<u64> for (u64, u64) {
    fn width(&self) -> u64 {
        self.0
    }

    fn height(&self) -> u64 {
        self.1
    }
}