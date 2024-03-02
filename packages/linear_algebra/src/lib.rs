/// TODO:
/// - To_column_vector()
/// - To_row_vector()
/// - gauss_elim()
///
/// - Submatrix() (clone)
/// - (determinant())
/// - Crossproduct()
///
/// - Add Vector(?)

pub mod matrix {
    use std::{fmt::Display, ops::{Add, Index, IndexMut, Mul, Neg, Range, Sub}};

    /// Dynamically sized n-dim matrices;
    pub struct Matrix {
        inner: Vec<f64>,
        rows: usize,
        columns: usize,
    }

    impl Matrix {
        pub fn from_slices(slices: &[&[f64]]) -> Self {
            let column_len = slices[0].len();

            for row in slices {
                if column_len != row.len() {
                    panic!("Matrix row/column lenghts are not homogenous.")
                }
            }

            Self {
                inner: slices.concat(),
                rows: slices.len(),
                columns: slices[0].len(),
            }
        }

        pub fn from_row_matrices(matrices: &[&Matrix]) -> Self {
            let rows = matrices.len();
            let columns = matrices[0].columns();

            let mut matrix = Matrix {
                inner: vec![0.0; rows * columns],
                rows,
                columns,
            };

            for row in 0..matrices.len() {
                if columns != matrices[row].columns {
                    panic!("(Row) Matrices are not uniform.")
                }

                for column in 0..columns {
                    matrix[(row,column)] = matrices[row][column];
                }
            }

            matrix
        }

        pub fn from_column_matrices(matrices: &[&Matrix]) -> Self {
            let rows = matrices[0].rows();
            let columns = matrices.len();

            let mut matrix = Matrix {
                inner: vec![0.0; rows * columns],
                rows,
                columns,
            };

            for column in 0..matrices.len() {
                if rows != matrices[column].rows {
                    panic!("(Column) Matrices are not uniform.")
                }

                for row in 0..rows {
                    matrix[(row,column)] = matrices[column][row];
                }
            }

            matrix
        }

        pub fn zeros(rows: usize, columns: usize) -> Self {
            Self {
                inner: vec![0.0; rows * columns],
                rows,
                columns,
            }
        }

        pub fn identity(rows_cols: usize) -> Self {
            let mut identity = Self {
                inner: vec![0.0; rows_cols.pow(2)],
                rows: rows_cols,
                columns: rows_cols,
            };

            for element in identity.inner.iter_mut().step_by(identity.columns + 1) {
                *element = 1.0;
            }

            identity
        }

        pub fn transpose(&self) -> Self {
            let mut matrix = Matrix {
                inner: vec![0.0; self.rows * self.columns],
                rows: self.columns,
                columns: self.rows,
            };

            for row in 0..self.rows {
                for column in 0..self.columns {
                    matrix[(column,row)] = self[(row,column)];
                }
            }

            matrix
        }

        pub fn scalar(&mut self, scalar: f64) {
            for val in self.inner.iter_mut() {
                *val *= scalar;
            }
        }

        pub fn slice(&self, range_row: Range<usize>, range_column: Range<usize>) -> Matrix {
            let range_row_len = range_row.len();
            let range_column_len = range_column.len();

            if range_row_len > self.rows || range_column_len > self.columns {
                panic!("Argument ranges are outside of base matrix scope.");
            }

            let mut matrix = Matrix {
                inner: vec![0.0; range_row_len * range_column_len],
                rows: range_row_len,
                columns: range_column_len,
            };
    
            for row in range_row.clone() {
                for column in range_column.clone() {
                    matrix[(row-range_row.start,column-range_column.start)] = self[(row,column)];
                }
            }

            matrix
        }

        pub fn inner(&self) -> &[f64] {
            return &self.inner;
        }

        pub fn row(&self, row: usize) -> Self {
            self.slice(row..row+1, 0..self.columns)
        }

        pub fn rows(&self) -> usize {
            self.rows
        }

