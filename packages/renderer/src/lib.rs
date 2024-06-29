/// Current implementation only supports a terminal (/text) output for rendering,
/// and for simplicity only [f64] is used.

pub mod strategy;

pub use strategy::renderer;
pub use linear_algebra::{matrix::{Matrix, MatrixDataTrait}, vector::VectorRow};

#[derive(Clone)]
pub struct Camera {
    pub resolution: (u64, u64),
    pub position: VectorRow<f64, 3>,
    pub direction: VectorRow<f64, 3>,
    pub fov: u64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            resolution: (32, 32),
            position: VectorRow::from([0.0, -1.0, 0.0]),
            direction: VectorRow::from([0.0, 1.0, 0.0]),
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
    pub camera: Camera,
    pub option: RenderOption,
}

/// [RendererBuilderTrait] are categorized settings and initial values for a renderer ([RendererTrait]).
pub trait RendererBuilderTrait<'a>: Default {
    type Renderer: RendererTrait<'a>;

    fn with_camera(self, camera: Camera) -> Self;
    fn with_option(self, option: RenderOption) -> Self;

    /// Build an instance of [RendererTrait].
    fn build(self) -> Self::Renderer;

    /// Build an instance of [RendererTrait] using an existing [RendererConfiguration].
    fn build_with_config(self, config: RendererConfiguration) -> Self::Renderer;
}

/// [RendererTrait] for rendering to display.
pub trait RendererTrait<'a> {
    /// Get [RendererConfiguration].
    fn config(&self) -> RendererConfiguration;

    /// Set a new config ([RendererConfiguration]) for the [RendererTrait].
    /// Returns [Result::Ok] if configuration is valid for current renderer.
    fn set_config(&mut self, config: RendererConfiguration) -> Result<(), &'static str>;

    /// Vertices are used as "anchors"/"points in space" from which lines can be drawn.
    fn set_vertices(&'a mut self, vertices: &'a [VectorRow<f64, 3>]);

    /// Index for each vertex given in [RendererTrait::set_vertices] decides drawing order.
    /// 
    /// # Example
    /// Draw a line from (0,0,0) to (1,0,0) and from (0,0,0) to (0,1,0) to (1,0,0).
    /// 
    /// ```Rust
    /// let vertices = vec![VectorRow::from([0.0, 0.0, 0.0]), VectorRow::from([1.0, 0.0, 0.0]), VectorRow::from([0.0, 1.0, 0.0])]
    /// let draw_order = vec![[0,1], [0,2,1]];
    /// 
    /// some_already_configured_renderer.set_vertices(&vertices);
    /// some_already_configured_renderer.set_vertices_line_draw_order(&draw_order);
    /// ```
    fn set_vertices_line_draw_order(&'a mut self, order: &'a [&'a [usize]]);

    /// Do the render! What is rendered in the final artefact is decided the the [RenderOption]s.
    fn render(&self);
}

/// Hidden trait methods for [RendererTrait].
trait __RendererTrait<'a>: RendererTrait<'a> {
    /// Create new instance.
    fn new(config: RendererConfiguration) -> Self;
}
