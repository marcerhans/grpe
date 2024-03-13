use std::marker::PhantomData;

use crate::{RendererBuilderTrait, RendererTrait, SurfaceTrait};

use super::common::*;

mod character {
    pub static LINE_HORIZONTAL: char = '\u{254c}';  // ╌
    pub static LINE_VERTICAL: char = '\u{2506}';    // ┆
    pub static CENTER: char = '\u{253c}';           // ┼
    pub static UPPER: char = '\u{2580}';            // ▀
    pub static LOWER: char = '\u{2584}';            // ▄
    pub static FULL: char = '\u{2588}';             // █
    pub static EMPTY: char = ' ';                   //
}

pub struct TerminalBuilder<'a, Data>(PhantomData<&'a Data>);

impl<'a> RendererBuilderTrait<'a> for TerminalBuilder<'a, f64> {
    type Dimensions = (usize, usize);
    type Camera = Vertex<'a, f64>;
    type Canvas = Surface<'a, f64>;
    type Renderer = Terminal<'a, f64>;
    
    fn man() -> &'static str {
        todo!()
    }
    
    fn new() -> Self {
        todo!()
    }
    
    fn with_dimensions(self, dimensions: Self::Dimensions) -> Self {
        todo!()
    }
    
    fn with_camera(self, camera: Self::Camera) -> Self {
        todo!()
    }
    
    fn with_canvas(self, canvas: Self::Canvas) -> Self {
        todo!()
    }
    
    fn build(self) -> Self::Renderer {
        todo!()
    }
    
    fn build_with_config(self, config: crate::RendererConfiguration) -> Self::Renderer {
        todo!()
    }

}

pub struct Terminal<'a, T> {
    _phantom_data: PhantomData<&'a T>
}

/// A terminals blocks are usually not square, but rectangular. In order to achieve 
/// evenly sized blocks, the terminal is designed to use special
/// block characters (see [character]). This introduces some extra complexity, but the
/// result with be worth it. Otherwise, the final image would be quite oblong. 
impl<'a, T> Terminal<'a, T> {
    /// Get appropriate character to use for given vertical position.
    fn character_at(y: usize) -> char {
        if y % 2 == 0 {
            return character::UPPER;
        }

        character::LOWER
    }
}

impl<'a> RendererTrait<'a> for Terminal<'a, f64> {
    type Vertex = Vertex<'a, f64>;
    
    fn config(&self) -> crate::RendererConfiguration {
        todo!()
    }
    
    fn set_config(&self) -> Result<(), ()> {
        todo!()
    }
    
    fn project(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) -> &dyn SurfaceTrait {
        todo!()
    }
    
    fn rasterize(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) -> &dyn SurfaceTrait {
        todo!()
    }
}