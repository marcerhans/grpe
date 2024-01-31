pub mod matrix2 {
    /// Dynamically sized n-dim matrices;
    pub struct Matrix {
        inner: Vec<f64>,
        rows: usize,
        columns: usize,
    }

    impl Matrix {
        pub fn from_arrays(columns: &[&[f64]]) -> Self {
            let standard_rows_in_column = columns[0].len();

            for column in columns {
                if standard_rows_in_column != column.len() {
                    panic!("Matrix row/column lenghts are not homogenous.")
                }
            }

            Self {
                inner: columns.concat(),
                rows: columns[0].len(),
                columns: columns.len(),
            }
        }

        pub fn zeros(rows: usize, columns: usize) -> Self {
            Self {
                inner: vec![0.0; rows * columns],
                rows,
                columns,
            }
        }
    }

    pub mod macros {
        #[macro_export]
        macro_rules! matrix {
            ($(matrix:expr)*) => {
                crate::matrix2::Matrix::zeros()
            };
        }

        pub use matrix;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_from_arrays() {
            let matrix = Matrix::from_arrays(&[
                &[1.0, 1.0, 1.0, 1.0],
                &[1.0, 1.0, 1.0, 1.0],
                &[1.0, 1.0, 1.0, 1.0],
                &[1.0, 1.0, 1.0, 1.0],
                &[1.0, 1.0, 1.0, 1.0],
            ]);
        }
    }
}

pub mod matrix {
    pub use std::ops::{Add, Mul, Sub};

    pub trait MatrixTraits: Clone + Default {}
    impl<T: Clone + Default> MatrixTraits for T {}

    pub struct Matrix<T: MatrixTraits> {
        inner: Vec<Vec<T>>,
        rows: usize,
        columns: usize,
    }

    impl Matrix<usize> {
        pub fn new(matrix: Vec<Vec<usize>>) -> Result<Self, &'static str> {
            let rows = matrix.len();
            let columns = matrix[0].len();

            for column in matrix.iter() {
                if column.len() != columns {
                    return Err("Matrix is malformed. Inconsistent column size.");
                }
            }

            Ok(Self {
                inner: matrix,
                rows,
                columns,
            })
        }

