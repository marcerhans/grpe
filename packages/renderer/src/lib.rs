use linear_algebra::matrix::Matrix;

pub mod strategy;
pub use strategy::renderer;

pub trait VertexTrait {
    fn position(&self) -> &Matrix;
}

pub trait CameraTrait: VertexTrait {}

pub trait CanvasTrait : VertexTrait {
    fn parameters(&self) -> (&Matrix, &Matrix);
}

pub trait DimensionsTrait {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
}

/// [RendererBuilderTrait] are categorized settings and initial values for a renderer ([RendererTrait]).
pub trait RendererBuilderTrait {
    type Dimensions: DimensionsTrait;
    type Camera: CameraTrait;
    type Canvas: CanvasTrait;

    fn new() -> Self;
    fn dimensions(self, dimensions: Self::Dimensions) -> Self;
    fn camera(self, camera: Self::Camera) -> Self;
    fn canvas(self, canvas: Self::Canvas) -> Self;
    fn build(self) -> renderer::Renderer;
}

/// [RendererTrait] for rendering to display.
pub trait RendererTrait {
    type Vertex: VertexTrait;

    /// Project vertices on to [CanvasTrait].
    fn project(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]);

    /// Rasterize [RendererTrait::project]ed vertices.
    fn rasterize(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]);

    /// Do all steps needed, in correct order, to produce a fully rendered image.
    fn run_pipeline(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) {
        self.project(vertices);
        self.rasterize(vertices);
    }
}