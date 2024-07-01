use crate::matrix::{Matrix, MatrixDataTrait};
use crate::vector::{VectorColumn, VectorRow};

/// Find the unique solution given an equation system using Gauss-elimination.
/// Fails to compute if there are infinitely many solutions or none.
pub fn solve_eq_system<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
    equation_system: Matrix<T, ROWS, COLS>,
) -> Option<VectorColumn<T, ROWS>> {
    todo!()
}

mod gauss_elimination {
    use super::*;

    /// Find next pivot point (row) given row_start and current column.
    /// Returns the row for next suitable pivot. Otherwise returns [None].
    /// Note: Want to have it as nested function, but then I cannot test it ;(.
    fn find_pivot<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
        equation_system: &Matrix<T, ROWS, COLS>,
        row_start: usize,
        column: usize,
    ) -> Option<usize> {
        for row in row_start..ROWS {
            if !equation_system[row][column].eqa(&T::zero(), &T::epsilon()) {
                return Some(row);
            }
        }

        return None;
    }

    /// Subtract elements in current row with values from pivot row, based on pivot column.
    /// Note: Want to have it as nested function, but then I cannot test it ;(.
    fn subtract_based_on_pivot_row<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
        equation_system: &mut Matrix<T, ROWS, COLS>,
        pivot_row: usize,
        pivot_column: usize,
        current_row: usize,
    ) {
        let current_row_pivot_factor = equation_system[current_row][pivot_column]; // Need to save since we mutate during iteration.

        for column in 0..COLS {
            let pivot_row_term = equation_system[pivot_row][column] * current_row_pivot_factor;
            let current_row_term = equation_system[current_row][column] * equation_system[pivot_row][pivot_column];
            equation_system[current_row][column] =  current_row_term - pivot_row_term;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn find_pivot_test() {
            let eq_system = Matrix::from([
                [2, 3, 4, 5],
                [6, 7, 8, 9],
            ]);
            assert!(find_pivot(&eq_system, 0, 0).unwrap() == 0);

            let eq_system = Matrix::from([
                [0, 3, 4, 5],
                [6, 7, 8, 9],
            ]);
            assert!(find_pivot(&eq_system, 0, 0).unwrap() == 1);

            let eq_system = Matrix::from([
                [0, 0, 4, 5],
                [6, 7, 8, 9],
                [0, 0, 8, 9],
                [0, 1, 8, 9],
            ]);
            assert!(find_pivot(&eq_system, 2, 1).unwrap() == 3);
        }

        #[test]
        fn subtract_based_on_pivot_row_test() {
            let mut eq_system = Matrix::from([
                [2, 3, 4, 5],
                [6, 7, 8, 9],
            ]);
            subtract_based_on_pivot_row(&mut eq_system, 0, 0, 1);
            assert!(eq_system == Matrix::from([
                [2, 3, 4, 5],
                [0, -4, -8, -12],
            ]));

            let mut eq_system = Matrix::from([
                [2, 3, 4, 5],
                [6, 7, 8, 9],
                [2, 3, 4, 5],
            ]);
            subtract_based_on_pivot_row(&mut eq_system, 1, 1, 2);
            assert!(eq_system == Matrix::from([
                [2, 3, 4, 5],
                [6, 7, 8, 9],
                [-4, 0, 4, 8],
            ]));
        }
    }
}

