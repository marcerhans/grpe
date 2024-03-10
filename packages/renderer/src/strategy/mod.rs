pub mod terminal;
pub use terminal as renderer;

/// Common structures and traits.
pub mod common {
    use crate::{DimensionsTrait, VertexTrait};

    impl DimensionsTrait for (usize, usize) {
        fn width(&self) -> usize {
            self.0
        }

        fn height(&self) -> usize {
            self.1
        }
    }

    pub struct Vertex(f64, f64, f64);

    impl VertexTrait for Vertex {
        type Output = f64;

        fn x(&self) -> Self::Output {
            self.0
        }

        fn y(&self) -> Self::Output {
            self.1
        }

        fn z(&self) -> Self::Output {
            self.2
        }
    }

    /// Temporary
    struct Matrix<Data> {
        _phantom_data: std::marker::PhantomData<Data>,
    }

    impl Into<Matrix<Vertex>> for &[(Vertex, Vertex, Vertex)] {
        fn into(self) -> Matrix<Vertex> {
            todo!()
        }
    }
}
