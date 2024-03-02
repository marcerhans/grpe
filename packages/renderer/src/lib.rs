use linear_algebra::matrix::Matrix;

pub mod strategy;
pub use strategy::renderer;

pub struct Camera {
    position: [f64; 3],
}

pub struct Canvas {
    position: [f64; 3],
    parameter_1
}

pub struct Vertex {
    position: [f64; 3],
}

pub trait DimensionsTrait {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
}

/// [RendererBuilderTrait] are categorized settings for a renderer.
pub trait RendererBuilderTrait {
    type Dimensions: DimensionsTrait;

    fn new() -> Self;
    fn dimensions(self, dimensions: Self::Dimensions) -> Self;
    fn build(self) -> renderer::Renderer;
}

/// [RendererTrait] for rendering to display.
pub trait RendererTrait {
    /// Project vertices on to canvas.
    fn project(&self, vertices: &[(Vertex, Vertex, Vertex)]);

    /// Rasterize [RendererTrait::project]ed vertices.
    fn rasterize(&self, vertices: &[(Vertex, Vertex, Vertex)]);

    /// Do all steps needed, in correct order, to produce a fully rendered image.
    fn run_pipeline(&self, vertices: &[(Vertex, Vertex, Vertex)]) {
        self.rasterize(vertices);
    }
}