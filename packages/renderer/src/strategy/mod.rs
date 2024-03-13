// TODO: These should be guarded with a feature flag or similar. Currently only terminal is implemented.
pub mod terminal;
pub use terminal as renderer;

/// Common traits and structures.
pub mod common {
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
    pub struct Vertex<'a, Data: DataTrait>(Matrix<Data>, PhantomData<&'a Data>);
    pub type Camera<'a, Data: DataTrait> = Vertex<'a, Data>;

    impl<'a, Data: DataTrait> VertexTrait<'a> for Vertex<'a, Data> {
        type Data = Data;
        type DataRef = &'a Data;

        fn new(data: [Self::Data; 3]) -> Self {
            Self(Matrix::from_array([data; 1]), PhantomData)
        }

        fn x(&'a self) -> Self::DataRef {
            self.0.index(0, 0)
        }

        fn y(&'a self) -> Self::DataRef {
            self.0.index(0, 1)
        }

        fn z(&'a self) -> Self::DataRef {
            self.0.index(0, 2)
        }

        fn slice(&'a self) -> &[Self::Data] {
            &self.0.data()
        }
    }

    pub trait SurfaceTrait {}

    pub struct Surface<'a, T: DataTrait>(Matrix<T>, PhantomData<&'a T>);
    pub type Canvas<'a, T: DataTrait> = Surface<'a, T>;

    impl<'a, T: DataTrait> SurfaceTrait for Surface<'a, T> {}

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
}
