/// Common traits and structures.
use linear_algebra::{matrix::{Matrix, MatrixDataTrait}, utility::gauss_elimination};

pub trait PointTrait<T: MatrixDataTrait> {
    fn new(data: [T; 3]) -> Self;

    fn x(&self) -> &T;
    fn y(&self) -> &T;
    fn z(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
    fn z_mut(&mut self) -> &mut T;
}

impl<T: MatrixDataTrait> PointTrait<T> for Matrix<T> {
    fn new(data: [T; 3]) -> Self {
        Matrix::from_array([data; 1])
    }

    fn x(&self) -> &T {
        self.index(0, 0)
    }

    fn y(&self) -> &T {
        self.index(0, 1)
    }

    fn z(&self) -> &T {
        self.index(0, 2)
    }

    fn x_mut(&mut self) -> &mut T {
        self.index_mut(0, 0)
    }

    fn y_mut(&mut self) -> &mut T {
        self.index_mut(0, 1)
    }

    fn z_mut(&mut self) -> &mut T {
        self.index_mut(0, 2)
    }
}

pub trait PlaneTrait<T> {
    fn point(&self) -> &[T];
    fn parameter_a(&self) -> &[T];
    fn parameter_b(&self) -> &[T];
}

impl<T: MatrixDataTrait> PlaneTrait<T> for Matrix<T> {
    fn point(&self) -> &[T] {
        &self.data()[0..3]
    }

    fn parameter_a(&self) -> &[T] {
        &self.data()[3..6]
    }

    fn parameter_b(&self) -> &[T] {
        &self.data()[6..9]
    }
}

pub trait DimensionsTrait {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl DimensionsTrait for (usize, usize) {
    fn width(&self) -> usize {
        self.0
    }

    fn height(&self) -> usize {
        self.1
    }
}

/// Solves the equation system below to find the point at which the
/// line and the plane intersects.
///
/// |plane_x_0|          |x|          |x|   |line_x_0|         |x|
/// |plane_y_0| + plane_t|y| + plane_s|y| = |line_y_0| + line_t|y|
/// |plane_z_0|          |z|          |z|   |line_z_0|         |z|
pub fn intersect_plane_line(plane: &Matrix<f64>, line: &Matrix<f64>) -> Matrix<f64> {
    let plane_origin = plane.slice(0..1, 0..3);
    let plane_t_vec = plane.slice(1..2, 0..3);
    let plane_s_vec = plane.slice(2..3, 0..3);

    let line_origin = line.slice(0..1, 0..3);
    let line_t_vec = line.slice(1..2, 0..3);

    let origin = [
        line_origin[0] - plane_origin[0],
        line_origin[1] - plane_origin[1],
        line_origin[2] - plane_origin[2],
    ];

    let mut intersection_point = Matrix::<f64>::from_array([
        [*plane_t_vec[0], *plane_s_vec[0], -*line_t_vec[0], origin[0]],
        [*plane_t_vec[1], *plane_s_vec[1], -*line_t_vec[1], origin[1]],
        [*plane_t_vec[2], *plane_s_vec[2], -*line_t_vec[2], origin[2]],
    ]);

    gauss_elimination(&mut intersection_point);
    intersection_point
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_plane_line_test() {
        let plane = Matrix::from_array([
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
        ]);

        let line = Matrix::from_array([
            [0.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
        ]);

        let intersection = intersect_plane_line(&plane, &line);

        println!("{:?}", intersection);
    }
}