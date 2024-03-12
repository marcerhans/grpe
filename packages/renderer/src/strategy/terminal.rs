use crate::RendererTrait;

use super::common::*;

mod symbol {
    pub static LINE_HORIZONTAL: char = '\u{254c}';  // ╌
    pub static LINE_VERTICAL: char = '\u{2506}';    // ┆
    pub static CENTER: char = '\u{253c}';           // ┼
    pub static UPPER: char = '\u{2580}';            // ▀
    pub static LOWER: char = '\u{2584}';            // ▄
    pub static FULL: char = '\u{2588}';             // █
    pub static EMPTY: char = ' ';                   //
}

pub struct Renderer;

impl RendererTrait for Renderer {
    type Vertex = Vertex<i64>;
    
    fn project(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) -> &dyn crate::SurfaceTrait {
        todo!()
    }
    
    fn rasterize(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) -> &dyn crate::SurfaceTrait {
        todo!()
    }
}