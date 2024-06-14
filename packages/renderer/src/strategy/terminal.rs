use std::{borrow::BorrowMut, cell::RefCell};

use linear_algebra::{
    matrix::{Matrix, MatrixDataTrait},
    utility::gauss_elimination,
};

use crate::{
    common::*, Camera, RenderOption, RendererBuilderTrait, RendererConfiguration, RendererTrait,
    __RendererTrait,
};

mod character {
    pub static LINE_HORIZONTAL: char = '\u{254c}'; // ╌
    pub static LINE_VERTICAL: char = '\u{2506}'; // ┆
    pub static CENTER: char = '\u{253c}'; // ┼
    pub static UPPER: char = '\u{2580}'; // ▀
    pub static LOWER: char = '\u{2584}'; // ▄
    pub static FULL: char = '\u{2588}'; // █
    pub static UPPER_EMPTY: char = '\u{1FB8E}'; // 🮎
    pub static LOWER_EMPTY: char = '\u{1FB8F}'; // 🮏
    pub static FULL_EMPTY: char = '\u{2592}'; // ▒
    pub static EMPTY: char = ' '; // ▒
}

pub struct TerminalBuilder {
    config: RendererConfiguration,
}

impl<'a> Default for TerminalBuilder {
    fn default() -> Self {
        Self {
            config: RendererConfiguration::default(),
        }
    }
}

impl RendererBuilderTrait<f64> for TerminalBuilder {
    type Renderer = Terminal<f64>;

    fn man() -> &'static str {
        todo!()
    }

    fn with_camera(mut self, camera: Camera<f64>) -> Self {
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

    fn build_with_config(self, config: crate::RendererConfiguration) -> Self::Renderer {
        Self::Renderer::new(config)
    }
}

struct DerivedConfiguration<T: MatrixDataTrait> {
    /// Position
    viewpoint: Matrix<T>,

    /// The viewport is a plane described by a parametric form, where each row describes:
    /// - Position
    /// - Parameter vector A
    /// - Parameter vector B
    viewport: Matrix<T>,
}

impl DerivedConfiguration<f64> {
    /// Derived configuration determining the [Self::viewpoint] and [Self::viewport]
    /// based on given configuration. Mainly determined by the cameras position and FOV.
    fn new<'a>(config: &RendererConfiguration) -> Self {
        let position = config.camera.position();
        let direction = config.camera.direction();
        let fov = config.camera.fov();

        let parametric_form = DerivedConfiguration::normal_to_parametric_form(direction, position);

        let viewport = Matrix::from_slice::<3, 3>(&[
            &parametric_form.data()[0..3].try_into().unwrap(), // Point
            &parametric_form.data()[3..6].try_into().unwrap(), // Parameter a
            &parametric_form.data()[6..9].try_into().unwrap(), // Parameter b
        ]);

        // TODO: Fix for FOV later :) Just use static value for now. 10 "units" back based on normal and center of viewport.
        let mut direction = direction.clone();
        direction.scalar(-10.0);
        let viewpoint =  position - &direction;

        Self {
            viewpoint,
            viewport,
        }
    }

    /// Rotation of camera is not implemented yet, assume rotation is 0 degrees.
    /// TODO: THIS ONLY WORKS IF THE CAMERA DOES NOT ROTATE MORE THAN 90 DEGREES!...
    /// Trigger warning: bad
    fn normal_to_parametric_form(normal: &Matrix<f64>, origin: &Matrix<f64>) -> Matrix<f64> {
        // Use any vector that is not parallel to the normal.
        // We decide to use the unit vector along the x-axis,
        // otherwise the y-axis if the x-axis is parallel
        let vx = Matrix::from_array([
            [1.0, 0.0, 0.0],
        ]);
        let vy = Matrix::from_array([
            [0.0, 0.0, 1.0],
        ]);

        let v;

        if normal.cross3(&vx) != Matrix::zeros::<1, 3>() {
            v = vx;
        } else {
            v = vy;
        }

        let mut up = v.cross3(&normal);
        up.scalar(1.0 / up.length()); // normalize

        let mut right = normal.cross3(&up);
        right.scalar(1.0 / right.length()); // normalize

        Matrix::from_array([
            [*origin.index(0, 0), *origin.index(0, 1), *origin.index(0, 2)],
            [*right.index(0, 0), *right.index(0, 1), *right.index(0, 2)],
            [*up.index(0, 0), *up.index(0, 1), *up.index(0, 2)],
        ])
    }
}

