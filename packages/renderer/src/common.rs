/// Common traits and structures.
use linear_algebra::matrix::{MatrixDataTrait, Matrix};

pub trait VertexTrait {
    type Data: MatrixDataTrait;

    fn new(data: [Self::Data; 3]) -> Self;

    fn x(&self) -> &Self::Data;
    fn y(&self) -> &Self::Data;
    fn z(&self) -> &Self::Data;
    fn matrix(&self) -> &Matrix<Self::Data>;

    fn x_mut(&mut self) -> &mut Self::Data;
    fn y_mut(&mut self) -> &mut Self::Data;
    fn z_mut(&mut self) -> &mut Self::Data;
    fn matrix_mut(&mut self) -> &mut Matrix<Self::Data>;
}

// #[derive(Debug, Default, Clone)]
// pub struct Vertex<Data: DataTrait>(Matrix<Data>);

// impl<Data: DataTrait> VertexTrait for Vertex<Data> {
impl<Data: MatrixDataTrait> VertexTrait for Matrix<Data> {
    type Data = Data;

    fn new(data: [Self::Data; 3]) -> Self {
        Self(Matrix::from_array([data; 1]))
    }

    fn x(&self) -> &Self::Data {
        self.0.index(0, 0)
    }

    fn y(&self) -> &Self::Data {
        self.0.index(0, 1)
    }

    fn z(&self) -> &Self::Data {
        self.0.index(0, 2)
    }

    fn matrix(&self) -> &Self::Data {
        &self.0
    }

    fn x_mut(&mut self) -> &mut Self::Data {
        self.0.index_mut(0, 0)
    }

    fn y_mut(&mut self) -> &mut Self::Data {
        self.0.index_mut(0, 1)
    }

    fn z_mut(&mut self) -> &mut Self::Data {
        self.0.index_mut(0, 2)
    }

    fn matrix_mut(&mut self) -> &mut Matrix<Data> {
        &mut self.0
    }
}

// impl<Data: DataTrait> Into<Matrix<Data>> for Vertex<Data> {
//     fn into(self) -> Matrix<Data> {
//         Matrix::from_array([[*self.x(), *self.y(), *self.z()]])
//     }
// }

pub trait PlaneTrait<Data> {
    fn point(&self) -> &[Data];
    fn parameter_a(&self) -> &[Data];
    fn parameter_b(&self) -> &[Data];
}

impl<Data: MatrixDataTrait> PlaneTrait<Data> for Matrix<Data> {
    fn point(&self) -> &[Data] {
        self.row(0)
    }

    fn parameter_a(&self) -> &[Data] {
        self.row(1)
    }

    fn parameter_b(&self) -> &[Data] {
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

pub struct Camera<Data: MatrixDataTrait> {
    resolution: (usize, usize),
    position: (Data, Data, Data),
    direction: (Data, Data, Data),
}

impl<Data: MatrixDataTrait> Camera<Data> {
    fn new(resolution: (usize, usize), position: (Data, Data, Data), direction: (Data, Data, Data)) -> Self {
        Self {
            resolution,
            position,
            direction,
        }
    }
}

impl<Data: MatrixDataTrait> Default for Camera<Data> {
    fn default() -> Self {
        Self {
            resolution: (100, 100),
            position: (0, 0, -10),
            direction: (0, 0, 1),
        }
    }
}
