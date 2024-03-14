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
    type Camera = Vertex<f64>;
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
            center_offset: (dim.0 / 2, dim.1 / 2),
        }
    }

    fn build_with_config(self, config: crate::RendererConfiguration<'a>) -> Self::Renderer {
        let dim = config.dimensions;
        Self::Renderer {
            config: config,
            buffer: Matrix::zeros_dyn(dim.width(), dim.height()),
            center_offset: (dim.0 / 2, dim.1 / 2),
        }
    }
}

pub struct Terminal<'a, Data: DataTrait> {
    config: RendererConfiguration<'a>,
    buffer: Matrix<Data>,
    center_offset: (usize, usize),
}

/// A terminals blocks are usually not square, but rectangular. In order to achieve
/// evenly sized blocks, the terminal is designed to use special
/// block characters (see [character]). This introduces some extra complexity, but the
/// result with be worth it. Otherwise, the final image would be quite oblong.
impl<'a> Terminal<'a, f64> {
    /// Get appropriate character to use for given vertical position.
    fn character_at(y: usize) -> char {
        if y % 2 == 0 {
            return character::UPPER;
        }

        character::LOWER
    }

    fn adjust_points(&self, points: &mut [(Vertex<f64>, Vertex<f64>, Vertex<f64>)]) {
        for (ref mut a, ref mut b, ref mut c) in points.iter_mut() {
            *a.x_mut() -= self.center_offset.0 as f64;
            *a.y_mut() -= self.center_offset.1 as f64;
            *b.x_mut() -= self.center_offset.0 as f64;
            *b.y_mut() -= self.center_offset.1 as f64;
            *c.x_mut() -= self.center_offset.0 as f64;
            *c.y_mut() -= self.center_offset.1 as f64;
        }
    }

    /// Clear previously rendered frame.
    fn clear(&self) {}

    /// Render to stdout.
    fn render(&self, points: &mut [(Vertex<f64>, Vertex<f64>, Vertex<f64>)]) {
        let mut buffer = vec![vec![' '; self.config.dimensions.1]; self.config.dimensions.0];

        for (ref a, ref b, ref c) in points.iter() {
            self.render_vertex(&mut buffer, a);
            self.render_vertex(&mut buffer, b);
            self.render_vertex(&mut buffer, c);
        }

        for characters in buffer.iter() {
            for character in characters.iter() {
                print!("{character}");
            }
            println!();
        }
    }

    fn render_vertex(&self, buffer: &mut Vec<Vec<char>>, vertex: &Vertex<f64>) {
        let x = *vertex.x() as usize;
        let y = *vertex.y() as usize;
        let mut character = Self::character_at(y);

        if y > 0 {
            if buffer[y - 1][x] == character::UPPER && character == character::LOWER {
                character = character::FULL;
            }
        }

        if y < self.config.dimensions.1 - 1 {
            if buffer[y + 1][x] == character::LOWER && character == character::UPPER {
                character = character::FULL;
            }
        }

        let _ = std::mem::replace(&mut buffer[y][x], character);
    }
}

impl<'a> RendererTrait<'a> for Terminal<'a, f64> {
    type Vertex = Vertex<f64>;

    fn config(&self) -> crate::RendererConfiguration<'a> {
        self.config.clone()
    }

    fn set_config(&mut self, config: RendererConfiguration<'a>) -> Result<(), ()> {
        self.config = config;
        Ok(())
    }

    fn project_on_canvas(
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

/// These tests are not that thorough, just helpful testing/probing during development.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let renderer = TerminalBuilder::default().with_dimensions((10, 10)).build();

        let mut vertex_triples = vec![
            (
                Vertex::new([0.0, 0.0, 0.0]),
                Vertex::new([4.0, 0.0, 0.0]),
                Vertex::new([4.0, 4.0, 0.0]),
            ),
            // [Vertex::new([]), Vertex::new([]), Vertex::new([])],
        ];

        renderer.adjust_points(&mut vertex_triples);
        renderer.render(&mut vertex_triples);
        // renderer.run_pipeline();
    }
}
