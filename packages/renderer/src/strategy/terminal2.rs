use std::cell::RefCell;
use std::io::Write;

use crate::{Camera, RenderOption, RendererBuilderTrait, RendererConfiguration, RendererTrait, __RendererTrait};
use linear_algebra::vector::VectorRow;

mod character {
    pub static LINE_HORIZONTAL: char = '\u{254c}'; // ╌
    pub static LINE_VERTICAL: char = '\u{2506}'; // ┆
    pub static CENTER: char = '\u{253c}'; // ┼
    // pub static UPPER: char = '\u{2580}'; // ▀
    pub static UPPER: char = '\u{1FB91}'; // ▀
    // pub static LOWER: char = '\u{2584}'; // ▄
    pub static LOWER: char = '\u{1FB92}'; // ▄
    pub static FULL: char = '\u{2588}'; // █
    // pub static UPPER_EMPTY: char = '\u{1FB91}'; // 🮎
    // pub static LOWER_EMPTY: char = '\u{1FB92}'; // 🮏
    // pub static FULL_EMPTY: char = '\u{2592}'; // ▒
    pub static EMPTY: char = '\u{2592}';
}

mod ansi {
    pub static CLEAR_SCREEN: &str = "\x1B[2J";
    pub static GO_TO_0_0: &str = "\x1B[H";
}

#[derive(Default)]
pub struct TerminalBuilder {
    config: RendererConfiguration,
}

impl<'a> RendererBuilderTrait<'a> for TerminalBuilder {
    type Renderer = Terminal<'a>;

    fn with_camera(mut self, camera: Camera) -> Self {
        self.config.camera = camera;
        self
    }

    fn with_option(mut self, option: RenderOption) -> Self {
        self.config.option = option;
        self
    }

    fn build(self) -> Self::Renderer {
        Self::Renderer::new(self.config)
    }

    fn build_with_config(self, config: RendererConfiguration) -> Self::Renderer {
        Self::Renderer::new(config)
    }
}

/// Typed state terminal renderer.
pub struct Terminal<'a> {
    config: RendererConfiguration,
    vertices: Option<&'a [VectorRow<f64, 3>]>,
    // line_draw_order: Vec<usize>, // TODO
    buffer: RefCell<Vec<Vec<char>>>,
}

/// This implementation can be seen as being the pipeline stages for the renderer, in the order of definitions.
impl<'a> Terminal<'a> {
    /// Clear the buffer and the terminal screen.
    fn clear(&self) {
        for v in self.buffer.borrow_mut().iter_mut() {
            for c in v.iter_mut() {
                *c = character::EMPTY;
            }
        }

        let stdout = std::io::stdout();
        let mut handle = std::io::BufWriter::new(stdout.lock());
        write!(handle, "{}{}", ansi::CLEAR_SCREEN, ansi::GO_TO_0_0).unwrap();
        handle.flush().unwrap() // TODO: Potentially do not flush here, but when doing the last step of the pipeline.
    }

    /// Projects vertices ([VectorRow]) onto the plane of the viewport that is the [Camera].
    /// TODO: If viewport could be a more concrete type/member of a struct, add reference here.
    fn project_vertices_on_viewport(&self) {
        todo!()
    }

    fn render_vertices(&self) {
        todo!()
    }
    
    fn render_lines(&self) {
        todo!()
    }
    
    /// Print buffer to terminal.
    fn print_to_terminal(&self) {
        let stdout = std::io::stdout();
        let mut handle = std::io::BufWriter::new(stdout.lock());

        for character_row in self.buffer.borrow().iter() {
            for character in character_row.iter() {
                write!(handle, "{character}").unwrap();
            }
            write!(handle, "\n").unwrap();
        }

        handle.flush().unwrap()
    }
}

impl<'a> RendererTrait<'a> for Terminal<'a> {
    fn config(&self) -> RendererConfiguration {
        self.config.clone()
    }

    fn set_config(&mut self, config: RendererConfiguration) -> Result<(), &'static str> {
        self.config = config;
        Ok(())
    }

    fn set_vertices(&'a mut self, vertices: &'a [VectorRow<f64, 3>]) {
        self.vertices = Some(vertices);
    }

    fn set_vertices_line_draw_order(&'a mut self, order: &'a [&'a [usize]]) {
        todo!("Implement this later")
    }

    fn render(&self) {
        self.clear();
        // self.project_vertices_on_viewport();
        // self.render_vertices();
        // self.render_lines();
        self.print_to_terminal();
    }
}

impl<'a> __RendererTrait<'a> for Terminal<'a> {
    fn new(config: RendererConfiguration) -> Self {
        let resolution = config.camera.resolution;

        Self {
            config,
            vertices: None,
            buffer: RefCell::new(vec![vec![character::EMPTY; resolution.0 as usize]; (resolution.1 / 2) as usize]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{RendererConfiguration, RendererTrait};
    use super::Terminal;
}