pub mod matrix2 {
    use std::ops::{Add, Index, IndexMut, Mul, Sub};

    /// Dynamically sized n-dim matrices;
    pub struct Matrix {
        inner: Vec<f64>,
        rows: usize,
        columns: usize,
    }

    impl Matrix {
        pub fn from_arrays(arrays: &[&[f64]]) -> Self {
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
            if self.rows != rhs.rows || self.columns != rhs.columns {
                panic!("Addition cannot be performed on matrices of different dimensions.")
            }

            let mut sum = Matrix::zeros(self.rows, self.columns);

            for index in 0..self.inner.len() {
                sum[index] = self[index] * rhs[index];
            }

            sum
        }
    }


    pub mod macros {
        #[macro_export]
        macro_rules! matrix {
            [ $( [ $( $row:expr ),* ] $(,)* )* ] => {
                crate::matrix2::Matrix::from_arrays(
                    &[
                        $(
                            &[$($row),*]
                        ),*
                    ]
                )
            };

            ( $rows:expr, $columns:expr ) => {
                crate::matrix2::Matrix::zeros(($rows), ($columns))
            };

            ( $rows_cols:expr ) => {
                crate::matrix2::Matrix::zeros(($rows_cols), ($rows_cols))
            };
        }
        pub use matrix;

        #[macro_export]
        macro_rules! identity {
            ( $rows_cols:expr ) => {
                crate::matrix2::Matrix::identity(($rows_cols))
            };
        }
        pub use identity;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        static EPSILON: f64 = std::f64::EPSILON;

        mod test_from_arrays {
            use super::*;

            #[test]
            fn from_arrays() {
                let matrix = Matrix::from_arrays(&[
                    &[1.0, 2.0, 3.0, 4.0],
                    &[5.0, 6.0, 7.0, 8.0],
                    &[9.0, 10.0, 11.0, 12.0],
                    &[13.0, 14.0, 15.0, 16.0],
                    &[17.0, 18.0, 19.0, 20.0],
                ]);

                check(matrix);
            }

            #[test]
            fn from_arrays_with_macro() {
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
                check(matrix,3 ,4);
            }

            #[test]
            fn zeros_with_macro() {
                let matrix = macros::matrix!(3,4);
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

                matrix[4*0] = 1.0;
                matrix[4*1] = 1.0;
                matrix[4*2] = 1.0;
                matrix[4*3] = 1.0;

                check_usize(matrix);
            }

            #[test]
            fn index_usize_usize() {
                let mut matrix = macros::matrix!(4);

                matrix[(0,0)] = 1.0;
                matrix[(1,1)] = 1.0;
                matrix[(2,2)] = 1.0;
                matrix[(3,3)] = 1.0;

                check_usize_usize(matrix);
            }

            #[test]
            #[should_panic]
            fn index_out_of_bounds_usize() {
                // Ignore stacktrace output
                std::panic::set_hook(Box::new(|_| {}));

                let mut matrix = macros::matrix!(4);
                matrix[4*4] = 1.0;
            }

            #[test]
            #[should_panic]
            fn index_out_of_bounds_usize_usize() {
                // Ignore stacktrace output
                std::panic::set_hook(Box::new(|_| {}));

                let mut matrix = macros::matrix!(4);
                matrix[(4,4)] = 1.0;
            }

            fn check_usize(matrix: Matrix) {
                assert!((matrix[4*0] - 1.0).abs() < EPSILON);
                assert!((matrix[4*1] - 1.0).abs() < EPSILON);
                assert!((matrix[4*2] - 1.0).abs() < EPSILON);
                assert!((matrix[4*3] - 1.0).abs() < EPSILON);
            }

            fn check_usize_usize(matrix: Matrix) {
                assert!((matrix[(0,0)] - 1.0).abs() < EPSILON);
                assert!((matrix[(1,1)] - 1.0).abs() < EPSILON);
                assert!((matrix[(2,2)] - 1.0).abs() < EPSILON);
                assert!((matrix[(3,3)] - 1.0).abs() < EPSILON);
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
                let mut a = macros::matrix!(4);
                a[(0,0)] = 1.0;
                a[(1,1)] = 2.0;
                a[(2,2)] = 3.0;
                a[(3,3)] = 4.0;

                let mut b = macros::matrix!(4);
                b[(0,0)] = 5.0;
                b[(1,1)] = 6.0;
                b[(2,2)] = 7.0;
                b[(3,3)] = 8.0;

                let c = &a * &b;
                check(c);
            }

            #[test]
            #[should_panic]
            fn mul_panic() {
                // Ignore stacktrace output
                std::panic::set_hook(Box::new(|_| {}));
                let a = macros::identity!(4);
                let b = macros::identity!(5);
                let _c = &a * &b;
            }

            fn check(matrix: Matrix) {
                let mut offset = 0;

                for (index, element) in matrix.inner().iter().enumerate() {
                    if index == matrix.columns * offset + offset {
                        offset += 1;
                    } else {
                        assert!(*element == 0.0);
                    }
                }

                assert!((matrix[(0,0)] - 5.0).abs() < EPSILON);
                assert!((matrix[(1,1)] - 12.0).abs() < EPSILON);
                assert!((matrix[(2,2)] - 21.0).abs() < EPSILON);
                assert!((matrix[(3,3)] - 32.0).abs() < EPSILON);
            }
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
