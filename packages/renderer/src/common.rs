/// Common traits and structures.
use linear_algebra::matrix::{MatrixDataTrait, Matrix};

pub trait VertexTrait {
    type T: MatrixDataTrait;

    fn new(data: [Self::T; 3]) -> Self;

    fn x(&self) -> &Self::T;
    fn y(&self) -> &Self::T;
    fn z(&self) -> &Self::T;

    fn x_mut(&mut self) -> &mut Self::T;
    fn y_mut(&mut self) -> &mut Self::T;
    fn z_mut(&mut self) -> &mut Self::T;
}

impl<T: MatrixDataTrait> VertexTrait for Matrix<T> {
    type T = T;

    fn new(data: [Self::T; 3]) -> Self {
        Matrix::from_array([data; 1])
    }

    fn x(&self) -> &Self::T {
        self.index(0, 0)
    }

    fn y(&self) -> &Self::T {
        self.index(0, 1)
    }

    fn z(&self) -> &Self::T {
        self.index(0, 2)
    }

    fn x_mut(&mut self) -> &mut Self::T {
        self.index_mut(0, 0)
    }

    fn y_mut(&mut self) -> &mut Self::T {
        self.index_mut(0, 1)
    }

    fn z_mut(&mut self) -> &mut Self::T {
        self.index_mut(0, 2)
    }
}

pub trait PlaneTrait<T> {
    fn point(&self) -> &[T];
    fn parameter_a(&self) -> &[T];
    fn parameter_b(&self) -> &[T];
}

/// TODO: A bit ugly, because it is dependent on how matrix is implemented.
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
