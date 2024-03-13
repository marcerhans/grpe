pub mod strategy;
pub use strategy::renderer;

pub trait VertexTrait<'a> {
    type Data;
    type DataRef;

    fn new(data: [Self::Data; 3]) -> Self;
    fn x(&'a self) -> Self::DataRef;
    fn y(&'a self) -> Self::DataRef;
    fn z(&'a self) -> Self::DataRef;
    fn slice(&'a self) -> &[Self::Data];
}

pub trait SurfaceTrait {}

pub trait DimensionsTrait {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

/// [RendererBuilderTrait] are categorized settings and initial values for a renderer ([RendererTrait]).
pub trait RendererBuilderTrait<'a> {
    type Dimensions: DimensionsTrait;
    type Camera: VertexTrait<'a>;
    type Canvas: SurfaceTrait;
    type Renderer: RendererTrait<'a>;

    fn new() -> Self;
    fn dimensions(self, dimensions: Self::Dimensions) -> Self;
    fn camera(self, camera: Self::Camera) -> Self;
    fn canvas(self, canvas: Self::Canvas) -> Self;
    fn build(self) -> Self::Renderer;
}

/// [RendererTrait] for rendering to display on some [SurfaceTrait].
pub trait RendererTrait<'a> {
    type Vertex: VertexTrait<'a>;

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