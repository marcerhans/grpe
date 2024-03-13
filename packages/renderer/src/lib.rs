/// TODO: Currently (mostly) focuses on f64, and not general data types.

pub mod strategy;
pub use strategy::renderer;

use common::*;

#[derive(Default)]
pub struct RendererConfiguration<'a> {
    dimensions: (usize, usize),
    camera: Vertex<'a, f64>,
    canvas: Surface<'a, f64>,
}

/// [RendererBuilderTrait] are categorized settings and initial values for a renderer ([RendererTrait]).
pub trait RendererBuilderTrait<'a> {
    type Dimensions: DimensionsTrait;
    type Camera: VertexTrait<'a>;
    type Canvas: SurfaceTrait;
    type Renderer: RendererTrait<'a>;

    /// In order to instantiate this type, since the implementation may vary for different renderers,
    /// the implementation should provide a 'man' (manual/info) string for what info is needed to 
    /// initialize it.
    fn man() -> &'static str;

    /// Create new instance of the [RendererBuilderTrait].
    fn new() -> Self;

    /// Create new instance of the [RendererBuilderTrait] with default values for each parameter.
    fn default() -> Self;

    fn with_dimensions(self, dimensions: Self::Dimensions) -> Self;
    fn with_camera(self, camera: Self::Camera) -> Self;
    fn with_canvas(self, canvas: Self::Canvas) -> Self;

    /// Build an instance of [RendererTrait].
    fn build(self) -> Self::Renderer;

    /// Build an instance of [RendererTrait] using an existing [RendererConfiguration].
    fn build_with_config(self, config: RendererConfiguration) -> Self::Renderer;
}

/// [RendererTrait] for rendering to display on some [SurfaceTrait].
pub trait RendererTrait<'a> {
    type Vertex: VertexTrait<'a>;

    /// Get [RendererConfiguration].
    fn config(&self) -> RendererConfiguration;

    /// Set a new config ([RendererConfiguration]) for the [RendererTrait].
    /// Useful if the dimensions of the [Canvas] changes in size, for example.
    /// Returns [Result::Ok] if configuration is valid for current renderer.
    fn set_config(&self) -> Result<(), ()>;

    /// Project vertices on to a [SurfaceTrait].
    fn project(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) -> &dyn SurfaceTrait;

    /// Rasterize [RendererTrait::project]ed vertices.
    fn rasterize(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) -> &dyn SurfaceTrait;

    /// Do all steps needed, in correct order, to produce a fully rendered image on some [SurfaceTrait].
    fn run_pipeline(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) {
        self.project(vertices);
        self.rasterize(vertices);
    }
}

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

    #[derive(Default)]
    pub struct Vertex<'a, Data: DataTrait>(Matrix<Data>, PhantomData<&'a Data>);

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

    #[derive(Default)]
    pub struct Surface<'a, Data: DataTrait>(Matrix<Data>, PhantomData<&'a Data>);

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
}