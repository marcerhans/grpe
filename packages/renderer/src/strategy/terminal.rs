use std::borrow::Borrow;
use std::cell::RefCell;
use std::io::Write;
use std::io::{BufWriter, Stdout};
use std::panic;
use std::rc::Rc;
use std::sync::OnceLock;

use crate::{
    Camera, RenderOption, RendererBuilderTrait, RendererConfiguration, RendererTrait,
    __RendererTrait,
};
use linear_algebra::{matrix::Matrix, quaternion::Quaternion, vector::VectorRow};

/// Panic hook
static PANIC_HOOK_FLAG: OnceLock<()> = OnceLock::new();

mod character {
    // pub static LINE_HORIZONTAL: char = '\u{254c}'; // ╌
    // pub static LINE_VERTICAL: char = '\u{2506}'; // ┆
    // pub static CENTER: char = '\u{253c}'; // ┼
    pub static UPPER: char = '\u{2580}'; // ▀
                                         // pub static UPPER: char = '\u{1FB91}'; // ▀
    pub static LOWER: char = '\u{2584}'; // ▄
                                         // pub static LOWER: char = '\u{1FB92}'; // ▄
    pub static FULL: char = '\u{2588}'; // █
                                        // pub static UPPER_EMPTY: char = '\u{1FB91}'; // 🮎
                                        // pub static LOWER_EMPTY: char = '\u{1FB92}'; // 🮏
                                        // pub static FULL_EMPTY: char = '\u{2592}'; // ▒
                                        // pub static EMPTY: char = '\u{2592}';
    pub static EMPTY: char = ' ';
}

mod ansi {
    pub static CLEAR_SCREEN: &str = "\x1B[2J";
    // pub static GO_TO_0_0: &str = "\x1B[H";
    pub static GO_TO_1_0: &str = "\x1B[2H";
}

#[derive(Default)]
pub struct TerminalBuilder {
    config: RendererConfiguration,
}

impl RendererBuilderTrait for TerminalBuilder {
    type Renderer = Terminal;

    fn with_camera(mut self, mut camera: Camera) -> Result<Self, &'static str> {
        Terminal::check_config_camera(&mut camera)?;
        self.config.camera = camera;
        Ok(self)
    }

    fn with_option(mut self, mut option: RenderOption) -> Result<Self, &'static str> {
        Terminal::check_config_option(&mut option)?;
        self.config.option = option;
        Ok(self)
    }

    fn build(self) -> Result<Self::Renderer, &'static str> {
        Self::Renderer::new(self.config)
    }

    fn build_with_config(
        self,
        config: RendererConfiguration,
    ) -> Result<Self::Renderer, &'static str> {
        Self::Renderer::new(config)
    }
}

struct Canvas {
    buffer: Vec<Vec<char>>,
    /// Return: None if no intersection found. Otherwise point at which line between vertex and viewpoint intersects the viewport.
    line_intersection_checker: Box<dyn Fn(&VectorRow<f64, 3>) -> Option<VectorRow<f64, 3>>>,
    rotation: Quaternion<f64>,
    rotation_inverse: Quaternion<f64>,
}

impl Canvas {
    /// Calculates the viewpoint position in order to fulfill the requested FOV.
    fn calc_viewpoint_position(
        position: &VectorRow<f64, 3>,
        resolution: &(u64, u64),
        fov: &u64,
    ) -> VectorRow<f64, 3> {
        VectorRow::<f64, 3>::from([
            position[0],
            position[1]
                - (resolution.0 as f64 / 2.0)
                    / f64::tan((*fov as f64 / 2.0) * (std::f64::consts::PI / 180.0)),
            position[2],
        ])
    }

    fn new(config: &RendererConfiguration) -> Self {
        let resolution = &config.camera.resolution;
        let (rotation, rotation_inverse) = Self::calc_rotation(&config);

        Self {
            buffer: vec![
                vec![character::EMPTY; resolution.0 as usize];
                (resolution.1 / 2) as usize
            ],
            line_intersection_checker: Self::create_intersection_checker(
                &config,
                &rotation,
                &rotation_inverse,
            ),
            rotation,
            rotation_inverse,
        }
    }

