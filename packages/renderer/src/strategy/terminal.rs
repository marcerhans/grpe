use linear_algebra::matrix::{DataTrait, Matrix};

use crate::{common::*, RendererBuilderTrait, RendererConfiguration, RendererTrait, __RendererTrait, SurfaceTrait};

mod character {
    pub static LINE_HORIZONTAL: char = '\u{254c}'; // ╌
    pub static LINE_VERTICAL: char = '\u{2506}'; // ┆
    pub static CENTER: char = '\u{253c}'; // ┼
    pub static UPPER: char = '\u{2580}'; // ▀
    pub static LOWER: char = '\u{2584}'; // ▄
    pub static FULL: char = '\u{2588}'; // █
    pub static EMPTY: char = '+'; //
}

pub struct TerminalBuilder<'a> {
    config: RendererConfiguration<'a>,
}

impl<'a> Default for TerminalBuilder<'a> {
    fn default() -> Self {
        Self {
            config: RendererConfiguration::default(),
        }
    }
}

impl<'a> RendererBuilderTrait<'a, f64> for TerminalBuilder<'a> {
    type Dimensions = (usize, usize);
    type Camera = Vertex<f64>;
    type Canvas = Surface<'a, f64>;
    type Renderer = Terminal<'a, f64>;

    fn man() -> &'static str {
        todo!()
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
        Self::Renderer::new(self.config)
    }

    fn build_with_config(self, config: crate::RendererConfiguration<'a>) -> Self::Renderer {
        Self::Renderer::new(self.config)
    }
}

pub struct Terminal<'a, Data: DataTrait> {
    config: RendererConfiguration<'a>,
    vertices: Vec<Vertex<Data>>,
    line_draw_order: Vec<usize>,
    buffer: Vec<Vec<char>>,
    center_offset: (isize, isize),
}

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
            *a.x_mut() += self.center_offset.0 as f64;
            *a.y_mut() += self.center_offset.1 as f64;
            *b.x_mut() += self.center_offset.0 as f64;
            *b.y_mut() += self.center_offset.1 as f64;
            *c.x_mut() += self.center_offset.0 as f64;
            *c.y_mut() += self.center_offset.1 as f64;
        }
    }

    /// Clear previously rendered frame.
    fn clear(&self) {}

    /// Render to stdout.
    fn render(&self, points: &mut [(Vertex<f64>, Vertex<f64>, Vertex<f64>)]) {
        let mut buffer = vec![vec![character::EMPTY; self.config.dimensions.1]; self.config.dimensions.0];

        for (ref a, ref b, ref c) in points.iter() {
            self.render_vertex(&mut buffer, a);
            self.render_vertex(&mut buffer, b);
            self.render_vertex(&mut buffer, c);
            println!("{a:?},{b:?},{c:?}");
        }

        println!("{:?}", buffer);

        for characters in buffer.iter() {
            for character in characters.iter() {
                print!("{character}");
            }
            println!();
        }
    }

    fn render_vertex(&self, buffer: &mut Vec<Vec<char>>, vertex: &Vertex<f64>) {
        let x = *vertex.x() as isize;
        let y = *vertex.y() as isize;
        let mut character = Self::character_at(y.abs() as usize);

        if y > 0 {
            if buffer[y as usize - 1][x as usize] == character::UPPER && character == character::LOWER {
                character = character::FULL;
            }
        }

        if y < self.config.dimensions.1 as isize - 1 {
            if buffer[y as usize + 1][x as usize] == character::LOWER && character == character::UPPER {
                character = character::FULL;
            }
        }

        let _ = std::mem::replace(&mut buffer[y as usize][x as usize], character);
    }
}

impl<'a> RendererTrait<'a, f64> for Terminal<'a, f64> {
    type Vertex = Vertex<f64>;

    fn config(&self) -> crate::RendererConfiguration<'a> {
        self.config.clone()
    }

    fn set_config(&mut self, config: RendererConfiguration<'a>) -> Result<(), &'static str> {
        self.config = config;
        Ok(())
    }
    
    fn set_vertices(&mut self, vertices: &[Vertex<f64>]) {
        todo!()
    }
    
    fn set_vertices_line_draw_order(&mut self, order: &[&[usize]]) {
        todo!()
    }
    
    fn render(&self) {
        todo!()
    }

}

impl<'a> __RendererTrait<'a, f64> for Terminal<'a, f64> {
    fn new(config: RendererConfiguration<'a>) -> Self {
        let dim = config.dimensions;
        Self {
            config,
            vertices: Vec::new(),
            line_draw_order: Vec::new(),
            buffer: vec![vec![character::EMPTY; dim.1]; dim.0],
            center_offset: ((dim.0 / 2) as isize, -((dim.1 / 2) as isize)),
        }
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
                Vertex::new([0.0, 2.0, 0.0]),
                Vertex::new([1.0, 0.0, 0.0]),
                Vertex::new([2.0, 0.0, 0.0]),
            ),
            // [Vertex::new([]), Vertex::new([]), Vertex::new([])],
        ];

        renderer.render(&mut vertex_triples);
        renderer.adjust_points(&mut vertex_triples);
        // renderer.print(&vertex_triples);
        // renderer.run_pipeline();
    }
}