pub struct Terminal<T: MatrixDataTrait> {
    config: RendererConfiguration,
    config_derived: DerivedConfiguration<T>,
    vertices: Vec<Matrix<T>>,
    line_draw_order: Vec<usize>,
    buffer: RefCell<Vec<Vec<char>>>,
    center_offset: (i64, i64),
}

impl Terminal<f64> {
    /// Get appropriate character to use for given vertical position.
    fn character_at(y: usize) -> char {
        if y % 2 == 0 {
            return character::UPPER;
        }

        character::LOWER
    }

    /// Center points such that, for example, vertex (0,0,0) appears in the middle of the terminal
    /// (which would be at (5,0,-5) after centering using a terminal with dimensions (9,9)).
    /// This is due to the origin of a terminal being at the top left.
    fn adjust_point_to_terminal(center_offset: &(i64, i64), point: &mut Matrix<f64>) {
        *point.index_mut(0, 0) += center_offset.0 as f64;
        *point.index_mut(0, 2) += center_offset.1 as f64;
    }

    /// Clear previously rendered buffer.
    fn clear(&self) {
        for v in self.buffer.borrow_mut().iter_mut() {
            for c in v.iter_mut() {
                *c = character::EMPTY;
            }
        }

        print!("\x1B[2J");
        print!("\x1B[H");
    }

    fn render_vertex(&self, buffer: &mut Vec<Vec<char>>, vertex: &Matrix<f64>) {
        let x = *vertex.x() as isize;
        let mut z = *vertex.z() as isize;

        if !(z >= 0 && z < self.config.camera.resolution.1 as isize) || 
            !(x >= 0 && x < self.config.camera.resolution.0 as isize) {
            return;
        }

        let mut character = Self::character_at(z as usize);

        z = z / 2;

        if buffer[z as usize][x as usize] == character::UPPER && character == character::LOWER {
            character = character::FULL;
        }

        if buffer[z as usize][x as usize] == character::LOWER && character == character::UPPER {
            character = character::FULL;
        }

        let _ = std::mem::replace(&mut buffer[z as usize][x as usize], character);
    }
}

impl RendererTrait<f64> for Terminal<f64> {
    type Vertex = Matrix<f64>;

    fn config(&self) -> crate::RendererConfiguration {
        self.config.clone()
    }

    fn set_config(&mut self, config: RendererConfiguration) -> Result<(), &'static str> {
        self.config = config;
        self.config_derived = DerivedConfiguration::<f64>::new(&self.config);
        Ok(()) // TODO: when does this fail again?
    }

    fn set_vertices(&mut self, vertices: &[Self::Vertex]) {
        self.vertices = vertices.to_owned();
    }

    fn set_vertices_line_draw_order(&mut self, order: &[&[usize]]) {
        self.line_draw_order = order.iter().cloned().flatten().cloned().collect();
    }

    fn render(&self) {
        for vertex in &self.vertices {
            let parameter = Matrix::from_array([
                [
                    vertex.index(0, 0) - self.config_derived.viewpoint.index(0, 0),
                    vertex.index(0, 1) - self.config_derived.viewpoint.index(0, 1),
                    vertex.index(0, 2) - self.config_derived.viewpoint.index(0, 2),
                ]
            ]);

            let viewpoint_to_vertex_line = Matrix::from_array([
                [*self.config_derived.viewpoint.index(0, 0),*self.config_derived.viewpoint.index(0, 1),*self.config_derived.viewpoint.index(0, 2)],
                [*parameter.index(0, 0), *parameter.index(0, 1), *parameter.index(0, 2)]
            ]);

            let intersection = intersect_plane_line(&self.config_derived.viewport, &viewpoint_to_vertex_line);
            let mut intersection = Matrix::from_array([
                [
                    *self.config_derived.viewpoint.index(0, 0) + *parameter.index(0, 0) * *intersection.index(0, 11),
                    *self.config_derived.viewpoint.index(0, 1) + *parameter.index(0, 1) * *intersection.index(0, 11),
                    *self.config_derived.viewpoint.index(0, 2) + *parameter.index(0, 2) * *intersection.index(0, 11),
                ]
            ]);
            
            Terminal::adjust_point_to_terminal(&self.center_offset, &mut intersection);
            self.render_vertex(&mut self.buffer.borrow_mut(), &intersection);
        }

        for character_row in self.buffer.borrow().iter() {
            for character in character_row.iter() {
                print!("{character}");
            }
            print!("\n");
        }

        self.clear();
    }
}

