/// Common traits and structures.
use linear_algebra::matrix::{MatrixDataTrait, Matrix};

pub type Vertex<T> = Matrix<T>;

pub trait Point<T: MatrixDataTrait> {
    fn new(data: [T; 3]) -> Self;

    fn x(&self) -> &T;
    fn y(&self) -> &T;
    fn z(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
    fn z_mut(&mut self) -> &mut T;
}

impl<T: MatrixDataTrait> Point<T> for Matrix<T> {
    fn new(data: [T; 3]) -> Self {
        Matrix::from_array([data; 1])
    }

    fn x(&self) -> &T {
        self.index(0, 0)
    }

    fn y(&self) -> &T {
        self.index(0, 1)
    }

    fn z(&self) -> &T {
        self.index(0, 2)
    }

    fn x_mut(&mut self) -> &mut T {
        self.index_mut(0, 0)
    }

    fn y_mut(&mut self) -> &mut T {
        self.index_mut(0, 1)
    }

    fn z_mut(&mut self) -> &mut T {
        self.index_mut(0, 2)
    }
}

pub trait PlaneTrait<T> {
    fn point(&self) -> &[T];
    fn parameter_a(&self) -> &[T];
    fn parameter_b(&self) -> &[T];
}

impl<T: MatrixDataTrait> PlaneTrait<T> for Matrix<T> {
    fn point(&self) -> &[T] {
        &self.data()[0..3]
    }

    fn parameter_a(&self) -> &[T] {
        &self.data()[3..6]
    }

    fn parameter_b(&self) -> &[T] {
        &self.data()[6..9]
    }
}

pub trait DimensionsTrait {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl DimensionsTrait for (usize, usize) {
    fn width(&self) -> usize {
        self.0
    }

    fn height(&self) -> usize {
        self.1
    }
}
