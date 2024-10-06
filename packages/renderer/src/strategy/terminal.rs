use std::cell::RefCell;
use std::io::Write;
use std::io::{BufWriter, Stdout};
use std::panic;
use std::rc::Rc;
use std::sync::OnceLock;

use crate::{
    Camera, ProjectionMode, RenderOption, RendererBuilderTrait, RendererConfiguration,
    RendererTrait, ViewMode, __RendererTrait,
};
use linear_algebra::{quaternion::rotate, quaternion::Quaternion, vector::VectorRow};

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

// TODO: Create either a whole new struct like "CanvasPerspective" and "CanvasOrthographic". OR something like "Canvas<Perspective>" and "Canvas<Orthographic>".
struct Canvas {
    buffer: Vec<Vec<char>>,
    /// Return: None if no intersection found. Otherwise point at which line between vertex and viewpoint intersects the viewport.
    line_intersection_checker: Box<dyn Fn(&VectorRow<f64, 3>) -> Option<VectorRow<f64, 3>>>,
}

impl Canvas {
    fn new(config: &RendererConfiguration) -> Self {
        let resolution = &config.camera.resolution;

        // TODO: Fix orthographic option
        let fov = if let ProjectionMode::Perspective { fov } = config.camera.projection_mode {
            fov
        } else {
            90
        };

        Self {
            buffer: vec![
                vec![character::EMPTY; resolution.0 as usize];
                (resolution.1 / 2) as usize
            ],
            line_intersection_checker: Self::create_intersection_checker(
                &config.camera.resolution,
                &config.camera.position,
                &fov,
                &config.camera.view_mode,
                &config.camera.rotation.0,
                &config.camera.rotation.1,
            ),
        }
    }