        pub fn column(&self, column: usize) -> Self {
            self.slice(0..self.rows, column..column+1)
        }

        pub fn columns(&self) -> usize {
            self.columns
        }

        pub fn swap_rows(&mut self, this: usize, that: usize) {
            let mut buf;

            for column in 0..self.columns {
                buf = self[(this, column)];
                self[(this,column)] = self[(that, column)];
                self[(that,column)] = buf;
            }
        }

        pub fn swap_columns(&mut self, this: usize, that: usize) {
            let mut buf;

            for row in 0..self.rows {
                buf = self[(row, this)];
                self[(row, this)] = self[(row, that)];
                self[(row, that)] = buf;
            }
        }

        pub fn set_row(&mut self, row: usize, matrix: Matrix) {
            for column in 0..self.columns {
                self[(row, column)] = matrix[(0,column)];
            }
        }

        pub fn set_column(&mut self, column: usize, matrix: Matrix) {
            for row in 0..self.rows {
                self[(row, column)] = matrix[(row,0)];
            }
        }

        pub fn push_row(&mut self, mut matrix: Matrix) {
            if matrix.columns != self.columns && self.rows != 0 {
                panic!("Matrix dimensions are not compatible.");
            }

            if self.rows == 0 {
                self.columns = matrix.columns;
            }

            self.inner.append(&mut matrix.inner);
            self.rows += 1;
        }
    }

    impl std::fmt::Debug for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut fmt_vec = Vec::with_capacity(self.rows);

            for row in 0..self.rows {
                let mut columns = Vec::with_capacity(self.columns);

                for column in 0..self.columns {
                    columns.push(self[(row, column)]);
                }

                fmt_vec.push(columns);
            }

