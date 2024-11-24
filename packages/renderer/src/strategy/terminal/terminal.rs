use std::io::Write;
use std::rc::Rc;
use std::{cell::RefCell, ptr::write};

use super::buffer::*;
use crate::{
    Camera, ProjectionMode, RenderOption, RendererBuilderTrait, RendererConfiguration,
    RendererTrait, ViewMode, __RendererTrait,
};
use linear_algebra::{quaternion::rotate, quaternion::Quaternion, vector::VectorRow};

struct Canvas {
    buffer: TerminalBuffer,
    /// Returns [None] if no intersection is found. Otherwise point at which line between vertex and viewpoint intersects the viewport, and it's depth.
    line_intersection_checker: Box<dyn Fn(&VectorRow<f64, 3>) -> Option<(VectorRow<f64, 3>, f64)>>,
}

impl Canvas {
    fn new(config: &RendererConfiguration) -> Self {
        // TODO: Fix orthographic option
        let fov = if let ProjectionMode::Perspective { fov } = config.camera.projection_mode {
            fov
        } else {
            90
        };

        Self {
            buffer: TerminalBuffer::new(&config.camera.resolution),
            line_intersection_checker: Self::create_intersection_checker(
                &config.camera.resolution,
                config.camera.position.clone(),
                fov,
                &config.camera.view_mode,
                config.camera.rotation.0.clone(),
                config.camera.rotation.1.clone(),
            ),
        }
    }

    /// Returns checker for line intersection with canvas plane.
    fn create_intersection_checker(
        camera_resolution: &(u64, u64),
        camera_position: VectorRow<f64, 3>,
        camera_fov: u64,
        camera_view_mode: &ViewMode,
        rotation: Quaternion<f64>,
        rotation_inverse: Quaternion<f64>,
    ) -> Box<dyn Fn(&VectorRow<f64, 3>) -> Option<(VectorRow<f64, 3>, f64)>> {
        Box::new({
            // Cached values for closure.
            let normal = VectorRow::<f64, 3>::from([0.0, 1.0, 0.0]);
            let mut viewpoint: VectorRow<f64, 3> = VectorRow::from([0.0, 0.0, 0.0]);
            let mut viewport_origin: VectorRow<f64, 3> = VectorRow::from([0.0, 0.0, 0.0]);

            match camera_view_mode {
                crate::ViewMode::FirstPerson => {
                    viewport_origin = VectorRow::<f64, 3>::from([
                        0.0,
                        0.0 + (camera_resolution.0 as f64 / 2.0)
                            / f64::tan((camera_fov as f64 / 2.0) * (std::f64::consts::PI / 180.0)),
                        0.0,
                    ]);
                }
                crate::ViewMode::Orbital => {
                    viewpoint = VectorRow::<f64, 3>::from([
                        0.0,
                        0.0 - (camera_resolution.0 as f64 / 2.0)
                            / f64::tan((camera_fov as f64 / 2.0) * (std::f64::consts::PI / 180.0)),
                        0.0,
                    ]);
                }
            }

            let d0 = normal.dot(&viewport_origin);
            let d1 = normal.dot(&viewpoint);
            let diff = d0 - d1;

            // Closure.
            // (This is based on the plane formula and the parametric form of the line from the viewpoint to a vertex).
            move |vertex_origin| {
                let vertex: VectorRow<f64, 3> = (&vertex_origin.0 - &camera_position.0).into();
                let vertex = rotate(&vertex, &rotation_inverse, &rotation);
                let mut viewpoint_to_vertex_direction_vector =
                    VectorRow::from(&vertex.0 - &viewpoint.0);
                let divisor = normal.dot(&viewpoint_to_vertex_direction_vector);

                if divisor.abs() < f64::EPSILON {
                    return None;
                }

                let t = diff / divisor;

                if t < 0.0 {
                    return None;
                }

                viewpoint_to_vertex_direction_vector.0.scale(t);

                Some((
                    (&viewpoint.0 + &viewpoint_to_vertex_direction_vector.0).into(),
                    vertex[1],
                ))
            }
        })
    }

    fn update(&mut self, config: &RendererConfiguration) -> Result<(), &'static str> {
        let resolution = config.camera.resolution;
        let len = TerminalBuffer::pixels_required(&resolution);

