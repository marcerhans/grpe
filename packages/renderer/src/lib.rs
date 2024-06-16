/// Current implementation only supports a terminal (/text) output for rendering,
/// and for simplicity only [f64] is used.

pub mod strategy;
pub use strategy::renderer;

mod common;

use linear_algebra::matrix::{Matrix, MatrixDataTrait};

#[derive(Clone)]
pub struct Camera<T: MatrixDataTrait> {
    pub resolution: (u64, u64),
    pub position: Matrix<T>,
    pub direction: Matrix<T>,
    pub fov: u64,
}

impl<T: MatrixDataTrait> Camera<T> {
    pub fn new(resolution: (u64, u64), position: &[T; 3], direction: &[T; 3], fov: u64) -> Self {
        Self {
            resolution,
            position: Matrix::from_slice(&[position]),
            direction: Matrix::from_slice(&[direction]),
            fov,
        }
    }

    pub fn resolution(&self) -> &(u64, u64) {
        &self.resolution
    }

    pub fn position(&self) -> &Matrix<T> {
        &self.position
    }

    pub fn direction(&self) -> &Matrix<T> {
        &self.direction
    }
    
    pub fn fov(&self) -> &u64{
        &self.fov
    }
}

impl Default for Camera<f64> {
    fn default() -> Self {
        Self {
            resolution: (32, 32),
            position: Matrix::from_array([[0.0, 0.0, 0.0]]),
            direction: Matrix::from_array([[0.0, 1.0, 0.0]]),
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
pub struct RendererConfiguration<T: MatrixDataTrait> {
    pub camera: Camera<f64>,
    pub option: RenderOption,
    pub(crate) viewpoint: Matrix<T>,
    pub(crate) viewport: Matrix<T>,
}

/// [RendererBuilderTrait] are categorized settings and initial values for a renderer ([RendererTrait]).
pub trait RendererBuilderTrait<T: MatrixDataTrait>: Default {
    type Renderer: RendererTrait<T>;

    fn with_camera(self, camera: Camera<T>) -> Self;
    fn with_option(self, option: RenderOption) -> Self;

    /// Build an instance of [RendererTrait].
    fn build(self) -> Self::Renderer;

    /// Build an instance of [RendererTrait] using an existing [RendererConfiguration].
    fn build_with_config(self, config: RendererConfiguration<T>) -> Self::Renderer;
}

/// [RendererTrait] for rendering to display.
pub trait RendererTrait<T: MatrixDataTrait> {
    type Vertex;

    /// Get [RendererConfiguration].
    fn config(&self) -> RendererConfiguration<T>;

    /// Set a new config ([RendererConfiguration]) for the [RendererTrait].
    /// Useful if the dimensions of the viewport ([common::PlaneTrait]) changes in size, for example.
    /// Returns [Result::Ok] if configuration is valid for current renderer.
    fn set_config(&mut self, config: RendererConfiguration<T>) -> Result<(), &'static str>;

    /// Vertices ([VertexTrait]) are used as "anchors"/"points in space" from which lines can be drawn.
    fn set_vertices(&mut self, vertices: &[Self::Vertex]);

    /// Index for each vertex given in [RendererTrait::set_vertices] decides drawing order.
    /// 
    /// # Example
    /// Draw a line from (0,0) to (1,0) and from (0,0) to (0,1) to (1,0).
    /// 
    /// ```Rust
    /// let vertices = vec![Vertex::from_array([[0.0, 0.0, 0.0]]), Vertex::from_array([[1.0, 0.0, 0.0]]), Vertex::from_array([[0.0, 1.0, 0.0]])]
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
    fn new(config: RendererConfiguration<T>) -> Self;
}