    /// Returns checker for line intersection with canvas plane.
    fn create_intersection_checker(
        config: &RendererConfiguration,
        rotation: &Quaternion<f64>,
        rotation_inverse: &Quaternion<f64>,
    ) -> Box<dyn Fn(&VectorRow<f64, 3>) -> Option<VectorRow<f64, 3>>> {
        Box::new({
            // Cached values for closure.
            // Without rotation
            let viewpoint = Self::calc_viewpoint_position(
                &config.camera.position,
                &config.camera.resolution,
                &config.camera.fov,
            );
            let normal = VectorRow::<f64, 3>::from([0.0, 1.0, 0.0]);

            // With rotation applied.
            let viewpoint: VectorRow<f64, 3> =
                (&(rotation * &viewpoint.borrow().into()) * &rotation_inverse).into();
            let normal: VectorRow<f64, 3> =
                (&(rotation * &normal.borrow().into()) * &rotation_inverse).into();
            let position: VectorRow<f64, 3> =
                (&(rotation * &config.camera.position.borrow().into()) * &rotation_inverse).into();
            let d0 = normal.dot(&position);
            let d1 = normal.dot(&viewpoint);
            let diff = d0 - d1;

            // Closure.
            // (This is based on the plane formula and the parametric form of the line from the viewpoint to a vertex).
            move |vertex_origin| {
                let mut viewpoint_to_vertex_direction_vector =
                    VectorRow::from(&vertex_origin.0 - &viewpoint.0);
                let divisor = normal.dot(&viewpoint_to_vertex_direction_vector);

                if divisor.abs() < f64::EPSILON {
                    return None;
                }

                let t = diff / divisor;

                if t < 0.0 {
                    return None;
                }

                viewpoint_to_vertex_direction_vector.0.scale(t);

                Some((&viewpoint.0 + &viewpoint_to_vertex_direction_vector.0).into())
            }
        })
    }

    fn update(&mut self, config: &RendererConfiguration) -> Result<(), &'static str> {
        let (rotation, rotation_inverse) = Self::calc_rotation(&config);
        self.rotation = rotation;
        self.rotation_inverse = rotation_inverse;
        self.line_intersection_checker =
            Canvas::create_intersection_checker(config, &self.rotation, &self.rotation_inverse);
        Ok(())
    }

    fn calc_rotation(config: &RendererConfiguration) -> (Quaternion<f64>, Quaternion<f64>) {
        let rotation = (config.camera.rotation.0 / 2.0, config.camera.rotation.1 / 2.0);

        let pitch = Quaternion {
            q0: rotation.0.cos(),
            q1: rotation.0.sin(),
            q2: 0.0,
            q3: 0.0,
        };
        let yaw = Quaternion {
            q0: rotation.1.cos(),
            q1: 0.0,
            q2: 0.0,
            q3: rotation.1.sin(),
        };
        let rotation = &pitch * &yaw;
        let rotation_inverse = Quaternion {
            q0: rotation.q0,
            q1: -rotation.q1,
            q2: -rotation.q2,
            q3: -rotation.q3,
        };

        (rotation, rotation_inverse)
    }
}

/// Terminal renderer.
/// TODO: Split config and pipeline stuff up?
/// (struct TerminalConfiguration(RendererConfiguration, OtherDerivedConfigs, canvas(?)) and struct Pipeline(vertices, vertices_projected, canvas(?), stdout_buffer)
pub struct Terminal {
    // Affected by config.
    config: RendererConfiguration,
    canvas: Canvas,

    // Pipeline stuff. Not directly affected by [Self::config].
    vertices: Option<Rc<RefCell<Vec<VectorRow<f64, 3>>>>>,
    vertices_projected: Vec<VectorRow<f64, 3>>,
    stdout_buffer: BufWriter<Stdout>,
}

/// This implementation can be seen as being the pipeline stages for the renderer, in the order of definitions.
impl Terminal {
    fn check_config_camera(camera: &mut Camera) -> Result<(), &'static str> {
        if camera.fov < 1 || camera.fov > 170 {
            return Err("FOV has to be kept in the range [1,170].");
        }

        if camera.resolution.1 % 2 != 0 {
            // Needed to protect against out of bounds.
            // I.e, when mapping from intersection/viewport plane to the array buffer
            // the indexing cannot be uneven.
            camera.resolution.1 -= 1;
        }

        // Limit angle. TODO!!!!
        // camera.rotation.0 %= std::f64::consts::PI;
        // camera.rotation.1 %= std::f64::consts::PI;

