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
    use std::ops::{Add, Index, IndexMut, Mul, Sub};

    /// Dynamically sized n-dim matrices;
    pub struct Matrix {
        inner: Vec<f64>,
        rows: usize,
        columns: usize,
    }

    impl Matrix {
        pub fn from_slices(arrays: &[&[f64]]) -> Self {
            let column_len = arrays[0].len();

            for row in arrays {
                if column_len != row.len() {
                    panic!("Matrix row/column lenghts are not homogenous.")
                }
            }

            Self {
                inner: arrays.concat(),
                rows: arrays.len(),
                columns: arrays[0].len(),
            }
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

        pub fn inner(&self) -> &[f64] {
            return &self.inner;
        }

        pub fn rows(&self) -> usize {
            self.rows
        }

        pub fn columns(&self) -> usize {
            self.columns
        }
    }

    impl std::fmt::Debug for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Matrix")
                .field("rows", &self.rows)
                .field("columns", &self.columns)
                .field("inner", &self.inner)
                .finish()
        }
    }

    impl Index<usize> for Matrix {
        type Output = f64;

        fn index(&self, index: usize) -> &Self::Output {
            &self.inner()[index]
        }
    }

    impl Index<(usize, usize)> for Matrix {
        type Output = f64;

        fn index(&self, index: (usize, usize)) -> &Self::Output {
            &self.inner()[index.0 * self.columns + index.1]
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

pub mod utility {
    use super::*;

    fn gauss_elimination() {}
}