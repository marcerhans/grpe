/// Current implementation only supports a terminal (/text) output for rendering,
/// and for simplicity only [f64] is used.
/// 
/// TODO: How does the Into trait get implemented automatically from From?

pub mod strategy;
pub use strategy::renderer;

mod common;

use linear_algebra::matrix::MatrixDataTrait;

#[derive(Clone)]
pub struct Camera<T: MatrixDataTrait> {
    resolution: (usize, usize),
    position: (T, T, T),
    direction: (T, T, T),
    fov: usize,
}

impl<T: MatrixDataTrait> Camera<T> {
    pub fn new(resolution: (usize, usize), position: (T, T, T), direction: (T, T, T), fov: usize) -> Self {
        Self {
            resolution,
            position,
            direction,
            fov,
        }
    }

    pub fn resolution(&self) -> &(usize, usize) {
        &self.resolution
    }

    pub fn position(&self) -> &(T, T, T) {
        &self.position
    }

    pub fn direction(&self) -> &(T, T, T) {
        &self.direction
    }
    
    pub fn fov(&self) -> &usize {
        &self.fov
    }
}

impl Default for Camera<f64> {
    fn default() -> Self {
        Self {
            resolution: (100, 100),
            position: (0.0, 0.0, -10.0),
            direction: (0.0, 0.0, 1.0),
            fov: 90,
        }
    }
}

#[derive(Default, Clone)]
pub enum RenderOption {
    All,
    #[default]
    Line,
    Vertices,
}

#[derive(Default, Clone)]
pub struct RendererConfiguration {
    pub camera: Camera<f64>,
    pub option: RenderOption,
}

/// [RendererBuilderTrait] are categorized settings and initial values for a renderer ([RendererTrait]).
pub trait RendererBuilderTrait<T: MatrixDataTrait>: Default {
    type Renderer: RendererTrait<T>;

    /// In order to instantiate this type, since the implementation may vary for different renderers,
    /// the implementation should provide a 'man' (manual/info) string for what info is needed to 
    /// initialize it.
    /// 
    /// TODO: There are better solutions, but this will do for now.
    fn man() -> &'static str;

    fn with_camera(self, camera: Camera<T>) -> Self;
    fn with_option(self, option: RenderOption) -> Self;

    /// Build an instance of [RendererTrait].
    fn build(self) -> Self::Renderer;

    /// Build an instance of [RendererTrait] using an existing [RendererConfiguration].
    fn build_with_config(self, config: RendererConfiguration) -> Self::Renderer;
}

/// [RendererTrait] for rendering to display on some [PlaneTrait].
pub trait RendererTrait<T: MatrixDataTrait> {
    type Vertex;

    /// Get [RendererConfiguration].
    fn config(&self) -> RendererConfiguration;

    /// Set a new config ([RendererConfiguration]) for the [RendererTrait].
    /// Useful if the dimensions of the viewport ([Plane]) changes in size, for example.
    /// Returns [Result::Ok] if configuration is valid for current renderer.
    fn set_config(&mut self, config: RendererConfiguration) -> Result<(), &'static str>;

    /// Vertices ([VertexTrait]) are used as "anchors" from which lines can be drawn.
    fn set_vertices(&mut self, vertices: &[Self::Vertex]);

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
trait __RendererTrait<T: MatrixDataTrait>: RendererTrait<T> {
    /// Create new instance.
    fn new(config: RendererConfiguration) -> Self;
}