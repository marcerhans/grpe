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
    pub static EMPTY: char = '+'; //
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

pub struct Terminal<T: MatrixDataTrait> {
    config: RendererConfiguration,
    vertices: Vec<Matrix<T>>,
    line_draw_order: Vec<usize>,
    viewpoint: Matrix<T>,
    viewport: Matrix<T>,
    buffer: Vec<Vec<char>>,
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
    /// (which would be at (5,-5,0) after centering using a terminal with dimensions (9,9)).
    /// This is due to the origin of a terminal being at the top left.
    fn center_viewport_points(center_offset: &(i64, i64), viewport: &mut Matrix<f64>) {
        for row in 0..viewport.rows() {
            *viewport.index_mut(row, 0) += center_offset.0 as f64;
            *viewport.index_mut(row, 1) += center_offset.1 as f64;
        }
    }

    /// Maps viewport data to buffer by using the parameters of the [PlaneTrait].
    fn map_viewport_to_buffer(&mut self) {
        for vertex in self.vertices.iter() {
            let mut eq_system = self.viewport.clone();

            eq_system.push_row(vertex.clone());

            for column in 0..vertex.columns() {
                *eq_system.index_mut(3, column) -= *eq_system.index(0, column);
            }

            eq_system.pop_row(0);
            eq_system = eq_system.transpose();
            eq_system.pop_row(2);
            let _ = gauss_elimination(&mut eq_system);

            println!("{eq_system:?}");
            let parameters = eq_system;
            let parameter_x = *parameters.index(0,2);
            let parameter_y = *parameters.index(1,2);

            let x = self.viewport.index(1, 0) * parameter_x;
            let y = self.viewport.index(2,1) * parameter_y;

            self.buffer[y as usize][x as usize] = 'x';
        }
    }

    /// Clear previously rendered frame.
    fn clear(&self) {}

    // /// Render to stdout.
    // fn render(&self, points: &mut [(Vertex<f64>, Vertex<f64>, Vertex<f64>)]) {
    //     let mut buffer = vec![vec![character::EMPTY; self.config.dimensions.1]; self.config.dimensions.0];

    //     for (ref a, ref b, ref c) in points.iter() {
    //         self.render_vertex(&mut buffer, a);
    //         self.render_vertex(&mut buffer, b);
    //         self.render_vertex(&mut buffer, c);
    //         println!("{a:?},{b:?},{c:?}");
    //     }

    //     println!("{:?}", buffer);

    //     for characters in buffer.iter() {
    //         for character in characters.iter() {
    //             print!("{character}");
    //         }
    //         println!();
    //     }
    // }

    // fn render_vertex(&self, buffer: &mut Vec<Vec<char>>, vertex: &Vertex<f64>) {
    //     let x = *vertex.x() as isize;
    //     let y = *vertex.y() as isize;
    //     let mut character = Self::character_at(y.abs() as usize);

    //     if y > 0 {
    //         if buffer[y as usize - 1][x as usize] == character::UPPER && character == character::LOWER {
    //             character = character::FULL;
    //         }
    //     }

    //     if y < self.config.dimensions.1 as isize - 1 {
    //         if buffer[y as usize + 1][x as usize] == character::LOWER && character == character::UPPER {
    //             character = character::FULL;
    //         }
    //     }

    //     let _ = std::mem::replace(&mut buffer[y as usize][x as usize], character);
    // }
}

impl RendererTrait<f64> for Terminal<f64> {
    type Vertex = Matrix<f64>;

    fn config(&self) -> crate::RendererConfiguration {
        self.config.clone()
    }

    fn set_config(&mut self, config: RendererConfiguration) -> Result<(), &'static str> {
        // let dim = config.dimensions;
        // self.config = config;
        // self.buffer = vec![vec![character::EMPTY; dim.1]; dim.0];
        // self.center_offset = ((dim.0 as f64 / 2.0).ceil() as isize, -((dim.1 as f64 / 2.0).ceil() as isize));
        // Ok(())
        todo!()
    }

    fn set_vertices(&mut self, vertices: &[Self::Vertex]) {
        self.vertices.extend_from_slice(vertices)
    }

    fn set_vertices_line_draw_order(&mut self, order: &[&[usize]]) {
        todo!()
    }

    fn render(&self) {
        todo!()
    }
}

impl __RendererTrait<f64> for Terminal<f64> {
    // TODO: Fix the viewport both parameters and position.
    fn new(config: RendererConfiguration) -> Self {
        let resolution = config.camera.resolution().clone();
        let position = config.camera.position().clone();
        let direction = config.camera.direction().clone();
        let fov = config.camera.fov().clone();

        // TODO: Determine viewport (camera) and viewpoint position to achieve desired fov
        // TODO: Just use something for now...
        let viewpoint = { Matrix::<f64>::from_array([[0.0, 0.0, -10.0]]) };

        let viewport =
            { Matrix::<f64>::from_array([[0.0, 0.0, -5.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]) };

        Self {
            config,
            vertices: Default::default(),
            line_draw_order: Default::default(),
            viewpoint,
            viewport,
            buffer: vec![vec![character::EMPTY; resolution.width()]; resolution.height()],
            center_offset: (
                (resolution.0 as f64 / 2.0).ceil() as i64,
                -((resolution.1 as f64 / 2.0).ceil() as i64),
            ),
        }
    }
}

/// These tests are not that thorough, just helpful testing/probing during development.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        // 1. Create vertices
        // 2. Define line order
        // 3. Render()
        //     Project vertices onto surface (viewport) from #1
        //     Draw lines between the projected points on surface (viewport) defined by #2
        //     Adjust points coordinates (now present in viewport) for Terminal.
        //     Map viewport (matrix) to simple 2d vec buffer.
        //     Print to stdout (terminal)
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

    #[test]
    fn map_viewport_to_buffer() {
        let mut renderer = TerminalBuilder::default().build();
        renderer.set_vertices(&[Matrix::from_array([[2.0, 3.0, 0.0]])]);
        renderer.map_viewport_to_buffer();

        // let expected = vec![vec![

        // ]];

        println!("{:?}", renderer.buffer);

        // assert!(renderer.buffer == expected);
    }
}
