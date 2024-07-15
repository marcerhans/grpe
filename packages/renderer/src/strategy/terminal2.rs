use std::io::BufWriter;
use std::io::{StdoutLock, Write};

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

struct Canvas {
    buffer: Vec<Vec<char>>,
    /// Return: None if no intersection found. Otherwise point at which line between vertex and viewpoint intersects the viewport.
    line_intersection_checker: Box<dyn Fn(&VectorRow<f64, 3>) -> Option<VectorRow<f64, 3>>>,
}

impl Canvas {
    /// Calculates the viewpoint position in order to fulfill the requested FOV.
    fn calc_viewpoint_position(position: &VectorRow<f64, 3>, resolution: &(u64, u64), fov: &u64) -> VectorRow<f64, 3> {
        VectorRow::<f64, 3>::from([
            position[0],
            position[1] - (resolution.0 as f64 / 2.0) / f64::tan((*fov as f64 / 2.0) * (std::f64::consts::PI / 180.0)),
            position[2],
        ])
    }

    fn new(config: &RendererConfiguration) -> Self {
        let resolution = &config.camera.resolution;

        Self { 
            buffer: vec![vec![character::EMPTY; resolution.0 as usize]; (resolution.1 / 2) as usize],
            line_intersection_checker: Self::create_intersection_checker(&config),
        }
    }
    
    /// Returns checker for line intersection with canvas plane.
    fn create_intersection_checker(config: &RendererConfiguration) -> Box<dyn Fn(&VectorRow<f64, 3>) -> Option<VectorRow<f64, 3>>> {
        Box::new({
            // Cached values for closure.
            let viewpoint = Self::calc_viewpoint_position(&config.camera.position, &config.camera.resolution, &config.camera.fov);
            let normal = VectorRow::<f64, 3>::from([0.0, 1.0, 0.0]); // This will always be true (done before rotation).
            let d0 = normal.dot(&config.camera.position);
            let d1 = normal.dot(&viewpoint); 
            let diff = d0 - d1;

            // Closure.
            // (This is based on the plane formula and the parametric form of the line from the viewpoint to a vertex).
            move |vertex_origin| {
                if diff.signum() == (d0 - normal.dot(&vertex_origin)).signum() {
                    // Ignore vertices which are on the wrong side of the viewport plane.
                    return None;
                }

                let mut viewpoint_to_vertex_direction_vector = VectorRow::from(&vertex_origin.0 - &viewpoint.0);
                let divisor = normal.dot(&viewpoint_to_vertex_direction_vector);

                if divisor.abs() < f64::EPSILON {
                    return None;
                }

                let t = diff / divisor;
                viewpoint_to_vertex_direction_vector.0.scale(t);

                Some((&viewpoint.0 + &viewpoint_to_vertex_direction_vector.0).into())
            }
        })
    }
}

/// Terminal renderer.
pub struct Terminal<'a> {
    config: RendererConfiguration,
    vertices: Option<&'a [VectorRow<f64, 3>]>,
    vertices_projected: Vec<VectorRow<f64, 3>>,
    canvas: Canvas,
    stdout_buffer: Option<BufWriter<StdoutLock<'static>>>,
}

/// This implementation can be seen as being the pipeline stages for the renderer, in the order of definitions.
impl<'a> Terminal<'a> {
    /// Get appropriate character to use for given vertical position.
    fn character_at(y: usize) -> char {
        if y % 2 != 0 {
            return character::UPPER;
        }

        character::LOWER
    }

    /// Clear the canvas buffer and the terminal screen.
    fn clear(&mut self) {
        for v in self.canvas.buffer.iter_mut() {
            for c in v.iter_mut() {
                *c = character::EMPTY;
            }
        }

        self.stdout_buffer = Some(BufWriter::new(std::io::stdout().lock()));
        write!(self.stdout_buffer.as_mut().unwrap(), "{}", ansi::GO_TO_0_0).unwrap();
    }

    /// Projects vertices ([VectorRow]) onto the plane of the viewport that is the [Camera]/[Canvas].
    fn project_vertices_on_viewport(&mut self) {
        self.vertices_projected.clear();

        for vertex in self.vertices.as_ref().unwrap().iter() {
            if let Some(intersection) = (self.canvas.line_intersection_checker)(vertex) {
                self.vertices_projected.push(intersection);
            }
        }
    }