            f.debug_struct("Matrix")
                .field("rows", &self.rows)
                .field("columns", &self.columns)
                .field("inner", &fmt_vec)
                .finish()
        }
    }

    impl Index<usize> for Matrix {
        type Output = f64;

        fn index(&self, index: usize) -> &Self::Output {
            &self.inner[index]
        }
    }

    impl Index<(usize, usize)> for Matrix {
        type Output = f64;

        fn index(&self, index: (usize, usize)) -> &Self::Output {
            &self.inner[index.0 * self.columns + index.1]
        }
    }

    impl IndexMut<usize> for Matrix {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.inner[index]
        }
    }

    impl IndexMut<(usize, usize)> for Matrix {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            &mut self.inner[index.0 * self.columns + index.1]
        }
    }

    impl Eq for Matrix {}
    impl PartialEq for Matrix {
        fn eq(&self, other: &Self) -> bool {
            if self.rows != other.rows || self.columns != other.columns {
                return false;
            }

            for (lhs, rhs) in self.inner.iter().zip(other.inner.iter()) {
                if (lhs - rhs).abs() > f64::EPSILON {
                    return false;
                }
            }

            true
        }
    }

    impl Neg for &Matrix {
        type Output = Matrix;

        fn neg(self) -> Self::Output {
            let mut neg = Matrix::zeros(self.rows, self.columns);

            for index in 0..self.inner.len() {
                neg[index] = -self[index];
            }

            neg
        }
    }

    /// TODO: Add, Sub, and Mul have similarities. Maybe it can be refactored?
    impl Add for &Matrix {
        type Output = Matrix;

        fn add(self, rhs: Self) -> Self::Output {
            if self.rows != rhs.rows || self.columns != rhs.columns {
                panic!("Addition cannot be performed on matrices of different dimensions.")
            }

            let mut sum = Matrix::zeros(self.rows, self.columns);

            for index in 0..self.inner.len() {
                sum[index] = self[index] + rhs[index];
            }

            sum
        }
    }

    impl Sub for &Matrix {
        type Output = Matrix;

        fn sub(self, rhs: Self) -> Self::Output {
            if self.rows != rhs.rows || self.columns != rhs.columns {
                panic!("Subtraction cannot be performed on matrices of different dimensions.")
            }

            let mut sum = Matrix::zeros(self.rows, self.columns);

            for index in 0..self.inner.len() {
                sum[index] = self[index] - rhs[index];
            }

            sum
        }
    }

    impl Mul for &Matrix {
        type Output = Matrix;

        fn mul(self, rhs: Self) -> Self::Output {
            if self.columns != rhs.rows {
                panic!("Matrix multiplication cannot be performed on matrices with incompatible dimensions.")
            }

            let mut product = Matrix::zeros(self.rows, rhs.columns);

            for product_index_row in 0..self.rows {
                for product_index_column in 0..rhs.columns {
                    for index_column_row in 0..self.columns {
                        product[(product_index_row, product_index_column)] += self
                            [(product_index_row, index_column_row)]
                            * rhs[(index_column_row, product_index_column)];
                    }
                }
            }

            product
        }
    }

    pub mod macros {
        #[macro_export]
        macro_rules! matrix {
            [ $( [ $( $row:expr ),* ] $(,)* )* ] => {
                crate::matrix::Matrix::from_slices(
                    &[
                        $(
                            &[$($row),*]
                        ),*
                    ]
                )
            };

            ( $rows:expr, $columns:expr ) => {
                crate::matrix::Matrix::zeros(($rows), ($columns))
            };

            ( $rows_cols:expr ) => {
                crate::matrix::Matrix::zeros(($rows_cols), ($rows_cols))
            };
        }
        pub use matrix;

        #[macro_export]
        macro_rules! identity {
            ( $rows_cols:expr ) => {
                crate::matrix::Matrix::identity(($rows_cols))
            };
        }
        pub use identity;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        static EPSILON: f64 = std::f64::EPSILON;

        mod test_from_slices {
            use super::*;

            #[test]
            fn from_slices() {
                let matrix = Matrix::from_slices(&[
                    &[1.0, 2.0, 3.0, 4.0],
                    &[5.0, 6.0, 7.0, 8.0],
                    &[9.0, 10.0, 11.0, 12.0],
                    &[13.0, 14.0, 15.0, 16.0],
                    &[17.0, 18.0, 19.0, 20.0],
                ]);

                check(matrix);
            }

            #[test]
            fn from_slices_with_macro() {
                let matrix = macros::matrix![
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                    [17.0, 18.0, 19.0, 20.0],
                ];

                check(matrix);
            }

            fn check(matrix: Matrix) {
                let mut expected = 1.0;

                for val in matrix.inner() {
                    assert!((val - expected).abs() < EPSILON, "Matrix not valid");
                    expected += 1.0;
                }
            }
        }

        mod test_from_row_matrices {
            use super::*;

            #[test]
            fn from_row_matrices() {
                let a = macros::matrix![
                    [1.0, 2.0, 3.0],
                ];
                let b = macros::matrix![
                    [4.0, 5.0, 6.0],
                ];
                let c = macros::matrix![
                    [7.0, 8.0, 9.0],
                ];

                let vector = [&a,&b,&c];

                assert!(Matrix::from_row_matrices(&vector) == macros::matrix![
                    [1.0, 2.0, 3.0],
                    [4.0, 5.0, 6.0],
                    [7.0, 8.0, 9.0],
                ])
            }
        }

        mod test_from_column_matrices {
            use super::*;

            #[test]
            fn from_column_matrices() {
                let a = macros::matrix![
                    [1.0],
                    [2.0],
                    [3.0],
                ];
                let b = macros::matrix![
                    [4.0],
                    [5.0],
                    [6.0],
                ];
                let c = macros::matrix![
                    [7.0],
                    [8.0],
                    [9.0],
                ];

                let vector = [&a,&b,&c];

                assert!(Matrix::from_column_matrices(&vector) == macros::matrix![
                    [1.0, 4.0, 7.0],
                    [2.0, 5.0, 8.0],
                    [3.0, 6.0, 9.0],
                ])
            }
        }

        mod test_zeros {
            use super::*;

            #[test]
            fn zeros() {
                let matrix = Matrix::zeros(3, 4);
                check(matrix, 3, 4);
            }

            #[test]
            fn zeros_with_macro() {
                let matrix = macros::matrix!(3, 4);
                check(matrix, 3, 4);

                let matrix = macros::matrix!(4);
                check(matrix, 4, 4);
            }

            fn check(matrix: Matrix, rows: usize, columns: usize) {
                let expected = 0.0;

                for val in matrix.inner() {
                    assert!((val - expected).abs() < EPSILON, "Matrix not valid");
                }

                assert!(matrix.rows == rows);
                assert!(matrix.columns == columns);
                assert!(matrix.inner().len() == matrix.rows * matrix.columns);
            }
        }

        mod test_identity {
            use super::*;

            #[test]
            fn identity() {
                let matrix = Matrix::identity(4);
                check(matrix);
            }

            #[test]
            fn identity_with_macro() {
                let matrix = macros::identity!(4);
                check(matrix);
            }

            fn check(matrix: Matrix) {
                let mut offset = 0;

                for (index, element) in matrix.inner().iter().enumerate() {
                    if index == matrix.columns * offset + offset {
                        assert!((*element - 1.0).abs() < EPSILON);
                        offset += 1;
                    } else {
                        assert!(*element == 0.0);
                    }
                }
            }
        }

        mod test_transpose {
            use super::*;

            #[test]
            fn transpose() {
                let matrix = macros::matrix![
                    [1.0, 2.0],
                    [3.0, 4.0],
                    [5.0, 6.0],
                ];

                assert!(matrix.transpose() == macros::matrix![
                    [1.0, 3.0, 5.0],
                    [2.0, 4.0, 6.0],
                ]);
            }
        }

        mod test_swap {
            use super::*;

            #[test]
            fn swap_rows() {
                let mut matrix = macros::matrix![
                    [1.0,2.0],
                    [3.0,4.0],
                ];

                matrix.swap_rows(0, 1);

                assert!(matrix[(0,0)] - 3.0 < f64::EPSILON);
                assert!(matrix[(0,1)] - 4.0 < f64::EPSILON);
                assert!(matrix[(1,0)] - 1.0 < f64::EPSILON);
                assert!(matrix[(1,1)] - 2.0 < f64::EPSILON);
            }

            #[test]
            fn swap_columns() {
                let mut matrix = macros::matrix![
                    [1.0,2.0],
                    [3.0,4.0],
                ];

                matrix.swap_columns(0, 1);

                assert!(matrix[(0,0)] - 2.0 < f64::EPSILON);
                assert!(matrix[(0,1)] - 1.0 < f64::EPSILON);
                assert!(matrix[(1,0)] - 4.0 < f64::EPSILON);
                assert!(matrix[(1,1)] - 3.0 < f64::EPSILON);
            }
        }

        mod test_slice {
            use super::*;

            #[test]
            fn slice() {
                let matrix = macros::matrix![
                    [1.0, 2.0,  3.0,  4.0],
                    [5.0, 6.0,  7.0,  8.0],
                    [9.0,10.0, 11.0, 12.0],
                ];

                assert!(matrix.slice(0..1, 0..4) == macros::matrix![[1.0, 2.0,  3.0,  4.0]]);
                assert!(matrix.slice(0..3, 0..1) == macros::matrix![[1.0], [5.0], [9.0]]);
                assert!(matrix.slice(1..3, 1..4) == macros::matrix![[6.0, 7.0, 8.0], [10.0, 11.0, 12.0]]);
            }
        }

        mod test_trait_index_and_index_mut {
            use super::*;

            #[test]
            fn index_usize() {
                let mut matrix = macros::matrix!(4);

                matrix[4 * 0] = 1.0;
                matrix[4 * 1] = 1.0;
                matrix[4 * 2] = 1.0;
                matrix[4 * 3] = 1.0;

                check_usize(matrix);
            }

            #[test]
            fn index_usize_usize() {
                let mut matrix = macros::matrix!(4);

                matrix[(0, 0)] = 1.0;
                matrix[(1, 1)] = 1.0;
                matrix[(2, 2)] = 1.0;
                matrix[(3, 3)] = 1.0;

                check_usize_usize(matrix);
            }

            #[test]
            #[should_panic]
            fn index_out_of_bounds_usize() {
                // Ignore stacktrace output
                std::panic::set_hook(Box::new(|_| {}));

                let mut matrix = macros::matrix!(4);
                matrix[4 * 4] = 1.0;
            }

            #[test]
            #[should_panic]
            fn index_out_of_bounds_usize_usize() {
                // Ignore stacktrace output
                std::panic::set_hook(Box::new(|_| {}));

                let mut matrix = macros::matrix!(4);
                matrix[(4, 4)] = 1.0;
            }

            fn check_usize(matrix: Matrix) {
                assert!((matrix[4 * 0] - 1.0).abs() < EPSILON);
                assert!((matrix[4 * 1] - 1.0).abs() < EPSILON);
                assert!((matrix[4 * 2] - 1.0).abs() < EPSILON);
                assert!((matrix[4 * 3] - 1.0).abs() < EPSILON);
            }

            fn check_usize_usize(matrix: Matrix) {
                assert!((matrix[(0, 0)] - 1.0).abs() < EPSILON);
                assert!((matrix[(1, 1)] - 1.0).abs() < EPSILON);
                assert!((matrix[(2, 2)] - 1.0).abs() < EPSILON);
                assert!((matrix[(3, 3)] - 1.0).abs() < EPSILON);
            }
        }

        mod test_trait_add {
            use super::*;

            #[test]
            fn add() {
                let a = macros::identity!(4);
                let b = macros::identity!(4);
                let c = &a + &b;
                check(c);
            }

            #[test]
            #[should_panic]
            fn add_panic() {
                // Ignore stacktrace output
                std::panic::set_hook(Box::new(|_| {}));
                let a = macros::identity!(4);
                let b = macros::identity!(5);
                let _c = &a + &b;
            }

            fn check(matrix: Matrix) {
                let mut offset = 0;

                for (index, element) in matrix.inner().iter().enumerate() {
                    if index == matrix.columns * offset + offset {
                        assert!((*element - 2.0).abs() < EPSILON);
                        offset += 1;
                    } else {
                        assert!(*element == 0.0);
                    }
                }
            }
        }

        mod test_trait_sub {
            use super::*;

            #[test]
            fn sub() {
                let a = macros::identity!(4);
                let b = macros::identity!(4);
                let c = &a - &b;
                check(c);
            }

            #[test]
            #[should_panic]
            fn sub_panic() {
                // Ignore stacktrace output
                std::panic::set_hook(Box::new(|_| {}));
                let a = macros::identity!(4);
                let b = macros::identity!(5);
                let _c = &a - &b;
            }

            fn check(matrix: Matrix) {
                for element in matrix.inner().iter() {
                    assert!(*element == 0.0);
                }
            }
        }

        mod test_trait_mul {
            use super::*;

            #[test]
            fn mul() {
                #[rustfmt::skip]
                let a = macros::matrix![
                    [1.0, 2.0],
                    [3.0, 4.0],
                    [5.0, 6.0],
                    [7.0, 8.0]
                ];

                #[rustfmt::skip]
                let b = macros::matrix![
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0]
                ];

                let c = &a * &b;
                check(c);
            }

            #[test]
            #[should_panic]
            fn mul_panic() {
                // Ignore stacktrace output
                std::panic::set_hook(Box::new(|_| {}));
                let a = macros::matrix![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [7.0, 8.0],];

                let b = macros::matrix![
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [0.0, 0.0, 0.0, 0.0],
                ];

                let _c = &a * &b;
            }

            fn check(matrix: Matrix) {
                #[rustfmt::skip]
                let matrix_compare = macros::matrix![
                    [1.0+10.0,      2.0+12.0,       3.0+14.0,       4.0+16.0],
                    [3.0+20.0,      6.0+24.0,       9.0+28.0,       12.0+32.0],
                    [5.0+30.0,      10.0+36.0,      15.0+42.0,      20.0+48.0],
                    [7.0+40.0,      14.0+48.0,      21.0+56.0,      28.0+64.0],
                ];

                assert!(
                    matrix_compare == matrix,
                    "Matrix:\n{:?}\n\nMatrix_Compare:\n{:?}",
                    matrix,
                    matrix_compare
                );
            }
        }
    }
}

