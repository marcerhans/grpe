pub mod terminal;
pub use terminal as renderer;

/// Common structures and traits.
pub mod common {
    use std::marker::PhantomData;

    use linear_algebra::matrix::{DataTrait, Matrix};

    use crate::{DimensionsTrait, VertexTrait};

    impl DimensionsTrait for (usize, usize) {
        fn width(&self) -> usize {
            self.0
        }

        fn height(&self) -> usize {
            self.1
        }
    }

    pub struct Vertex<'a, Data: DataTrait>(Matrix<Data>, PhantomData<&'a Data>);

    impl<'a, Data: DataTrait> VertexTrait<'a> for Vertex<'a, Data> {
        type Data = Data;
        type DataRef = &'a Data;
        
        fn new(data: [Self::Data; 3]) -> Self {
            Self (
                Matrix::from_array([data; 1]),
                PhantomData,
            )
        }

        fn x(&self) -> Self::DataRef {
            self.0.index(0, 0)
        }

        fn y(&self) -> Self::DataRef {
            self.0.index(0, 1)
        }

        fn z(&self) -> Self::DataRef {
            self.0.index(0, 2)
        }

        fn slice(&self) -> &[Self::Data] {
            &self.0.data()
        }
    }
}