        pub fn new_vector(vector: Vec<usize>) -> Result<Self, &'static str> {
            Self::new(vec![vector; 1])
        }

        pub fn zeros(rows: usize, columns: usize) -> Self {
            let matrix = vec![vec![0; columns]; rows];
            Self::new(matrix).expect("Failed to create identity matrix.")
        }

        pub fn identity(rows: usize, columns: usize) -> Self {
            if rows != columns {
                panic!("Cannot create non-square identity matrix.");
            }

            let mut matrix = vec![vec![0; columns]; rows];

            for (row_index, column) in matrix.iter_mut().enumerate() {
                column[row_index] = 1;
            }

            Self::new(matrix).expect("Failed to create identity matrix.")
        }

        pub fn get(&self, row: usize, column: usize) -> usize {
            self.inner[row][column]
        }

        pub fn set(&mut self, row: usize, column: usize, value: usize) {
            self.inner[row][column] = value
        }

        pub fn dimensions(&self) -> (usize, usize) {
            (self.rows, self.columns)
        }

        pub fn inner(&self) -> &Vec<Vec<usize>> {
            &self.inner
        }
    }

    impl Add for &Matrix<usize> {
        type Output = Matrix<usize>;

        fn add(self, rhs: Self) -> Self::Output {
            if self.rows != rhs.rows || self.columns != rhs.columns {
                panic!("Matices are not of the same size/shape.");
            }

            let mut sum = Matrix::zeros(self.rows, self.columns);

            for ((self_column, rhs_column), sum_column) in
                self.inner.iter().zip(&rhs.inner).zip(&mut sum.inner)
            {
                for ((self_el, rhs_el), sum_el) in
                    self_column.iter().zip(rhs_column).zip(sum_column)
                {
                    *sum_el = *self_el + *rhs_el;
                }
            }

            sum
        }
    }

    impl Sub for &Matrix<usize> {
        type Output = Matrix<usize>;

        fn sub(self, rhs: Self) -> Self::Output {
            if self.rows != rhs.rows || self.columns != rhs.columns {
                panic!("Matices are not of the same size/shape.");
            }

            let mut difference = Matrix::zeros(self.rows, self.columns);

            for ((self_column, rhs_column), sum_column) in
                self.inner.iter().zip(&rhs.inner).zip(&mut difference.inner)
            {
                for ((self_el, rhs_el), sum_el) in
                    self_column.iter().zip(rhs_column).zip(sum_column)
                {
                    *sum_el = *self_el - *rhs_el;
                }
            }

            difference
        }
    }

    impl Mul for &Matrix<usize> {
        type Output = Matrix<usize>;

        fn mul(self, rhs: Self) -> Self::Output {
            if self.columns != rhs.rows {
                panic!("Matrix multiplication is not possible due to incompatible column/rows.");
            }

            let mut product = Matrix::zeros(self.rows, rhs.columns);

            for row in 0..product.rows {
                for column in 0..product.columns {
                    for i in 0..self.columns {
                        product.set(
                            row,
                            column,
                            product.get(row, column) + self.get(row, i) * rhs.get(i, column),
                        );
                    }
                }
            }

            product
        }
    }

    /// Note: Many of these test have to be checked manually. I was lazy.
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            assert!(
                Matrix::new(vec![
                    vec![1, 2, 3],
                    vec![4, 5, 6],
                    vec![4, 5, 6],
                    vec![7, 8, 9]
                ])
                .is_ok(),
                "Simple creation of valid matrix did not work."
            );

            assert!(
                Matrix::new(vec![
                    vec![1, 2, 3],
                    vec![4, 5, 6],
                    vec![4, 5, 6, 7, 8],
                    vec![7, 8, 9],
                ])
                .is_err(),
                "Matrix does not allow inconsistently sized columns in matrix."
            );
        }

        #[test]
        fn test_zero() {
            let matrix = Matrix::zeros(3, 5);
            assert!(matrix.inner.len() == 3, "Dimensions are incorrect.");

            for column in matrix.inner.iter() {
                assert!(column.iter().sum::<usize>() == 0, "Zero matrix is not 0.");
                assert!(column.len() == 5, "Dimensions are incorrect.");
            }
        }

        #[test]
        fn test_identity() {
            let matrix = Matrix::identity(3, 3);
            assert!(matrix.inner.len() == 3, "Dimensions are incorrect.");

            for (row_index, column) in matrix.inner.iter().enumerate() {
                assert!(
                    column.iter().sum::<usize>() == 1,
                    "Identity matrix missing at least one '1'."
                );
                assert!(
                    column[row_index] == 1,
                    "Identity has a '1' at the wrong location."
                );
                assert!(column.len() == 3, "Dimensions are incorrect.");
            }
        }

        #[test]
        fn test_set_and_get() {
            let mut matrix = Matrix::zeros(3, 5);
            matrix.set(2, 4, 42);
            assert!(matrix.get(2, 4) == 42);
        }

        #[test]
        fn test_add() {
            let matrix_1 = Matrix::identity(2, 2);
            let matrix_2 = Matrix::identity(2, 2);
            let matrix_3 = &matrix_1 + &matrix_2;
            println!("{:?}", matrix_3.inner);
        }

        #[test]
        fn test_sub() {
            let matrix_1 = Matrix::identity(2, 2);
            let matrix_2 = Matrix::identity(2, 2);
            let matrix_3 = &matrix_1 - &matrix_2;
            println!("{:?}", matrix_3.inner);
        }

        #[test]
        fn test_mul() {
            let matrix_1 = Matrix::identity(2, 2);
            let matrix_2 = Matrix::identity(2, 2);
            let matrix_3 = &matrix_1 * &matrix_2;
            println!("{:?}", matrix_3.inner);
        }
    }
}