/// TODO:
/// Implement a vector type.
/// - Refactor and extract aspects from the matrix type as traits,
/// and share between both vector and matrix.
/// 
/// There is currently no NEED for vector (though it would be nice/better).
mod vector {
    use super::*;

    /// [Vector] which internally uses a nx1 [matrix::Matrix].
    pub struct Vector {
        inner: matrix::Matrix,
        len: usize,
    }

    impl Vector {
        fn from_slice(slice: &[f64]) -> Self {
            Self {
                inner: matrix::Matrix::from_slices(&[slice]),
                len: slice.len(),
            }
        }

        fn inner(&self) -> &matrix::Matrix {
            &self.inner
        }
    }

    pub mod macros {
        #[macro_export]
        macro_rules! vector {
            [ $( $row:expr ),* ] => {
                crate::vector::Vector::from_slice(
                    &[$($row),*]
                )
            };

            ( $len:expr ) => {
                crate::vector::Vector::from_slice(($len))
            };
        }
        pub use vector;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn main() {
            let vector = macros::vector![1.0,2.0,3.0];
        }
    }
}

pub mod utility {
    use super::*;

    /// Last column in matrix is seen as the sum of the values to the left,
    /// where the values to the left are parametric.
    /// Only solves n by n+1 matrices.
    pub fn gauss_elimination(mut matrix: &mut matrix::Matrix) -> Option<matrix::Matrix> {
        enum Direction {
            Down,
            Up,
        }

        /// Find a 'pivotable' point in a row given a column.
        fn find_next_pivot_row(matrix: &mut matrix::Matrix, start_row: usize, column: usize) -> Option<usize> {
            for row in start_row..matrix.rows() {
                if matrix[(row,column)] > f64::EPSILON {
                    return Some(row);
                }
            }

            None
        }

        /// Normalize the row for a given pivot coordinate.
        fn normalize_row(matrix: &mut matrix::Matrix, row: usize, divisor_column: usize) {
            let divisor = matrix[(row,divisor_column)];

            for column in 0..matrix.columns() {
                matrix[(row,column)] /= divisor;
            }
        }

        /// Eliminate all rows based on pivot.
        fn eliminate_columns(matrix: &mut matrix::Matrix, pivot_row: usize, pivot_column: usize, start_row: usize, direction: Direction) {
            match direction {
                Direction::Down => {
                    for row in start_row..matrix.rows() {
                        if matrix[(row, pivot_column)] < f64::EPSILON {
                            continue;
                        }

                        let pivot_factor = matrix[(row, pivot_column)];

                        for column in 0..matrix.columns() {
                            matrix[(row, column)] -= matrix[(pivot_row, column)] * pivot_factor;
                        }
                    }
                },
                Direction::Up => {
                    for row in (0..start_row).rev() {
                        if matrix[(row, pivot_column)] < f64::EPSILON {
                            continue;
                        }

                        let pivot_factor = matrix[(row, pivot_column)];

                        for column in 0..matrix.columns() {
                            matrix[(row, column)] -= matrix[(pivot_row, column)] * pivot_factor;
                        }
                    }
                },
            }
        }

        if matrix.rows() + 1 != matrix.columns() {
            return None;
        }

        // "Downward"
        for row_and_column in 0..matrix.rows()-1 {
            if let Some(pivot_row) = find_next_pivot_row(matrix, row_and_column, row_and_column) {
                matrix.swap_rows(row_and_column,pivot_row);
                normalize_row(&mut matrix, row_and_column, row_and_column);
                eliminate_columns(&mut matrix, row_and_column, row_and_column, row_and_column + 1, Direction::Down);
            } else {
                return None;
            }
        }

        // "Upward"
        for row_and_column in (1..matrix.rows()).rev() {
            normalize_row(&mut matrix, row_and_column, row_and_column);
            eliminate_columns(&mut matrix, row_and_column, row_and_column, row_and_column, Direction::Up);
        }

        None
    }