    /// Maps projected vertices to a [Canvas::buffer].
    fn map_vertices_to_canvas_buffer(&mut self) {
        for vertex in self.vertices_projected.iter() {
            // Extract and adjust vertex position based on camera position and resolution.
            let x = (vertex[0] as isize) - self.config.camera.position[0] as isize + (self.config.camera.resolution.0 / 2 - 1) as isize;
            let mut z = vertex[2] as isize - self.config.camera.position[2] as isize + (self.config.camera.resolution.1 / 2 - 1) as isize;

            // Only show vertices within view of [Camera].
            if !(x >= 0 && x < self.config.camera.resolution.0 as isize) ||
               !(z >= 0 && z < self.config.camera.resolution.1 as isize)
            {
                continue;
            }

            // (Some z-axis gymnastics below due to terminal characters always taking two slots.)
            let mut character = Self::character_at(z as usize);
            z = z / 2;
            let buff_val = &mut self.canvas.buffer[z as usize][x as usize];

            // Is it already filled?
            if *buff_val == character::FULL {
                continue;
            }

            // Is it partially filled?
            if *buff_val == character::UPPER && character == character::LOWER {
                character = character::FULL;
            } else if *buff_val == character::LOWER && character == character::UPPER {
                character = character::FULL;
            }

            let _ = std::mem::replace(buff_val, character);
        }
    }
    
    fn render_lines(&self) {
        todo!()
    }
    
    /// Print canvas buffer to terminal.
    fn print_canvas_buffer_to_stdout(&mut self) {
        let stdout = self.stdout_buffer.as_mut().unwrap();

        for character_row in self.canvas.buffer.iter().rev() {
            for character in character_row.iter() {
                write!(stdout, "{character}").unwrap();
            }
            write!(stdout, "\n").unwrap();
        }

        stdout.flush().unwrap();
    }
}

/// General process should be:
/// 1. Setup/Update and configure.
/// 2. Rotate [Canvas] as described by [RendererConfiguration].
/// 3. Project on [Canvas].
/// 4. Rotate [Canvas] back. (This makes mapping to 2d array buffer a lot easier).
/// 5. Map to 2d array buffer ([Canvas::buffer]).
/// 6. Print array to stdout.
impl<'a> RendererTrait<'a> for Terminal<'a> {
    fn config(&self) -> RendererConfiguration {
        self.config.clone()
    }

    fn set_config(&mut self, config: RendererConfiguration) -> Result<(), &'static str> {
        self.config = config;
        self.canvas.line_intersection_checker = Canvas::create_intersection_checker(&self.config);
        Ok(())
    }

    fn set_vertices(&mut self, vertices: &'a [VectorRow<f64, 3>]) {
        self.vertices = Some(vertices);
    }

    fn set_vertices_line_draw_order(&mut self, order: &'a [&'a [usize]]) {
        todo!("Implement this later")
    }

    fn render(&mut self) {
        self.clear();
        // TODO: Rotate canvas. (Step 2).
        self.project_vertices_on_viewport();
        // TODO: Rotate canvas back to 0,0,0. (Step 4).
        self.map_vertices_to_canvas_buffer();
        self.print_canvas_buffer_to_stdout();
    }
}

impl<'a> __RendererTrait<'a> for Terminal<'a> {
    fn new(config: RendererConfiguration) -> Self {
        print!("{}{}", ansi::CLEAR_SCREEN, ansi::GO_TO_0_0);

        Self {
            vertices: None,
            vertices_projected: Vec::with_capacity((config.camera.resolution.0 * config.camera.resolution.1) as usize),
            canvas: Canvas::new(&config),
            stdout_buffer: None,
            config,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let a = 4 as f64;
        let b = 90 as f64 / 2.0;
        let c = f64::tan(b * (std::f64::consts::PI / 180.0));
        println!("foo");
    }

    #[test]
    fn canvas_line_intersection_checker_test() {
        let renderer = TerminalBuilder::default().with_camera(Camera {
            resolution: (4, 4),
            position: VectorRow::from([0.0, 0.0, 0.0]),
            fov: 90,
        }).build();
        match (renderer.canvas.line_intersection_checker)(&VectorRow::from([-2.0, 2.0, 0.0])) {
            Some(intersection_point) => assert!(intersection_point.0.eqa(&VectorRow::from([-1.0, 0.0, 0.0]).0, &0.001), "{:?}", intersection_point),
            None => panic!()
        }
    }
}