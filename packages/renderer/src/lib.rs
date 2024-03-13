/// TODO: Currently (mostly) focuses on f64, and not general data types.

pub mod strategy;
pub use strategy::{renderer, common::*};


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