        Ok(())
    }

    fn check_config_option(_option: &mut RenderOption) -> Result<(), &'static str> {
        Ok(())
    }

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

        write!(self.stdout_buffer, "{}", ansi::GO_TO_1_0).unwrap();
    }

    /// Projects vertices ([VectorRow]) onto the plane of the viewport that is the [Camera]/[Canvas].
    fn project_vertices_on_viewport(&mut self) {
        self.vertices_projected.clear();

        let vertices = self.vertices.as_ref().unwrap().as_ref().borrow();

        for vertex in vertices.iter() {
            if let Some(intersection) = (self.canvas.line_intersection_checker)(vertex) {
                // Undo any rotation made on vertex.
                let intersection = (&(&self.canvas.rotation_inverse
                    * &intersection.borrow().into())
                    * &self.canvas.rotation)
                    .into();
                self.vertices_projected.push(intersection);
            }
        }
    }

    /// Maps projected vertices to a [Canvas::buffer].
    fn map_vertices_to_canvas_buffer(&mut self) {
        for vertex in self.vertices_projected.iter() {
            let camera = &self.config.camera;

            // Extract and adjust vertex position based on camera position and resolution (-1 for 0-indexing).
            let x = (vertex[0] as isize) - camera.position[0] as isize
                + (camera.resolution.0 / 2) as isize
                - 1;
            let mut z = vertex[2] as isize - camera.position[2] as isize
                + (camera.resolution.1 / 2) as isize
                - 1;

            // Only show vertices within view of [Camera].
            if !(x >= 0 && x < camera.resolution.0 as isize)
                || !(z >= 0 && z < camera.resolution.1 as isize)
            {
                continue;
            }

            // (Some z-axis gymnastics below due to terminal characters always taking two slots in the vertical/z-axis.)
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
        for character_row in self.canvas.buffer.iter().rev() {
            for character in character_row.iter() {
                write!(self.stdout_buffer, "{character}").unwrap();
            }
            write!(self.stdout_buffer, "\n").unwrap();
        }

        self.stdout_buffer.flush().unwrap();
    }
}

impl RendererTrait for Terminal {
    fn config(&self) -> &RendererConfiguration {
        &self.config
    }

    fn set_camera(mut self, mut camera: Camera) -> Result<Self, &'static str> {
        Terminal::check_config_camera(&mut camera)?;
        self.config.camera = camera;
        self.canvas.update(&self.config)?;
        Ok(self)
    }

    fn set_option(mut self, mut option: RenderOption) -> Result<Self, &'static str> {
        Terminal::check_config_option(&mut option)?;
        self.config.option = option;
        Ok(self)
    }

    fn set_config(mut self, config: RendererConfiguration) -> Result<Self, &'static str> {
        self = self.set_camera(config.camera)?;
        self = self.set_option(config.option)?;
        Ok(self)
    }

    fn set_vertices(&mut self, vertices: Rc<RefCell<Vec<VectorRow<f64, 3>>>>) {
        self.vertices = Some(vertices);
    }

    fn set_vertices_line_draw_order(&mut self, order: Rc<RefCell<[Box<[usize]>]>>) {
        todo!("Implement this later")
    }

    fn render(&mut self) {
        self.clear();
        self.project_vertices_on_viewport();
        self.map_vertices_to_canvas_buffer();
        self.print_canvas_buffer_to_stdout();
    }
}

impl __RendererTrait for Terminal {
    fn new(mut config: RendererConfiguration) -> Result<Self, &'static str> {
        let _ = PANIC_HOOK_FLAG.set({
            let old_hook = panic::take_hook();

            // TODO: Realized that this panic hook still will be in use even if [Terminal] is dropped. Should be fixed/considered.
            panic::set_hook(Box::new(move |panic_info| {
                println!("{}", ansi::CLEAR_SCREEN);
                old_hook(panic_info);
            }));
        });

        Terminal::check_config_camera(&mut config.camera)?;
        Terminal::check_config_option(&mut config.option)?;

        Ok(Self {
            vertices: None,
            vertices_projected: Vec::with_capacity(
                (config.camera.resolution.0 * config.camera.resolution.1) as usize,
            ),
            stdout_buffer: BufWriter::new(std::io::stdout()),
            canvas: Canvas::new(&config),
            config,
        })
    }
}
