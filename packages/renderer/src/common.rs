/// Common traits and structures.
use linear_algebra::matrix::{MatrixDataTrait, Matrix};

pub trait VertexTrait {
    type T: MatrixDataTrait;

    fn new(data: [Self::T; 3]) -> Self;

    fn x(&self) -> &Self::T;
    fn y(&self) -> &Self::T;
    fn z(&self) -> &Self::T;
    fn matrix(&self) -> &Matrix<Self::T>;

    fn x_mut(&mut self) -> &mut Self::T;
    fn y_mut(&mut self) -> &mut Self::T;
    fn z_mut(&mut self) -> &mut Self::T;
    fn matrix_mut(&mut self) -> &mut Matrix<Self::T>;
}

impl<T: MatrixDataTrait> VertexTrait for Matrix<T> {
    type T = T;

    fn new(data: [Self::T; 3]) -> Self {
        Self(Matrix::from_array([data; 1]))
    }

    fn x(&self) -> &Self::T {
        self.0.index(0, 0)
    }

    fn y(&self) -> &Self::T {
        self.0.index(0, 1)
    }

    fn z(&self) -> &Self::T {
        self.0.index(0, 2)
    }

    fn matrix(&self) -> &Self::T {
        &self.0
    }

    fn x_mut(&mut self) -> &mut Self::T {
        self.0.index_mut(0, 0)
    }

    fn y_mut(&mut self) -> &mut Self::T {
        self.0.index_mut(0, 1)
    }

    fn z_mut(&mut self) -> &mut Self::T {
        self.0.index_mut(0, 2)
    }

    fn matrix_mut(&mut self) -> &mut Matrix<T> {
        &mut self.0
    }
}

pub trait PlaneTrait<T> {
    fn point(&self) -> &[T];
    fn parameter_a(&self) -> &[T];
    fn parameter_b(&self) -> &[T];
}

impl<T: MatrixDataTrait> PlaneTrait<T> for Matrix<T> {
    fn point(&self) -> &[T] {
        self.row(0)
    }

    fn parameter_a(&self) -> &[T] {
        self.row(1)
    }

    fn parameter_b(&self) -> &[T] {
        self.row(2)
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

pub struct Camera<T: MatrixDataTrait> {
    resolution: (usize, usize),
    position: (T, T, T),
    direction: (T, T, T),
}

impl<T: MatrixDataTrait> Camera<T> {
    fn new(resolution: (usize, usize), position: (T, T, T), direction: (T, T, T)) -> Self {
        Self {
            resolution,
            position,
            direction,
        }
    }
}

impl<T: MatrixDataTrait> Default for Camera<T> {
    fn default() -> Self {
        Self {
            resolution: (100, 100),
            position: (0, 0, -10),
            direction: (0, 0, 1),
        }
    }
}
