/// Current implementation only supports a terminal (/text) output for rendering,
/// and for simplicity only [f64] is used.

pub mod strategy;

use std::{cell::RefCell, rc::Rc};

pub use strategy::renderer;
pub use linear_algebra::{matrix::{Matrix, MatrixDataTrait}, vector::VectorRow};

#[derive(Clone)]
pub struct Camera {
    pub resolution: (u64, u64),
    pub position: VectorRow<f64, 3>,
    pub rotation: VectorRow<f64, 3>,
    pub fov: u64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            resolution: (32, 32),
            position: VectorRow::from([0.0, -1.0, 0.0]),
            rotation: VectorRow::from([0.0, 0.0, 0.0]),
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
pub trait RendererBuilderTrait: Default {
    type Renderer: RendererTrait;

    fn with_camera(self, camera: Camera) -> Self;
    fn with_option(self, option: RenderOption) -> Self;

    /// Build an instance of [RendererTrait].
    fn build(self) -> Result<Self::Renderer, &'static str>;

    /// Build an instance of [RendererTrait] using an existing [RendererConfiguration].
    fn build_with_config(self, config: RendererConfiguration) -> Result<Self::Renderer, &'static str>;
}

/// [RendererTrait] for rendering to display.
pub trait RendererTrait where Self: Sized {
    /// Get reference to [RendererConfiguration].
    fn config(&self) -> &RendererConfiguration;

    /// Set a new [Camera].
    fn set_camera(self, camera: Camera) -> Result<Self, &'static str>;

    /// Set a new [RenderOption].
    fn set_option(self, option: RenderOption) -> Result<Self, &'static str>;

    /// Set a new config ([RendererConfiguration]) for the [RendererTrait].
    fn set_config(self, config: RendererConfiguration) -> Result<Self, &'static str>;

    /// Vertices are used as "anchors"/"points in space" from which lines can be drawn.
    fn set_vertices(&mut self, vertices: Rc<RefCell<Vec<VectorRow<f64, 3>>>>);

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
    /// 
    /// TODO: Update doc!
    /// ```
    fn set_vertices_line_draw_order(&mut self, order: Rc<RefCell<[Box<[usize]>]>>); // TODO: Probably not right :))

    /// Do the render! What is rendered in the final artefact is decided the the [RenderOption]s.
    fn render(&mut self);
}

/// Hidden trait methods for [RendererTrait].
trait __RendererTrait: RendererTrait {
    /// Create new instance.
    fn new(config: RendererConfiguration) -> Result<Self, &'static str>;
}
