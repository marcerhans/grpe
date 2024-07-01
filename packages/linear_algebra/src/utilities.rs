use crate::matrix::{Matrix, MatrixDataTrait};
use crate::vector::{VectorRow, VectorColumn};

pub fn solve_eq_system<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>() -> Result<VectorColumn<T, ROWS>, &'static str> {
    todo!()
}