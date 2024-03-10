pub mod strategy;
pub use strategy::renderer;

pub trait VertexTrait {
    type Output;

    fn x(&self) -> Self::Output;
    fn y(&self) -> Self::Output;
    fn z(&self) -> Self::Output;
}

pub trait SurfaceTrait {}

pub trait DimensionsTrait {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

/// [RendererBuilderTrait] are categorized settings and initial values for a renderer ([RendererTrait]).
pub trait RendererBuilderTrait<Data> {
    type Dimensions: DimensionsTrait;
    type Camera: VertexTrait;
    type Canvas: SurfaceTrait;

    fn new() -> Self;
    fn dimensions(self, dimensions: Self::Dimensions) -> Self;
    fn camera(self, camera: Self::Camera) -> Self;
    fn canvas(self, canvas: Self::Canvas) -> Self;
    fn build(self) -> renderer::Renderer;
}

/// [RendererTrait] for rendering to display.
pub trait RendererTrait {
    type Vertex: VertexTrait;

    /// Project vertices on to a [SurfaceTrait].
    fn project(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) -> &dyn SurfaceTrait;

    /// Rasterize [RendererTrait::project]ed vertices.
    fn rasterize(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) -> &dyn SurfaceTrait;

    /// Do all steps needed, in correct order, to produce a fully rendered image.
    fn run_pipeline(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) {
        self.project(vertices);
        self.rasterize(vertices);
    }
}