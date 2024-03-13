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
        type Output = Data;
        type OutputRef = &'a Data;

        fn x(&self) -> Self::OutputRef {
            self.0.index(0, 0)
        }

        fn y(&self) -> Self::OutputRef {
            self.0.index(0, 1)
        }

        fn z(&self) -> Self::OutputRef {
            self.0.index(0, 2)
        }

        fn slice(&self) -> &[Self::Output] {
            &self.0.data()
        }
    }
}