impl __RendererTrait<f64> for Terminal<f64> {
    fn new(config: RendererConfiguration) -> Self {
        Self {
            buffer: RefCell::new(vec![vec![character::EMPTY; config.camera.resolution().width()]; config.camera.resolution().height() / 2]),
            center_offset: (
                (config.camera.resolution().0 as f64 / 2.0) as i64 - 1,
                (config.camera.resolution().1 as f64 / 2.0) as i64 - 1,
            ),
            config_derived: DerivedConfiguration::<f64>::new(&config),
            config,
            vertices: Default::default(),
            line_draw_order: Default::default(),
        }
    }
}

/// These tests are not that thorough, just helpful testing/probing during development.
#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn normal_to_parametric_form() {
        let config = RendererConfiguration::default();
        let parametric = DerivedConfiguration::normal_to_parametric_form(&config.camera.direction, &config.camera.position);

        let a = parametric.slice(0..1, 0..3).into_iter().cloned().collect::<Vec<f64>>();
        assert!(a == vec![0.0, 0.0, -10.0], "Actual: {:?}", a);

        let a = parametric.slice(1..2, 0..3).into_iter().cloned().collect::<Vec<f64>>();
        assert!(a == vec![1.0, 0.0, 0.0], "Actual: {:?}", a);

        let a = parametric.slice(2..3, 0..3).into_iter().cloned().collect::<Vec<f64>>();
        assert!(a == vec![0.0, 0.0, 1.0], "Actual: {:?}", a);
    }
    
    #[test]
    fn a() {
        let center_offset = (5, 5);
        let mut point = Matrix::from_array([
            [0.0, 0.0, 0.0],
        ]);
        Terminal::adjust_point_to_terminal(&center_offset, &mut point);
        println!("{:?}", point);
    }

    #[test]
    fn main() {
        // 1. Create vertices
        let mut vertices = [
            Matrix::from_array([
                [0.0, 0.0, 0.0],
            ]),
            // Matrix::from_array([
            //     [1.0, 0.0, 0.0],
            // ]),
            // Matrix::from_array([
            //     [3.0, 0.0, 1.0],
            // ]),
            // Matrix::from_array([
            //     [0.0, 0.0, 1.0]
            // ])
        ];

        // 2. Define line order
        let line_draw_order = vec![vec![0,1], vec![0,2]];

        // 3. Render()
        let mut renderer = TerminalBuilder::default().build();
        // renderer.set_vertices_line_draw_order(&line_draw_order.iter().map(|v| v.as_slice()).collect::<Vec<&[usize]>>());

        loop {
            thread::sleep(Duration::from_millis(500));
            *vertices[0].index_mut(0, 2) += 1.0;
            renderer.set_vertices(&vertices);
            renderer.render();
        }
    }

    #[test]
    fn center_points() {
        let renderer = TerminalBuilder::default().build();

        // let mut test_surface = Matrix::from_array([
        //     [0.0, 0.0, 0.0],
        //     [1.0, 1.0, 1.0],
        //     [2.0, 2.0, 2.0],
        //     [1.0, 2.0, 3.0],
        // ]);

        // let expected = Matrix::from_array([
        //     [5.0, -5.0, 0.0],
        //     [6.0, -4.0, 1.0],
        //     [7.0, -3.0, 2.0],
        //     [6.0, -3.0, 3.0],
        // ]);

        // renderer.center_viewport_points(&mut test_surface);
        // assert!(test_surface == expected, "Result: {test_surface:?}\nExpected: {expected:?}");

        // let renderer = TerminalBuilder::default().with_dimensions((10,10)).build();

        // let mut test_surface = Matrix::from_array([
        //     [0.0, 0.0, 0.0],
        //     [1.0, 1.0, 1.0],
        //     [2.0, 2.0, 2.0],
        //     [1.0, 2.0, 3.0],
        // ]);

        // let expected = Matrix::from_array([
        //     [5.0, -5.0, 0.0],
        //     [6.0, -4.0, 1.0],
        //     [7.0, -3.0, 2.0],
        //     [6.0, -3.0, 3.0],
        // ]);

        // renderer.center_viewport_points(&mut test_surface);
        // assert!(test_surface == expected, "Result: {test_surface:?}\nExpected: {expected:?}");
    }
}