    pub fn rotate_x(matrix: &matrix::Matrix, radians: f64) -> matrix::Matrix {
        let rotation_matrix = matrix::macros::matrix![
            [1.0,   0.0,             0.0],
            [0.0,   radians.cos(),   -radians.sin()],
            [0.0,   radians.sin(),   radians.cos()],
        ];

        matrix * &rotation_matrix
    }

    pub fn rotate_y(matrix: &matrix::Matrix, radians: f64) -> matrix::Matrix {
        let rotation_matrix = matrix::macros::matrix![
            [radians.cos(),     0.0,   radians.sin()],
            [0.0,               1.0,   0.0],
            [-radians.sin(),    0.0,   radians.cos()],
        ];

        matrix * &rotation_matrix
    }

    pub fn rotate_z(matrix: &matrix::Matrix, radians: f64) -> matrix::Matrix {
        let rotation_matrix = matrix::macros::matrix![
            [radians.cos(),     -radians.sin(),     0.0],
            [radians.sin(),     radians.cos(),      0.0],
            [0.0,               0.0,                1.0],
        ];

        matrix * &rotation_matrix
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod test_gauss_elimination {
            use super::*;

            #[test]
            fn test_1() {
                let mut matrix = matrix::macros::matrix![
                    [1.0,0.0,0.0,1.0],
                    [0.0,1.0,0.0,2.0],
                    [0.0,0.0,1.0,3.0],
                ];

                gauss_elimination(&mut matrix);

                let expected = matrix::macros::matrix![
                    [1.0,0.0,0.0,1.0],
                    [0.0,1.0,0.0,2.0],
                    [0.0,0.0,1.0,3.0],
                ];

                check(matrix, expected);
            }

            #[test]
            fn test_2() {
                let mut matrix = matrix::macros::matrix![
                    [0.0,1.0,0.0,2.0],
                    [1.0,0.0,0.0,1.0],
                    [0.0,0.0,1.0,3.0],
                ];

                gauss_elimination(&mut matrix);

                let expected = matrix::macros::matrix![
                    [1.0,0.0,0.0,1.0],
                    [0.0,1.0,0.0,2.0],
                    [0.0,0.0,1.0,3.0],
                ];

                check(matrix, expected);
            }

            #[test]
            fn test_3() {
                let mut matrix = matrix::macros::matrix![
                    [0.0,0.0,1.0,3.0],
                    [0.0,1.0,0.0,2.0],
                    [1.0,0.0,0.0,1.0],
                ];

                gauss_elimination(&mut matrix);

                let expected = matrix::macros::matrix![
                    [1.0,0.0,0.0,1.0],
                    [0.0,1.0,0.0,2.0],
                    [0.0,0.0,1.0,3.0],
                ];

                check(matrix, expected);
            }

            #[test]
            fn test_4() {
                let mut matrix = matrix::macros::matrix![
                    [5.0, 3.0,7.0,1.0],
                    [2.0, 4.0,9.0,3.0],
                    [11.0,7.0,1.0,4.0],
                ];

                gauss_elimination(&mut matrix);

                let expected = matrix::macros::matrix![
                    [1.0,0.0,0.0,   -75.0/214.0],
                    [0.0,1.0,0.0,   243.0/214.0],
                    [0.0,0.0,1.0,   -10.0/107.0],
                ];

                check(matrix, expected);
            }

            fn check(matrix: matrix::Matrix, expected: matrix::Matrix) {
                assert!(matrix == expected);
            }
        }
    }
}