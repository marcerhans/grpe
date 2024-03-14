use std::marker::PhantomData;

use linear_algebra::matrix::{DataTrait, Matrix};

use crate::{common::*, RendererBuilderTrait, RendererConfiguration, RendererTrait, SurfaceTrait};

mod character {
    pub static LINE_HORIZONTAL: char = '\u{254c}'; // ╌
    pub static LINE_VERTICAL: char = '\u{2506}'; // ┆
    pub static CENTER: char = '\u{253c}'; // ┼
    pub static UPPER: char = '\u{2580}'; // ▀
    pub static LOWER: char = '\u{2584}'; // ▄
    pub static FULL: char = '\u{2588}'; // █
    pub static EMPTY: char = ' '; //
}

pub struct TerminalBuilder<'a> {
    config: RendererConfiguration<'a>,
}

impl<'a> RendererBuilderTrait<'a> for TerminalBuilder<'a> {
    type Dimensions = (usize, usize);
    type Camera = Vertex<'a, f64>;
    type Canvas = Surface<'a, f64>;
    type Renderer = Terminal<'a, f64>;

    fn man() -> &'static str {
        todo!()
    }

    fn default() -> Self {
        Self {
            config: RendererConfiguration::default(),
        }
    }

    fn with_dimensions(mut self, dimensions: Self::Dimensions) -> Self {
        self.config.dimensions = dimensions;
        self
    }

    fn with_camera(mut self, camera: Self::Camera) -> Self {
        self.config.camera = camera;
        self
    }

    fn with_canvas(mut self, canvas: Self::Canvas) -> Self {
        self.config.canvas = canvas;
        self
    }

    fn build(self) -> Self::Renderer {
        let dim = self.config.dimensions;
        Self::Renderer {
            config: self.config,
            buffer: Matrix::zeros_dyn(dim.width(), dim.height()),
        }
    }

    fn build_with_config(self, config: crate::RendererConfiguration<'a>) -> Self::Renderer {
        let dim = config.dimensions;
        Self::Renderer {
            config: config,
            buffer: Matrix::zeros_dyn(dim.width(), dim.height()),
        }
    }
}

pub struct Terminal<'a, Data: DataTrait> {
    config: RendererConfiguration<'a>,
    buffer: Matrix<Data>,
}

/// A terminals blocks are usually not square, but rectangular. In order to achieve
/// evenly sized blocks, the terminal is designed to use special
/// block characters (see [character]). This introduces some extra complexity, but the
/// result with be worth it. Otherwise, the final image would be quite oblong.
impl<'a, Data: DataTrait> Terminal<'a, Data> {
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

    fn config(&self) -> crate::RendererConfiguration<'a> {
        self.config.clone()
    }

    fn set_config(&mut self, config: RendererConfiguration<'a>) -> Result<(), ()> {
        self.config = config;
        Ok(())
    }

    fn project(
        &self,
        vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)],
    ) -> &dyn SurfaceTrait {
        todo!()
    }

    fn rasterize(
        &self,
        vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)],
    ) -> &dyn SurfaceTrait {
        todo!()
    }
}