        if self.buffer.data().len() != len {
            self.buffer = TerminalBuffer::new(&resolution);
        }

        // TODO: Fix orthographic option
        let fov = if let ProjectionMode::Perspective { fov } = config.camera.projection_mode {
            fov
        } else {
            90
        };

        self.line_intersection_checker = Self::create_intersection_checker(
            &config.camera.resolution,
            config.camera.position.clone(),
            fov,
            &config.camera.view_mode,
            config.camera.rotation.0.clone(),
            config.camera.rotation.1.clone(),
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

/// Terminal renderer.
pub struct Terminal {
    // Affected by config.
    config: RendererConfiguration,
    canvas: Canvas,

    // Pipeline stuff. Not directly affected by [Self::config].
    vertices: Option<Rc<RefCell<Vec<VectorRow<f64, 3>>>>>,
    vertices_projected: Vec<Option<VectorRow<f64, 3>>>,
    line_draw_order: Option<Rc<RefCell<Vec<Vec<usize>>>>>,

    // Extra
    extras: TerminalExtras,
}

/// This implementation can be seen as being the pipeline stages for the renderer, in the order of definitions.
impl Terminal {
    pub fn clear_screen(&self) {
        print!("\x1B[2J");
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

    /// Clear the canvas buffer and the terminal screen.
    fn clear(&mut self) {
        self.canvas.buffer.clear();
        print!("\x1B[2H"); // Move to row 1 (zero indexed).
    }

    fn render_pixel(
        buffer: &mut TerminalBuffer,
        camera: &Camera,
        x: isize,
        y: f64,
        z: isize,
        polygon_border: bool,
    ) {
        // Extract and adjust position based on camera resolution.
        let x = x + (camera.resolution.0 / 2) as isize;
        let mut z = z + (camera.resolution.1 / 2) as isize;

        // (Some z-axis gymnastics below due to terminal characters always taking two slots in the vertical/z-axis.)
        let mut character = pixel::Value::at(z as usize);
        z = z / 2;
        let pixel = buffer.pixel_mut(
            camera.resolution.1 as usize / 2 - z as usize - 1,
            x as usize,
        ); // TODO: Ugly fix for the buffer being upside down.

        // Update depth.
        let current_depth;

        match character {
            pixel::Value::Upper => current_depth = &mut pixel.depth.0,
            pixel::Value::Lower => current_depth = &mut pixel.depth.1,
            _ => unreachable!(),
        }

        if polygon_border {
            // Store depth temporarily for later processing.
            match character {
                pixel::Value::Upper => pixel.polygon_border.0 = Some(y),
                pixel::Value::Lower => pixel.polygon_border.1 = Some(y),
                _ => unreachable!(),
            }
        }

        if let Some(current_depth) = current_depth {
            if *current_depth > y {
                *current_depth = y;
            } else {
                return;
            }
        } else {
            *current_depth = Some(y);
        }

        // Update character.
        let pixel_value = pixel.value();

        if pixel_value == pixel::Value::Full.value() {
            // Already filled.
            return;
        } else if (pixel_value == pixel::Value::Upper.value() && character == pixel::Value::Lower)
            || (pixel_value == pixel::Value::Lower.value() && character == pixel::Value::Upper)
        {
            character = pixel::Value::Full;
        }

        pixel.set_value(character);
    }

    /// Projects vertices ([VectorRow]) onto the plane of the viewport that is the [Camera]/[Canvas].
    fn project_vertices_on_viewport(&mut self) {
        let vertices = self.vertices.as_ref().unwrap().as_ref().borrow();
        self.vertices_projected.fill(None);

        let x_min = -((self.config.camera.resolution.0 / 2) as isize);
        let x_max = (self.config.camera.resolution.0 / 2) as isize;
        let z_min = -((self.config.camera.resolution.1 / 2) as isize);
        let z_max = (self.config.camera.resolution.1 / 2) as isize;

        for (index, vertex) in vertices.iter().enumerate() {
            if let Some((mut intersection, depth)) = (self.canvas.line_intersection_checker)(vertex)
            {
                // Adjust points according to width heigh pixel ratio.
                intersection[0] = intersection[0] * self.extras.pixel_width_scaling;
                intersection[2] = intersection[2] * self.extras.pixel_height_scaling;

                // Do not store the point if it is outside of visible viewport space.
                if !(((intersection[0] as isize) >= x_min && (intersection[0] as isize) < x_max)
                    && ((intersection[2] as isize) >= z_min && (intersection[2] as isize) < z_max))
                {
                    continue;
                }

                // Store depth information.
                intersection[1] = depth;

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
                    vertex[1],
                    vertex[2] as isize,
                    false,
                );
            }
        }
    }

    /// This method renders:
    /// - Particles (single points + lines).
    /// - Wireframe lines (lines between vertices of a face).
    /// - Filled polygons (Technical detail: Will utilize depth buffer fully).
    /// Note: This method is doing a bit too much. Might need refactoring soon.
    fn render_entities(&mut self, culling: bool, polyfill: bool) {
        if polyfill {
            assert!(culling);
        }

        #[inline]
        fn interpolate_depth(start: f64, end: f64, steps_max: f64, step_current: f64) -> f64 {
            let div = step_current / steps_max;
            (1.0 - div) * start + div * end
        }

        fn render_lines(
            order: &[usize],
            vertices_projected: &Vec<Option<VectorRow<f64, 3>>>,
            buffer: &mut TerminalBuffer,
            camera: &Camera,
            polygon_border: bool,
        ) {
            #[inline]
            fn render_line(
                buffer: &mut TerminalBuffer,
                camera: &Camera,
                polygon_border: bool,
                a: &VectorRow<f64, 3>,
                b: &VectorRow<f64, 3>,
            ) {
                let mut x0 = a[0] as isize;
                let x1 = b[0] as isize;
                let mut z0 = a[2] as isize;
                let z1 = b[2] as isize;

                let dx = (x1 - x0).abs();
                let dz = (z1 - z0).abs();

                let sx = (x1 - x0).signum();
                let sz = (z1 - z0).signum();

                let mut err = dx - dz;
                let steps_max = (b[0] - a[0]).abs() + (b[2] - a[2]).abs();
                let mut steps_taken = 0;

                while x0 != x1 || z0 != z1 {
                    Terminal::render_pixel(
                        buffer,
                        camera,
                        x0,
                        interpolate_depth(a[1], b[1], steps_max, steps_taken as f64),
                        z0,
                        polygon_border,
                    );

                    let e2 = 2 * err;

                    if e2 > -dz {
                        err = err - dz;
                        x0 = x0 + sx;
                    }

                    if e2 < dx {
                        err = err + dx;
                        z0 = z0 + sz;
                    }

                    steps_taken += 1;
                }
            }

            for ab in order.windows(2) {
                if let (Some(a), Some(b)) = (&vertices_projected[ab[0]], &vertices_projected[ab[1]])
                {
                    render_line(buffer, camera, polygon_border, a, b);
                }
            }

            if order.len() > 2 {
                // Close the polygon by drawing line between start and end vertices.
                if let (Some(a), Some(b)) = (
                    &vertices_projected[order[order.len() - 1]],
                    &vertices_projected[order[0]],
                ) {
                    render_line(buffer, camera, polygon_border, a, b);
                }
            }
        }

        let line_draw_order = self.line_draw_order.as_ref().unwrap().as_ref().borrow();

        'outer: for order in line_draw_order.iter() {
            if let RenderOption::WireFrameAndParticles
            | RenderOption::CullingAndParticles
            | RenderOption::PolyfillAndCullingAndParticles = self.config.option
            {
                if order.len() == 1 {
                    // Render as single point particle.
                    if let Some(particle) = &self.vertices_projected[order[0]] {
                        Self::render_pixel(
                            &mut self.canvas.buffer,
                            &self.config.camera,
                            particle[0] as isize,
                            particle[1],
                            particle[2] as isize,
                            false,
                        );
                    }
                    continue;
                } else if order.len() == 2 {
                    render_lines(
                        order,
                        &self.vertices_projected,
                        &mut self.canvas.buffer,
                        &self.config.camera,
                        false,
                    );
                    continue;
                }
            }

            if order.len() < 3 {
                continue;
            }

            if culling {
                // Determine if the face should be culled.
                let mut bottom_right: Option<(usize, &VectorRow<f64, 3>)> = None;

                for index in 0..order.len() {
                    if let Some(vertex) = &self.vertices_projected[order[index]].as_ref() {
                        if bottom_right.is_none() {
                            bottom_right = Some((index, vertex));
                            continue;
                        } else if let Some(bottom_right) = &mut bottom_right {
                            if vertex[2] < bottom_right.1[2] {
                                *bottom_right = (index, vertex);
                            } else if vertex[2] == bottom_right.1[2] {
                                if vertex[0] > bottom_right.1[0] {
                                    *bottom_right = (index, vertex);
                                }
                            }
                        }
                    } else {
                        continue 'outer;
                    }
                }

                let bottom_right = bottom_right.unwrap();
                let next_index = (bottom_right.0 + 1) % order.len();
                let prev_index = bottom_right.0.checked_sub(1).unwrap_or(order.len() - 1);

                let current_to_next: VectorRow<f64, 3> = (&self.vertices_projected
                    [order[next_index]]
                    .as_ref()
                    .unwrap()
                    .0
                    - &bottom_right.1 .0)
                    .into();
                let current_to_prev: VectorRow<f64, 3> = (&self.vertices_projected
                    [order[prev_index]]
                    .as_ref()
                    .unwrap()
                    .0
                    - &bottom_right.1 .0)
                    .into();
                let should_be_culled = current_to_next.cross(&current_to_prev)[1] > 0.0;

                if should_be_culled {
                    continue;
                }

                render_lines(
                    &order,
                    &self.vertices_projected,
                    &mut self.canvas.buffer,
                    &self.config.camera,
                    true,
                );

                if polyfill {
                    // Save some performance by only doing polyfill if face was not culled.
                    // Filter out only relevant vertices.
                    let vertices = order
                        .iter()
                        .filter_map(|&index| self.vertices_projected[index].as_ref())
                        .collect::<Vec<&VectorRow<f64, 3>>>();

                    // Generate bounds/ranges.
                    let iter_x = vertices.iter().map(|vertex| vertex[0] as isize);
                    let start_x = iter_x.clone().min().unwrap();
                    let end_x = iter_x.clone().max().unwrap();

                    let iter_z = vertices.iter().map(|vertex| vertex[2] as isize);
                    let start_z = iter_z.clone().min().unwrap();
                    let end_z = iter_z.clone().max().unwrap();

                    // Scan from "bottom-left" to "top-right" and fill polygon.
                    for z in (start_z..=end_z).step_by(2) {
                        let mut start_upper: Option<isize> = None;
                        let mut start_lower: Option<isize> = None;

                        for x in start_x..=end_x {
                            // Extract and adjust position based on camera resolution.
                            let x = x + (self.config.camera.resolution.0 / 2) as isize;
                            let z = (z + (self.config.camera.resolution.1 / 2) as isize) / 2;
                            let z = self.config.camera.resolution.1 as usize / 2 - z as usize - 1;
                            let polygon_border =
                                self.canvas.buffer.pixel(z, x as usize).polygon_border;

                            if let Some(depth_end) = polygon_border.0 {
                                if let Some(start) = start_upper.as_mut() {
                                    let depth_start = self
                                        .canvas
                                        .buffer
                                        .pixel(z, *start as usize)
                                        .polygon_border
                                        .0
                                        .unwrap();
                                    let steps_max = (x - *start) as f64;
                                    let mut steps_taken = 1; // Start at 1, because we skip first.

                                    for step in (*start..x).skip(1) {
                                        let depth_new = interpolate_depth(
                                            depth_start,
                                            depth_end,
                                            steps_max,
                                            steps_taken as f64,
                                        );
                                        let pixel = self.canvas.buffer.pixel_mut(z, step as usize);

                                        if let Some(depth_old) = pixel.depth.0 {
                                            if pixel.polygon_border.0.is_none()
                                                && depth_old > depth_new
                                            {
                                                // Fill with empty space.
                                                if pixel.value() == pixel::Value::Upper.value() {
                                                    pixel.set_value(pixel::Value::Empty);
                                                } else if pixel.value()
                                                    == pixel::Value::Full.value()
                                                {
                                                    pixel.set_value(pixel::Value::Lower);
                                                }

                                                pixel.depth.0 = Some(depth_new);
                                            }
                                        } else {
                                            pixel.depth.0 = Some(depth_new);
                                        }

                                        // if pixel.value() == pixel::Value::Empty.value() {
                                        //     pixel.set_value(pixel::Value::Upper);
                                        // } else if pixel.value() == pixel::Value::Lower.value() {
                                        //     pixel.set_value(pixel::Value::Full);
                                        // }

                                        steps_taken += 1;
                                    }
                                }

                                start_upper = Some(x);
                            }

                            if let Some(depth_end) = polygon_border.1 {
                                if let Some(start) = start_lower.as_mut() {
                                    let depth_start = self
                                        .canvas
                                        .buffer
                                        .pixel(z, *start as usize)
                                        .polygon_border
                                        .1
                                        .unwrap();
                                    let steps_max = (x - *start) as f64;
                                    let mut steps_taken = 1.0; // Start at 1, because we skip first.

                                    for step in (*start..x).skip(1) {
                                        let depth_new = interpolate_depth(
                                            depth_start,
                                            depth_end,
                                            steps_max,
                                            steps_taken,
                                        );
                                        let pixel = self.canvas.buffer.pixel_mut(z, step as usize);

                                        if let Some(depth_old) = pixel.depth.1 {
                                            if pixel.polygon_border.1.is_none()
                                                && depth_old > depth_new
                                            {
                                                // Fill with empty space.
                                                if pixel.value() == pixel::Value::Lower.value() {
                                                    pixel.set_value(pixel::Value::Empty);
                                                } else if pixel.value()
                                                    == pixel::Value::Full.value()
                                                {
                                                    pixel.set_value(pixel::Value::Upper);
                                                }

                                                pixel.depth.1 = Some(depth_new);
                                            }
                                        } else {
                                            pixel.depth.1 = Some(depth_new);
                                        }

                                        // if pixel.value() == pixel::Value::Empty.value() {
                                        //     pixel.set_value(pixel::Value::Lower);
                                        // } else if pixel.value() == pixel::Value::Upper.value() {
                                        //     pixel.set_value(pixel::Value::Full);
                                        // }

                                        steps_taken += 1.0;
                                    }
                                }

                                start_lower = Some(x);
                            }
                        }

                        // Clear border flags.
                        for x in start_x..=end_x {
                            // Extract and adjust position based on camera resolution.
                            let x = x + (self.config.camera.resolution.0 / 2) as isize;
                            let z = (z + (self.config.camera.resolution.1 / 2) as isize) / 2;
                            let z = self.config.camera.resolution.1 as usize / 2 - z as usize - 1;
                            self.canvas.buffer.pixel_mut(z, x as usize).polygon_border =
                                (None, None);
                        }
                    }
                }
            } else {
                render_lines(
                    order,
                    &self.vertices_projected,
                    &mut self.canvas.buffer,
                    &self.config.camera,
                    false,
                );
            }
        }
    }

    /// Print canvas buffer to terminal.
    fn write_rendered_scene_to_stdout(&mut self) {
        std::io::stdout()
            .write_all(
                &self
                    .canvas
                    .buffer
                    .data()
                    .iter()
                    .collect::<String>()
                    .as_bytes(),
            )
            .expect("Failed to write to stdout");

        // std::io::stdout()
        //     .write_all(&self.canvas.buffer.data())
        //     .expect("Failed to write to stdout.");
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
                self.render_entities(false, false)
            }
            RenderOption::Culling | RenderOption::CullingAndParticles => {
                self.render_entities(true, false)
            }
            RenderOption::PolyfillAndCulling | RenderOption::PolyfillAndCullingAndParticles => {
                self.render_entities(true, true)
            }
        }

        self.write_rendered_scene_to_stdout();
    }
}

impl __RendererTrait for Terminal {
    fn new(mut config: RendererConfiguration) -> Result<Self, &'static str> {
        println!("\x1B[?1049h"); // Enter alternative buffer mode. I.e., do not affect previous terminal history.

        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            println!("\x1B[?1049l");
            prev_hook(info);
        }));

        Terminal::check_config_camera(&mut config.camera)?;
        Terminal::check_config_option(&mut config.option)?;

        Ok(Self {
            vertices: None,
            vertices_projected: Vec::new(),
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
