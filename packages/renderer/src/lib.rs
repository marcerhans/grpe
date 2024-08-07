/// Current implementation only supports a terminal (/text) output for rendering,
/// and for simplicity only [f64] is used.

pub mod strategy;

use std::{cell::RefCell, rc::Rc, str::FromStr};

pub use strategy::renderer;
pub use linear_algebra::{matrix::{Matrix, MatrixDataTrait}, vector::VectorRow};

#[derive(Clone)]
pub struct Camera {
    pub resolution: (u64, u64),
    pub position: VectorRow<f64, 3>,
    /// Pitch, Yaw
    pub rotation: (f64, f64),
    pub fov: u64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            resolution: (32, 32),
            position: VectorRow::from([0.0, -1.0, 0.0]),
            rotation: (0.0, 0.0),
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

impl FromStr for RenderOption {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "all" => Ok(RenderOption::All),
            "line" => Ok(RenderOption::Line),
            "vertices" => Ok(RenderOption::Vertices),
            _ => Err("Could not convert from string"),
        }
    }
}

impl ToString for RenderOption {
    fn to_string(&self) -> String {
        match self {
            RenderOption::All => "all".to_owned(),
            RenderOption::Line => "line".to_owned(),
            RenderOption::Vertices => "vertices".to_owned(),
        }
    }
}

#[derive(Default, Clone)]
pub struct RendererConfiguration {
    pub camera: Camera,
    pub option: RenderOption,
}

/// [RendererBuilderTrait] are categorized settings and initial values for a renderer ([RendererTrait]).
pub trait RendererBuilderTrait: Default {
    type Renderer: RendererTrait;

    fn with_camera(self, camera: Camera) -> Result<Self, &'static str>;
    fn with_option(self, option: RenderOption) -> Result<Self, &'static str>;

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

    /// Set a new config [RendererConfiguration].
    fn set_config(self, config: RendererConfiguration) -> Result<Self, &'static str>;

    /// Vertices are used as "anchors"/"points in space" from which lines can be drawn.
    fn set_vertices(&mut self, vertices: Rc<RefCell<Vec<VectorRow<f64, 3>>>>);

    /// Index for each vertex given in [RendererTrait::set_vertices] decides drawing order.
    /// 
    /// # Example
    /// Draw a line from (0,0,0) to (1,0,0) and from (0,0,0) to (0,1,0) to (1,0,0).
    /// ```
    fn set_vertices_line_draw_order(&mut self, order: Rc<RefCell<[Box<[usize]>]>>); // TODO: Probably not right :))

    /// Do the render! What is rendered in the final artefact is decided by the [RenderOption]s.
    fn render(&mut self);
}

/// Hidden trait methods for [RendererTrait].
trait __RendererTrait: RendererTrait {
    /// Create new instance.
    fn new(config: RendererConfiguration) -> Result<Self, &'static str>;
}
