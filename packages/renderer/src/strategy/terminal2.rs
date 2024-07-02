use std::{cell::RefCell, io::BufWriter};
use std::io::{StdoutLock, Write};

use crate::{Camera, RenderOption, RendererBuilderTrait, RendererConfiguration, RendererTrait, __RendererTrait};
use linear_algebra::vector::VectorRow;
use linear_algebra::matrix::Matrix;

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
    canvas: Vec<Vec<char>>,
    /// Fn(vertex_origin, vertex_direction_vector_to_viewpoint) -> Point at which the line crosses the canvas plane.
    canvas_line_intersection_checker: Box<dyn Fn(&mut VectorRow<f64, 3>, &mut VectorRow<f64, 3>) -> VectorRow<f64, 3>>,
    stdout_buffer: Option<BufWriter<StdoutLock<'static>>>,
}

/// This implementation can be seen as being the pipeline stages for the renderer, in the order of definitions.
impl<'a> Terminal<'a> {
    /// Clear the canvas buffer and the terminal screen.
    fn clear(&mut self) {
        for v in self.canvas.iter_mut() {
            for c in v.iter_mut() {
                *c = character::EMPTY;
            }
        }

        self.stdout_buffer = Some(BufWriter::new(std::io::stdout().lock()));
        write!(self.stdout_buffer.as_mut().unwrap(), "{}{}", ansi::CLEAR_SCREEN, ansi::GO_TO_0_0).unwrap();
    }

    /// Projects vertices ([VectorRow]) onto the plane of the viewport that is the [Camera].
    /// Returns the coordinates for the projected vertices.
    /// TODO: If viewport could be a more concrete type/member of a struct, add reference here.
    fn project_vertices_on_viewport(&self) -> Vec<VectorRow<f64, 3>> {
        let projected_vertices = Vec::new();

        for vertex in self.vertices.as_ref().unwrap().iter() {
            todo!(
                "Project each vertex on the viewport plane."
            )
        }

        projected_vertices
    }

    fn render_vertices(&self) {
        todo!()
    }
    
    fn render_lines(&self) {
        todo!()
    }
    
    /// Print canvas buffer to terminal.
    fn print_to_terminal(&mut self) {
        let stdout = self.stdout_buffer.as_mut().unwrap();

        for character_row in self.canvas.iter() {
            for character in character_row.iter() {
                write!(stdout, "{character}").unwrap();
            }
            write!(stdout, "\n").unwrap();
        }

        stdout.flush().unwrap();
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

    fn render(&mut self) {
        self.clear();
        // let vertices_to_render = self.project_vertices_on_viewport();
        // self.render_vertices();
        // self.render_lines();
        self.print_to_terminal();
    }
}

impl<'a> __RendererTrait<'a> for Terminal<'a> {
    fn new(config: RendererConfiguration) -> Self {
        let resolution = config.camera.resolution;

        let normal = config.camera.position.clone();

        // A(x - x0) + B(y - y0) + C(z - z0) = 0
        // lhs = Ax + By + Cz = Ax0 + By0 + Cz0 = D0 = rhs
        let d0 = config.camera.direction.dot(&normal);
        // if (x,y,z) is a line with origin + direction vector (t being a scalar) then:
        // lhs = At(x-x1) + Bt(y-y1) + Ct(z-z1) = D0 = rhs
        // <=>
        // lhs = Atx + Bty + Ctz - (Atx1 + Bty1 + Ctz1) = D0 = rhs
        // <=>
        // lhs = t(Ax + By + Cz) - t(Ax1 + By1 + Cz1) = D0 = rhs
        // <=>
        // lhs = t((Ax + By + Cz) - (Ax1 + By1 + Cz1)) = D0 = rhs
        // <=>
        // lhs = t = D0 / ((Ax + By + Cz) - (Ax1 + By1 + Cz1)) = rhs

        Self {
            config,
            vertices: None,
            canvas: vec![vec![character::EMPTY; resolution.0 as usize]; (resolution.1 / 2) as usize],
            canvas_line_intersection_checker: Box::new(move |point_origin, point_direction_vector| {
                let t = d0 / (
                    normal.dot(&point_direction_vector) - normal.dot(point_origin)
                );

                point_direction_vector.0.scale(t);
                let intersection_point = &point_origin.0 + &point_direction_vector.0;

                intersection_point.into()
            }),
            stdout_buffer: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{RendererConfiguration, RendererTrait};
    use super::Terminal;
}