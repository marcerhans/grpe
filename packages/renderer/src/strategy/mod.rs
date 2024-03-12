pub mod terminal;
pub use terminal as renderer;

/// Common structures and traits.
pub mod common {
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

    pub struct Vertex<Data>(Matrix<Data>);

    impl<Data: DataTrait> VertexTrait for Vertex<Data> {
        type Output = Data;

        fn x(&self) -> Self::Output {
            self.0.index(0,0)
        }

        fn y(&self) -> Self::Output {
            self.0.index(0,1)
        }

        fn z(&self) -> Self::Output {
            self.0.index(0,2)
        }
    }
}
