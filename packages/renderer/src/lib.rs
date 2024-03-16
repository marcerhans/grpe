/// Current implementation only supports a terminal (/text) output for rendering,
/// and for simplicity only [f64] is used.

pub mod strategy;
pub use strategy::renderer;

pub mod common;
pub use common::*;

use linear_algebra::matrix::DataTrait;

#[derive(Default, Clone)]
pub enum RenderOption {
    All,
    #[default]
    Line,
    Vertices,
}

#[derive(Default, Clone)]
pub struct RendererConfiguration<'a> {
    dimensions: (usize, usize),
    camera: Vertex<f64>,
    canvas: Surface<'a, f64>,
    option: RenderOption,
}

/// [RendererBuilderTrait] are categorized settings and initial values for a renderer ([RendererTrait]).
pub trait RendererBuilderTrait<'a, Data: DataTrait>: Default {
    type Dimensions: DimensionsTrait;
    type Camera: VertexTrait;
    type Canvas: SurfaceTrait;
    type Renderer: RendererTrait<'a, Data>;

    /// In order to instantiate this type, since the implementation may vary for different renderers,
    /// the implementation should provide a 'man' (manual/info) string for what info is needed to 
    /// initialize it.
    /// 
    /// TODO: There are better solutions, but this will do for now.
    fn man() -> &'static str;

    fn with_dimensions(self, dimensions: Self::Dimensions) -> Self;
    fn with_camera(self, camera: Self::Camera) -> Self;
    fn with_canvas(self, canvas: Self::Canvas) -> Self;
    fn with_option(self, option: RenderOption) -> Self;

    /// Build an instance of [RendererTrait].
    fn build(self) -> Self::Renderer;

    /// Build an instance of [RendererTrait] using an existing [RendererConfiguration].
    fn build_with_config(self, config: RendererConfiguration<'a>) -> Self::Renderer;
}

/// [RendererTrait] for rendering to display on some [SurfaceTrait].
pub trait RendererTrait<'a, Data: DataTrait> {
    type Vertex: VertexTrait;

    /// Get [RendererConfiguration].
    fn config(&self) -> RendererConfiguration<'a>;

    /// Set a new config ([RendererConfiguration]) for the [RendererTrait].
    /// Useful if the dimensions of the canvas ([Surface]) changes in size, for example.
    /// Returns [Result::Ok] if configuration is valid for current renderer.
    fn set_config(&mut self, config: RendererConfiguration<'a>) -> Result<(), &'static str>;

    /// Vertices ([Vertex]) are used as "anchors" from which lines can be drawn.
    fn set_vertices(&mut self, vertices: &[Vertex<Data>]);

    /// Index for each vertex given in [RendererTrait::set_vertices] decides drawing order.
    /// 
    /// # Example
    /// Draw a line from (0,0) to (1,0) and from (0,0) to (0,1) to (1,0).
    /// 
    /// ```Rust
    /// let vertices = vec![Vertex::new(0.0, 0.0, 0.0), Vertex::new(1.0, 0.0, 0.0), Vertex::new(0.0, 1.0, 0.0)]
    /// let draw_order = vec![[0,1], [0,2,1]];
    /// 
    /// some_already_configured_renderer.set_vertices(&vertices);
    /// some_already_configured_renderer.set_vertices_line_draw_order(&draw_order);
    /// ```
    fn set_vertices_line_draw_order(&mut self, order: &[&[usize]]);

    /// Do the render! What is rendered in the final artefact is decided the the [RenderOption]s.
    fn render(&self);
}

/// Hidden trait methods for [RendererTrait].
trait __RendererTrait<'a, Data: DataTrait>: RendererTrait<'a, Data> {
    /// Create new instance.
    fn new(config: RendererConfiguration<'a>) -> Self;
}