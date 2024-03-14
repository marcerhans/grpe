/// Common traits and structures.
use std::marker::PhantomData;

use linear_algebra::matrix::{DataTrait, Matrix};

pub trait VertexTrait<'a> {
    type Data;
    type DataRef;

    fn new(data: [Self::Data; 3]) -> Self;
    fn x(&'a self) -> Self::DataRef;
    fn y(&'a self) -> Self::DataRef;
    fn z(&'a self) -> Self::DataRef;
    fn slice(&'a self) -> &[Self::Data];
}

#[derive(Default, Clone)]
pub struct Vertex<'a, Data: DataTrait>(PhantomData<&'a Data>, Matrix<Data>);

impl<'a, Data: DataTrait> VertexTrait<'a> for Vertex<'a, Data> {
    type Data = Data;
    type DataRef = &'a Data;

    fn new(data: [Self::Data; 3]) -> Self {
        Self(PhantomData, Matrix::from_array([data; 1]))
    }

    fn x(&'a self) -> Self::DataRef {
        self.1.index(0, 0)
    }

    fn y(&'a self) -> Self::DataRef {
        self.1.index(0, 1)
    }

    fn z(&'a self) -> Self::DataRef {
        self.1.index(0, 2)
    }

    fn slice(&'a self) -> &[Self::Data] {
        &self.1.data()
    }
}

pub trait SurfaceTrait {}

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
