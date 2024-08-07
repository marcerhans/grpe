use crate::matrix::{Matrix, MatrixDataTrait};

/// Find the unique solution given an equation system using Gauss-elimination.
/// Fails to compute (returns [None]) if there are infinitely many solutions or none.
pub fn solve_eq_system<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
    equation_system: &mut Matrix<T, ROWS, COLS>,
) -> Option<()> {
    use gauss_elimination::*;

    for pivot_row in  0..(ROWS-1) {
        let found_pivot = find_pivot(&equation_system, pivot_row, pivot_row, Direction::DOWN);

        if found_pivot.is_none() {
            return None;
        }

        let found_pivot = found_pivot.unwrap();

        if pivot_row != found_pivot {
            equation_system.swap_row(pivot_row, found_pivot);
        }

        for current_row in (pivot_row+1)..ROWS {
            subtract_based_on_pivot_row(equation_system, pivot_row, pivot_row, current_row);
        }
    }

    for pivot_row in  (1..ROWS).rev() {
        let found_pivot = find_pivot(&equation_system, pivot_row, pivot_row, Direction::UP);

        if found_pivot.is_none() {
            return None;
        }

        let found_pivot = found_pivot.unwrap();

        if pivot_row != found_pivot {
            equation_system.swap_row(pivot_row, found_pivot);
        }

        for current_row in (0..pivot_row).rev() {
            subtract_based_on_pivot_row(equation_system, pivot_row, pivot_row, current_row);
        }
    }

    // Normalize values and check if solution exists.
    for pivot in 0..ROWS {
        if equation_system[pivot][pivot] == T::zero() {
            return None;
        }

        equation_system[pivot][COLS-1] = equation_system[pivot][COLS-1] / equation_system[pivot][pivot];
        equation_system[pivot][pivot] = T::one();
    }

    Some(())
}

mod gauss_elimination {
    use super::*;

    pub enum Direction {
        UP,
        DOWN,
    }

    /// Find next pivot point (row) given row_start and current column.
    /// Returns the row for next suitable pivot. Otherwise returns [None].
    /// Note: Want to have it as nested function, but then I cannot test it ;(.
    pub fn find_pivot<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
        equation_system: &Matrix<T, ROWS, COLS>,
        row_start: usize,
        column: usize,
        direction: Direction,
    ) -> Option<usize> {
        match direction {
            Direction::UP => {
                for row in (0..=row_start).rev() {
                    if !equation_system[row][column].eqa(&T::zero(), &T::epsilon()) {
                        return Some(row);
                    }
                }
            }
            Direction::DOWN => {
                for row in row_start..ROWS {
                    if !equation_system[row][column].eqa(&T::zero(), &T::epsilon()) {
                        return Some(row);
                    }
                }
            },
        }

        return None;
    }

    /// Subtract elements in current row with values from pivot row, based on pivot column.
    /// Note: Want to have it as nested function, but then I cannot test it ;(.
    pub fn subtract_based_on_pivot_row<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
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
            assert!(find_pivot(&eq_system, 0, 0, Direction::DOWN).unwrap() == 0);

            let eq_system = Matrix::from([
                [0, 3, 4, 5],
                [6, 7, 8, 9],
            ]);
            assert!(find_pivot(&eq_system, 0, 0, Direction::DOWN).unwrap() == 1);

            let eq_system = Matrix::from([
                [0, 0, 4, 5],
                [6, 7, 8, 9],
                [0, 0, 8, 9],
                [0, 1, 8, 9],
            ]);
            assert!(find_pivot(&eq_system, 2, 1, Direction::DOWN).unwrap() == 3);

            let eq_system = Matrix::from([
                [0, 0, 4, 5],
                [6, 7, 8, 9],
                [0, 0, 8, 9],
                [0, 1, 0, 0],
            ]);
            assert!(find_pivot(&eq_system, 3, 2, Direction::UP).unwrap() == 2);

            let eq_system = Matrix::from([
                [0, 0, 4, 5],
                [6, 7, 0, 0],
                [0, 0, 8, 9],
                [0, 1, 8, 0],
            ]);
            assert!(find_pivot(&eq_system, 1, 2, Direction::UP).unwrap() == 0);
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

            let mut eq_system = Matrix::from([
                [2, 3, 4, 5],
                [6, 7, 8, 9],
                [10, 11, 12, 13],
                [14, 15, 16, 17],
            ]);
            subtract_based_on_pivot_row(&mut eq_system, 1, 1, 2);
            subtract_based_on_pivot_row(&mut eq_system, 1, 1, 3);
            assert!(eq_system == Matrix::from([
                [2, 3, 4, 5],
                [6, 7, 8, 9],
                [4, 0, -4, -8],
                [8, 0, -8, -16],
            ]));

            let mut eq_system = Matrix::from([
                [2, 3, 4, 5],
                [6, 7, 8, 9],
                [10, 12, 12, 13],
                [14, 4, 5, 17],
            ]);
            subtract_based_on_pivot_row(&mut eq_system, 0, 0, 1);
            subtract_based_on_pivot_row(&mut eq_system, 0, 0, 2);
            subtract_based_on_pivot_row(&mut eq_system, 0, 0, 3);
            subtract_based_on_pivot_row(&mut eq_system, 1, 1, 2);
            subtract_based_on_pivot_row(&mut eq_system, 1, 1, 3);
            subtract_based_on_pivot_row(&mut eq_system, 2, 2, 3);
            assert!(eq_system == Matrix::from([
                [2, 3, 4, 5],
                [0, -4, -8, -12],
                [0, 0, 16, 24],
                [0, 0, 0, -2112],
            ]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_eq_system_test() {
        // These tests will work well with integers and have solution.
        let mut eq_system = Matrix::from([
            [1, 1, 1, 2],
            [6, -4, 5, 31],
            [5, 2, 2, 13],
        ]);
        assert!(solve_eq_system(&mut eq_system).is_some());
        assert!(eq_system == Matrix::from([
            [1, 0, 0, 3],
            [0, 1, 0, -2],
            [0, 0, 1, 1],
        ]));

        let mut eq_system = Matrix::from([
            [1, -2, 3, 9],
            [-1, 3, -1, -6],
            [2, -5, 5, 17],
        ]);
        assert!(solve_eq_system(&mut eq_system).is_some());
        assert!(eq_system == Matrix::from([
            [1, 0, 0, 1],
            [0, 1, 0, -1],
            [0, 0, 1, 2],
        ]));

        let mut eq_system = Matrix::from([
            [2, 1, -2, -1],
            [3, -3, -1, 5],
            [1, -2, 3, 6],
        ]);
        assert!(solve_eq_system(&mut eq_system).is_some());
        assert!(eq_system == Matrix::from([
            [1, 0, 0, 1],
            [0, 1, 0, -1],
            [0, 0, 1, 1],
        ]));

        // Should have no solution(s).
        let mut eq_system = Matrix::from([
            [1, -3, 1, 4],
            [-1, 2, -5, 3],
            [5, -13, 13, 8],
        ]);
        assert!(solve_eq_system(&mut eq_system).is_none());

        // Infinitely many solutions.
        let mut eq_system = Matrix::from([
            [2, 1, -3, 0],
            [4, 2, -6, 0],
            [1, -1, 1, 0],
        ]);
        assert!(solve_eq_system(&mut eq_system).is_none());
    }
}