use crate::{DimensionsTrait, RendererBuilderTrait, RendererTrait};

mod symbol {
    pub static LINE_HORIZONTAL: char = '\u{254c}';  // ╌
    pub static LINE_VERTICAL: char = '\u{2506}';    // ┆
    pub static CENTER: char = '\u{253c}';           // ┼
    pub static UPPER: char = '\u{2580}';            // ▀
    pub static LOWER: char = '\u{2584}';            // ▄
    pub static FULL: char = '\u{2588}';             // █
    pub static EMPTY: char = ' ';                   //
}

impl DimensionsTrait for (usize, usize) {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }
}

#[derive(Default)]
struct RendererConfig {
    dimensions: (usize, usize),
}

pub struct RendererBuilder {
    config: RendererConfig,
}

impl RendererBuilderTrait for RendererBuilder {
    fn new() -> Self {
        Self {
            config: RendererConfig::default(),
        }
    }

    fn dimensions(self, dimensions: (usize, usize)) -> Self {
        self.config.dimensions
    }

    fn build() -> self::Renderer {
        Renderer {

        }
    }
    
    type Dimensions;
}

pub struct Renderer {
    buffer: Vec<char>,
}

impl RendererTrait for Renderer {
    fn rasterize(&self, vertices: &[crate::Vertex]) {
        todo!()
    }
}