    /// Returns checker for line intersection with canvas plane.
    fn create_intersection_checker(
        camera_resolution: &(u64, u64),
        camera_position: &VectorRow<f64, 3>,
        camera_fov: &u64,
        camera_view_mode: &ViewMode,
        rotation: &Quaternion<f64>,
        rotation_inverse: &Quaternion<f64>,
    ) -> Box<dyn Fn(&VectorRow<f64, 3>) -> Option<VectorRow<f64, 3>>> {
        Box::new({
            // Cached values for closure.
            let normal = rotate(
                &VectorRow::<f64, 3>::from([0.0, 1.0, 0.0]),
                rotation,
                rotation_inverse,
            );
            let mut viewpoint: VectorRow<f64, 3>;
            let mut viewport_origin: VectorRow<f64, 3>;

            match camera_view_mode {
                crate::ViewMode::FirstPerson => {
                    viewpoint = camera_position.clone();
                    viewport_origin = VectorRow::<f64, 3>::from([
                        0.0,
                        0.0 + (camera_resolution.0 as f64 / 2.0)
                            / f64::tan((*camera_fov as f64 / 2.0) * (std::f64::consts::PI / 180.0)),
                        0.0,
                    ]);
                    viewport_origin = rotate(&viewport_origin, rotation, rotation_inverse);
                    viewport_origin = (&viewport_origin.0 + &viewpoint.0).into();
                }
                crate::ViewMode::Orbital => {
                    viewport_origin = camera_position.clone();
                    viewpoint = VectorRow::<f64, 3>::from([
                        0.0,
                        0.0 - (camera_resolution.0 as f64 / 2.0)
                            / f64::tan((*camera_fov as f64 / 2.0) * (std::f64::consts::PI / 180.0)),
                        0.0,
                    ]);
                    viewpoint = rotate(&viewpoint, rotation, rotation_inverse);
                    viewpoint = (&viewpoint.0 + &viewport_origin.0).into();
                }
            }

            let d0 = normal.dot(&viewport_origin);
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
        let resolution = config.camera.resolution;

        if self.buffer.len() != (resolution.1 as usize)
            || self.buffer[0].len() != (resolution.0 as usize)
        {
            self.buffer =
                vec![vec![character::EMPTY; resolution.0 as usize]; (resolution.1 / 2) as usize];
        }

        // TODO: Fix orthographic option
        let fov = if let ProjectionMode::Perspective { fov } = config.camera.projection_mode {
            fov
        } else {
            90
        };

        self.line_intersection_checker = Self::create_intersection_checker(
            &config.camera.resolution,
            &config.camera.position,
            &fov,
            &config.camera.view_mode,
            &config.camera.rotation.0,
            &config.camera.rotation.1,
        );
        Ok(())
    }
}

/// Extra settings specific to [Terminal].
#[derive(Clone)]
pub struct TerminalExtras {
    pub pixel_width_scaling: f64,
    pub pixel_height_scaling: f64,
}

impl Default for TerminalExtras {
    fn default() -> Self {
        Self {
            pixel_width_scaling: 1.0,
            pixel_height_scaling: 1.0,
        }
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
    vertices_projected: Vec<Option<VectorRow<f64, 3>>>,
    line_draw_order: Option<Rc<RefCell<Vec<Vec<usize>>>>>,
    stdout_buffer: BufWriter<Stdout>,

    // Extra
    extras: TerminalExtras,
}

/// This implementation can be seen as being the pipeline stages for the renderer, in the order of definitions.
impl Terminal {
    pub fn clear_screen(&self) {
        print!("{}", ansi::CLEAR_SCREEN);
    }

    pub fn set_extras(&mut self, extras: TerminalExtras) {
        self.extras = extras;
    }

    pub fn extras(&self) -> &TerminalExtras {
        &self.extras
    }

    /// Check all members of given [Camera].
    /// Unchecked fields shall simply have a comment.
    fn check_config_camera(camera: &mut Camera) -> Result<(), &'static str> {
        // Resolution
        if camera.resolution.1 % 2 != 0 {
            // Needed to protect against out of bounds.
            // I.e, when mapping from intersection/viewport plane to the array buffer
            // the indexing cannot be uneven.
            camera.resolution.1 -= 1;
        }

        // Position
        // Rotation
        // ViewMode

        // ProjectionMode
        if let ProjectionMode::Perspective { fov } = camera.projection_mode {
            if fov < 1 || fov > 170 {
                return Err("FOV has to be kept in the range [1,170].");
            }
        }

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

    fn render_pixel(buffer: &mut Vec<Vec<char>>, camera: &Camera, x: isize, z: isize) {
        // Extract and adjust position based on camera resolution.
        let x = x + (camera.resolution.0 / 2) as isize;
        let mut z = z + (camera.resolution.1 / 2) as isize;

        // (Some z-axis gymnastics below due to terminal characters always taking two slots in the vertical/z-axis.)
        let mut character = Self::character_at(z as usize);
        z = z / 2;
        let buff_val = &mut buffer[z as usize][x as usize];

        if *buff_val == character::FULL {
            // Is it already filled.
            return;
        } else if *buff_val == character::UPPER && character == character::LOWER {
            character = character::FULL;
        } else if *buff_val == character::LOWER && character == character::UPPER {
            character = character::FULL;
        }

        *buff_val = character;
    }

    /// Projects vertices ([VectorRow]) onto the plane of the viewport that is the [Camera]/[Canvas].
    fn project_vertices_on_viewport(&mut self) {
        let vertices = self.vertices.as_ref().unwrap().as_ref().borrow();
        self.vertices_projected.fill(None);

        let x_min = -((self.config.camera.resolution.0 / 2) as isize);
        let x_max = ((self.config.camera.resolution.0 / 2) as isize) - 1; // 0 included as positive
        let z_min = -((self.config.camera.resolution.1 / 2) as isize);
        let z_max = ((self.config.camera.resolution.1 / 2) as isize) - 1; // 0 included as positive

        for (index, vertex) in vertices.iter().enumerate() {
            if let Some(intersection) = (self.canvas.line_intersection_checker)(vertex) {
                // Undo any previously applied rotation.
                let mut intersection =
                    VectorRow::<f64, 3>::from(&intersection.0 - &self.config.camera.position.0);
                intersection = rotate(
                    &intersection,
                    &self.config.camera.rotation.1,
                    &self.config.camera.rotation.0,
                );

                // Adjust points according to width heigh pixel ratio.
                intersection[0] = intersection[0] * self.extras.pixel_width_scaling;
                intersection[2] = intersection[2] * self.extras.pixel_height_scaling;

                // Do not store the point if it is outside of visible viewport space.
                if !(((intersection[0] as isize) >= x_min && (intersection[0] as isize) < x_max)
                    && ((intersection[2] as isize) >= z_min && (intersection[2] as isize) < z_max))
                {
                    continue;
                }

                self.vertices_projected[index] = Some(intersection);
            }
        }
    }

    /// Maps projected vertices to a [Canvas::buffer].
    fn render_projected_vertices(&mut self) {
        for vertex in self.vertices_projected.iter() {
            if let Some(vertex) = vertex {
                Self::render_pixel(
                    &mut self.canvas.buffer,
                    &self.config.camera,
                    vertex[0] as isize,
                    vertex[2] as isize,
                );
            }
        }
    }

    /// Maps lines between vertices to a [Canvas::buffer].
    /// Note: This method is a bit all over the place.
    fn render_lines_and_particles(&mut self, culling: bool) {
        fn render_lines(
            order: &[usize],
            vertices_projected: &Vec<Option<VectorRow<f64, 3>>>,
            buffer: &mut Vec<Vec<char>>,
            camera: &Camera,
        ) {
            // While projected vertices have been filtered to only keep the ones within camera view;
            // with the currently impelemntation for drawing lines, some parts of line(s) can get
            // outside allowed boundaries.
            // TODO: Should not be an issue. Write better code.
            let x_min = -((camera.resolution.0 / 2) as isize);
            let x_max = ((camera.resolution.0 / 2) as isize) - 1; // 0 included as positive
            let z_min = -((camera.resolution.1 / 2) as isize);
            let z_max = ((camera.resolution.1 / 2) as isize) - 1; // 0 included as positive

            for ab in order.windows(2) {
                if let (Some(a), Some(b)) = (&vertices_projected[ab[0]], &vertices_projected[ab[1]])
                {
                    let dx = (a[0] - b[0]) as isize;
                    let dz = (a[2] - b[2]) as isize;
                    let dx_sign = dx.signum();
                    let dz_sign = dz.signum();
                    let dx = dx.abs();
                    let dz = dz.abs();

                    let (
                        large_base,
                        small_base,
                        large_diff,
                        small_diff,
                        large_direction,
                        small_direction,
                        swap,
                    ) = if dx >= dz {
                        (
                            a[0] as isize,
                            a[2] as isize,
                            dx,
                            dz,
                            -dx_sign,
                            -dz_sign,
                            true,
                        )
                    } else {
                        (
                            a[2] as isize,
                            a[0] as isize,
                            dz,
                            dx,
                            -dz_sign,
                            -dx_sign,
                            false,
                        )
                    };

                    if small_diff == 0 {
                        // Just draw straight line.
                        for large_step in 0..=large_diff {
                            if !swap {
                                let x = small_base;
                                let z = large_base + large_direction * large_step;
                                if x < x_min || x > x_max || z < z_min || z > z_max {
                                    continue;
                                }
                                Terminal::render_pixel(buffer, camera, x, z);
                            } else {
                                let x = large_base + large_direction * large_step;
                                let z = small_base;
                                if x < x_min || x > x_max || z < z_min || z > z_max {
                                    continue;
                                }
                                Terminal::render_pixel(buffer, camera, x, z);
                            }
                        }
                        continue;
                    }

                    let ratio = large_diff as f64 / small_diff as f64;
                    let mut small_step;

                    for large_step in 0..=large_diff {
                        small_step = (large_step as f64 / ratio) as isize;

                        if !swap {
                            let x = small_base + small_direction * small_step;
                            let z = large_base + large_direction * large_step;
                            if x < x_min || x > x_max || z < z_min || z > z_max {
                                continue;
                            }
                            Terminal::render_pixel(buffer, camera, x, z);
                        } else {
                            let x = large_base + large_direction * large_step;
                            let z = small_base + small_direction * small_step;
                            if x < x_min || x > x_max || z < z_min || z > z_max {
                                continue;
                            }
                            Terminal::render_pixel(buffer, camera, x, z);
                        }
                    }
                }
            }
        }

        let line_draw_order = self.line_draw_order.as_ref().unwrap().as_ref().borrow();
        let camera_normal = &VectorRow::from([0.0, 1.0, 0.0]);

        for order in line_draw_order.iter() {
            if let RenderOption::WireFrameAndParticles | RenderOption::CullingAndParticles =
                self.config.option
            {
                if order.len() == 1 {
                    // Render as single point particle.
                    if let Some(particle) = &self.vertices_projected[order[0]] {
                        Self::render_pixel(
                            &mut self.canvas.buffer,
                            &self.config.camera,
                            particle[0] as isize,
                            particle[2] as isize,
                        );
                    }
                    continue;
                } else if order.len() == 2 {
                    render_lines(
                        order,
                        &self.vertices_projected,
                        &mut self.canvas.buffer,
                        &self.config.camera,
                    );
                    continue;
                }
            }

            if order.len() < 3 {
                continue;
            }

            if culling {
                let mut order_culled: Vec<usize> = vec![];

                // Determine if the face should be culled.
                for abc in order.windows(3) {
                    if let (Some(a), Some(b), Some(c)) = (
                        &self.vertices_projected[abc[0]],
                        &self.vertices_projected[abc[1]],
                        &self.vertices_projected[abc[2]],
                    ) {
                        // Span two vectors stemming from the same point. This implies CCW rotation.
                        let v_a: VectorRow<f64, 3> = (&b.0 - &a.0).into();
                        let v_b: VectorRow<f64, 3> = (&c.0 - &a.0).into();

                        // Calculate the cross product.
                        let normal = v_a.cross(&v_b);

                        // Calculate angle between cross product and the [Camera] normal in order to decide if it should be culled.
                        let camera_normal_magnitude = 1.0; // We already know this due to just rotating existing length of 1.0.
                        let normal_magnitude =
                            (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2])
                                .sqrt();
                        let cos_angle = camera_normal.dot(&normal)
                            / (camera_normal_magnitude * normal_magnitude);

                        if cos_angle <= 0.0 {
                            order_culled.append(&mut abc.to_vec());
                            // order_culled.append(&mut order.to_vec()); // SLOOOOOOOWWWW!!!!
                        }
                    }
                }
                render_lines(
                    &order_culled,
                    &self.vertices_projected,
                    &mut self.canvas.buffer,
                    &self.config.camera,
                );
            } else {
                render_lines(
                    order,
                    &self.vertices_projected,
                    &mut self.canvas.buffer,
                    &self.config.camera,
                );
            }
        }
    }

    /// Print canvas buffer to terminal.
    fn write_rendered_scene_to_stdout_buffer(&mut self) {
        for character_row in self.canvas.buffer.iter().rev() {
            for character in character_row.iter() {
                write!(self.stdout_buffer, "{character}").unwrap();
            }
            write!(self.stdout_buffer, "\n").unwrap();
        }
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
        let len = self.vertices.as_ref().unwrap().borrow().len();
        self.vertices_projected.resize(len, None);
    }

    fn set_vertices_line_draw_order(&mut self, order: Rc<RefCell<Vec<Vec<usize>>>>) {
        self.line_draw_order = Some(order);
    }

    fn render(&mut self) {
        self.clear();
        self.project_vertices_on_viewport();

        match self.config.option {
            RenderOption::Vertices => self.render_projected_vertices(),
            RenderOption::WireFrame | RenderOption::WireFrameAndParticles => {
                self.render_lines_and_particles(false)
            }
            RenderOption::Culling | RenderOption::CullingAndParticles => {
                self.render_lines_and_particles(true)
            }
        }

        self.write_rendered_scene_to_stdout_buffer();
        self.stdout_buffer.flush().unwrap();
    }
}

impl __RendererTrait for Terminal {
    fn new(mut config: RendererConfiguration) -> Result<Self, &'static str> {
        println!("\x1B[?1049h"); // Enter alternative buffer mode. I.e., do not affect previous terminal history.

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
            vertices_projected: Vec::new(),
            stdout_buffer: BufWriter::new(std::io::stdout()),
            line_draw_order: None,
            canvas: Canvas::new(&config),
            config,
            extras: TerminalExtras::default(),
        })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        println!("\x1B[?1049l");
    }
}
