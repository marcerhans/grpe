/// Common traits and structures.
use std::marker::PhantomData;

use linear_algebra::matrix::{DataTrait, Matrix};

pub trait VertexTrait {
    type Data;

    fn new(data: [Self::Data; 3]) -> Self;

    fn x(&self) -> &Self::Data;
    fn y(&self) -> &Self::Data;
    fn z(&self) -> &Self::Data;

    fn x_mut(&mut self) -> &mut Self::Data;
    fn y_mut(&mut self) -> &mut Self::Data;
    fn z_mut(&mut self) -> &mut Self::Data;
}

#[derive(Debug, Default, Clone)]
pub struct Vertex<Data: DataTrait>(Matrix<Data>);

impl<Data: DataTrait> VertexTrait for Vertex<Data> {
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

    fn x_mut(&mut self) -> &mut Self::Data {
        self.0.index_mut(0, 0)
    }

    fn y_mut(&mut self) -> &mut Self::Data {
        self.0.index_mut(0, 1)
    }

    fn z_mut(&mut self) -> &mut Self::Data {
        self.0.index_mut(0, 2)
    }
}

impl<Data: DataTrait> Into<Matrix<Data>> for Vertex<Data> {
    fn into(self) -> Matrix<Data> {
        Matrix::from_array([
            [*self.x(), *self.y(), *self.z()]
        ])
    }
}

pub trait SurfaceTrait {}

impl<Data: DataTrait> SurfaceTrait for Matrix<Data> {}

#[derive(Default, Clone)]
pub struct Surface<'a, Data: DataTrait>(PhantomData<&'a Data>, Matrix<Data>);

impl<'a, Data: DataTrait> SurfaceTrait for Surface<'a, Data> {}

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
