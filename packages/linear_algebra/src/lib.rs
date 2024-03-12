// pub type Vectorfunny cute catsRow<Data, const ROWS: usize> = matrix::Matrix<Data, ROWS, 1>;
// pub type VectorCol<Data, const COLUMNS: usize> = matrix::Matrix<Data, 1, COLUMNS>;

pub mod matrix {
    use std::{
        fmt::Debug,
        ops::{Add, Div, Index, IndexMut, Mul, Neg, Range, Sub},
        slice::SliceIndex,
    };

    pub trait DataTrait:
        Add + Sub + Mul + Div + PartialEq + Clone + Default + Debug + Sized
    {
        fn zero() -> Self;
        fn one() -> Self;

        // TODO: Hmm these are not that great...
        fn neg(&self) -> Self;
        fn add(&self, rhs: &Self) -> Self;
        fn mul(&self, rhs: &Self) -> Self;
        fn eqq(&self, rhs: &Self) -> bool {
            *self == *rhs
        }
    }

    impl DataTrait for i64 {
        fn zero() -> Self {
            0
        }

        fn one() -> Self {
            1
        }

        fn neg(&self) -> Self {
            -self
        }

        fn add(&self, rhs: &Self) -> Self {
            self + rhs
        }

        fn mul(&self, rhs: &Self) -> Self {
            self * rhs
        }
    }

    impl DataTrait for f64 {
        fn zero() -> Self {
            0.0
        }

        fn one() -> Self {
            1.0
        }

        fn neg(&self) -> Self {
            -self
        }

        fn add(&self, rhs: &Self) -> Self {
            self + rhs
        }

        fn mul(&self, rhs: &Self) -> Self {
            self * rhs
        }

        fn eqq(&self, rhs: &Self) -> bool {
            self - rhs < Self::EPSILON
        }
    }

    #[derive(Clone)]
    pub struct Matrix<Data: DataTrait> {
        data: Vec<Data>,
        rows: usize,
        columns: usize,
    }

    impl<Data: DataTrait> Matrix<Data> {
        pub fn from_array<const ROWS: usize, const COLUMNS: usize>(
            data: [[Data; COLUMNS]; ROWS],
        ) -> Self {
            Self {
                data: data.iter().flatten().cloned().collect(),
                rows: ROWS,
                columns: COLUMNS,
            }
        }

        pub fn from_slice<const ROWS: usize, const COLUMNS: usize>(
            data: &[&[Data; COLUMNS]; ROWS],
        ) -> Self {
            Self {
                data: data.iter().cloned().flatten().cloned().collect(),
                rows: ROWS,
                columns: COLUMNS,
            }
        }

        pub fn zeros<const ROWS: usize, const COLUMNS: usize>() -> Self {
            Self {
                data: vec![Data::zero(); ROWS * COLUMNS],
                rows: ROWS,
                columns: COLUMNS,
            }
        }

        pub fn identity<const ROWS: usize, const COLUMNS: usize>() -> Self {
            let mut identity = Self {
                data: vec![Data::zero(); ROWS * COLUMNS],
                rows: ROWS,
                columns: COLUMNS,
            };

            for element in identity.data.iter_mut().step_by(COLUMNS + 1) {
                *element = Data::one();
            }

            identity
        }

        pub fn index(&self, row: usize, column: usize) -> &Data {
            &self.data[row * self.columns + column]
        }

        pub fn index_mut(&mut self, row: usize, column: usize) -> &mut Data {
            &mut self.data[row * self.columns + column]
        }

        // pub fn slice(&self, row: usize, column: usize) -> &Data {
        //     todo!()
        // }

        // pub fn slice_mut(&mut self, row: usize, column: usize) -> &mut Data {
        //     todo!()
        // }

        pub fn transpose(&self) -> Self {
            let mut transpose = Self {
                data: vec![Data::zero(); self.rows * self.columns],
                rows: self.columns,
                columns: self.rows,
            };

            for row in 0..self.rows {
                for column in 0..self.columns {
                    *transpose.index_mut(column, row) = self.index(row, column).clone();
                }
            }

            transpose
        }

        // pub fn scalar(&mut self, scalar: f64) {
        //     for val in self.inner.iter_mut() {
        //         *val *= scalar;
        //     }
        // }

        // pub fn slice(&self, range_row: Range<usize>, range_column: Range<usize>) -> Matrix {
        //     let range_row_len = range_row.len();
        //     let range_column_len = range_column.len();

        //     if range_row_len > self.rows || range_column_len > self.columns {
        //         panic!("Argument ranges are outside of base matrix scope.");
        //     }

        //     let mut matrix = Matrix {
        //         inner: vec![0.0; range_row_len * range_column_len],
        //         rows: range_row_len,
        //         columns: range_column_len,
        //     };

        //     for row in range_row.clone() {
        //         for column in range_column.clone() {
        //             matrix[(row - range_row.start, column - range_column.start)] =
        //                 self[(row, column)];
        //         }
        //     }

        //     matrix
        // }

        // pub fn inner(&self) -> &[f64] {
        //     return &self.inner;
        // }

        // pub fn row(&self, row: usize) -> Self {
        //     self.slice(row..row + 1, 0..self.columns)
        // }

        // pub fn rows(&self) -> usize {
        //     self.rows
        // }

        // pub fn column(&self, column: usize) -> Self {
        //     self.slice(0..self.rows, column..column + 1)
        // }

        // pub fn columns(&self) -> usize {
        //     self.columns
        // }

        // pub fn swap_rows(&mut self, this: usize, that: usize) {
        //     let mut buf;

        //     for column in 0..self.columns {
        //         buf = self[(this, column)];
        //         self[(this, column)] = self[(that, column)];
        //         self[(that, column)] = buf;
        //     }
        // }

        // pub fn swap_columns(&mut self, this: usize, that: usize) {
        //     let mut buf;

        //     for row in 0..self.rows {
        //         buf = self[(row, this)];
        //         self[(row, this)] = self[(row, that)];
        //         self[(row, that)] = buf;
        //     }
        // }

        // pub fn set_row(&mut self, row: usize, matrix: Matrix) {
        //     for column in 0..self.columns {
        //         self[(row, column)] = matrix[(0, column)];
        //     }
        // }

        // pub fn set_column(&mut self, column: usize, matrix: Matrix) {
        //     for row in 0..self.rows {
        //         self[(row, column)] = matrix[(row, 0)];
        //     }
        // }

        // pub fn push_row(&mut self, mut matrix: Matrix) {
        //     if matrix.columns != self.columns && self.rows != 0 {
        //         panic!("Matrix dimensions are not compatible.");
        //     }

        //     if self.rows == 0 {
        //         self.columns = matrix.columns;
        //     }

        //     self.inner.append(&mut matrix.inner);
        //     self.rows += 1;
        // }
    }

    impl<Data: DataTrait> std::fmt::Debug for Matrix<Data> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut fmt_vec = Vec::with_capacity(self.rows);

            for row in 0..self.rows {
                let mut columns = Vec::with_capacity(self.columns);

                for column in 0..self.columns {
                    columns.push(self.index(row, column));
                }

                fmt_vec.push(columns);
            }

            f.debug_struct("Matrix")
                .field("rows", &self.rows)
                .field("columns", &self.columns)
                .field("data", &fmt_vec)
                .finish()
        }
    }

    impl<Data: DataTrait> PartialEq for Matrix<Data> {
        fn eq(&self, other: &Self) -> bool {
            if self.rows != other.rows || self.columns != other.columns {
                return false;
            }

            for (lhs, rhs) in self.data.iter().zip(other.data.iter()) {
                if !lhs.eqq(rhs) {
                    return false;
                }
            }

            true
        }
    }

    impl<Data: DataTrait> Neg for &Matrix<Data> {
        type Output = Matrix<Data>;

        fn neg(self) -> Self::Output {
            let mut neg = self.to_owned();

            for index in 0..self.data.len() {
                neg.data[index] = DataTrait::neg(&self.data[index]);
            }

            neg
        }
    }

    impl<Data: DataTrait> Add for &Matrix<Data> {
        type Output = Matrix<Data>;

        fn add(self, rhs: Self) -> Self::Output {
            if self.rows != rhs.rows || self.columns != rhs.columns {
                panic!("Addition cannot be performed on matrices of different dimensions.")
            }

            let mut sum = self.to_owned();

            for index in 0..self.data.len() {
                sum.data[index] = DataTrait::add(&self.data[index], &rhs.data[index]);
            }

            sum
        }
    }

    impl<Data: DataTrait> Sub for &Matrix<Data> {
        type Output = Matrix<Data>;

        fn sub(self, rhs: Self) -> Self::Output {
            if self.rows != rhs.rows || self.columns != rhs.columns {
                panic!("Subtraction cannot be performed on matrices of different dimensions.")
            }

            let mut sum = self.to_owned();

            for index in 0..self.data.len() {
                sum.data[index] =
                    DataTrait::add(&self.data[index], &DataTrait::neg(&rhs.data[index]));
            }

            sum
        }
    }

    impl<Data: DataTrait> Mul for &Matrix<Data> {
        type Output = Matrix<Data>;

        fn mul(self, rhs: Self) -> Self::Output {
            if self.columns != rhs.rows {
                panic!("Matrix multiplication cannot be performed on matrices with incompatible dimensions.")
            }

            let mut product = Matrix {
                data: vec![Data::zero(); self.rows * rhs.columns],
                rows: self.rows,
                columns: rhs.columns,
            };

            for product_index_row in 0..self.rows {
                for product_index_column in 0..rhs.columns {
                    for index_column_row in 0..self.columns {
                        *product.index_mut(product_index_row, product_index_column) =
                            DataTrait::add(
                                product.index(product_index_row, product_index_column),
                                &DataTrait::mul(
                                    self.index(product_index_row, index_column_row),
                                    rhs.index(index_column_row, product_index_column),
                                ),
                            );
                    }
                }
            }

            product
        }
    }

    // pub mod macros {
    //     #[macro_export]
    //     macro_rules! matrix {
    //         [ $( [ $( $row:expr ),* ] $(,)* )* ] => {
    //             crate::matrix::Matrix::from_slices(
    //                 &[
    //                     $(
    //                         &[$($row),*]
    //                     ),*
    //                 ]
    //             )
    //         };

    //         ( $rows:expr, $columns:expr ) => {
    //             crate::matrix::Matrix::zeros(($rows), ($columns))
    //         };

    //         ( $rows_cols:expr ) => {
    //             crate::matrix::Matrix::zeros(($rows_cols), ($rows_cols))
    //         };
    //     }
    //     pub use matrix;

    //     #[macro_export]
    //     macro_rules! identity {
    //         ( $rows_cols:expr ) => {
    //             crate::matrix::Matrix::identity(($rows_cols))
    //         };
    //     }
    //     pub use identity;
    // }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod test_from_array {
            use super::*;

            #[test]
            fn from_array_f64() {
                let matrix_f64 = Matrix::from_array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);

                let mut check = 1.0;

                for row in 0..2 {
                    for column in 0..3 {
                        assert!(*matrix_f64.index(row, column) == check);
                        check += 1.0;
                    }
                }
            }

            #[test]
            fn from_array_i64() {
                let matrix_i64 = Matrix::from_array([[1, 2, 3], [4, 5, 6]]);

                let mut check = 1;

                for row in 0..2 {
                    for column in 0..3 {
                        assert!(*matrix_i64.index(row, column) == check);
                        check += 1;
                    }
                }
            }
        }

        mod test_from_slice {
            use super::*;

            #[test]
            fn from_slice_f64() {
                let matrix_f64 = Matrix::from_slice(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);

                let mut check = 1.0;

                for row in 0..2 {
                    for column in 0..3 {
                        assert!(*matrix_f64.index(row, column) == check);
                        check += 1.0;
                    }
                }
            }

            #[test]
            fn from_slice_i64() {
                let matrix_i64 = Matrix::from_slice(&[&[1, 2, 3], &[4, 5, 6]]);

                let mut check = 1;

                for row in 0..2 {
                    for column in 0..3 {
                        assert!(*matrix_i64.index(row, column) == check);
                        check += 1;
                    }
                }
            }
        }

        mod test_zeros {
            use super::*;

            #[test]
            fn zeros_f64() {
                let zeros = Matrix::<f64>::zeros::<3, 4>();

                for row in 0..3 {
                    for column in 0..4 {
                        assert!(*zeros.index(row, column) == f64::zero());
                    }
                }
            }

            #[test]
            fn zeros_i64() {
                let zeros = Matrix::<i64>::zeros::<3, 4>();

                for row in 0..3 {
                    for column in 0..4 {
                        assert!(*zeros.index(row, column) == i64::zero());
                    }
                }
            }
        }

        mod test_identity {
            use super::*;

            #[test]
            fn identity_f64() {
                let zeros = Matrix::<f64>::identity::<3, 4>();

                for row in 0..3 {
                    for column in 0..4 {
                        if row == column {
                            assert!(*zeros.index(row, column) == f64::one());
                        } else {
                            assert!(*zeros.index(row, column) == f64::zero());
                        }
                    }
                }
            }

            #[test]
            fn identity_i64() {
                let zeros = Matrix::<i64>::identity::<3, 4>();

                for row in 0..3 {
                    for column in 0..4 {
                        if row == column {
                            assert!(*zeros.index(row, column) == i64::one());
                        } else {
                            assert!(*zeros.index(row, column) == i64::zero());
                        }
                    }
                }
            }
        }

        mod test_index_and_index_mut {
            use super::*;

            #[test]
            fn index_mut() {
                let mut matrix = Matrix::<i64>::zeros::<3, 4>();

                let mut incr = 1;

                for row in 0..3 {
                    for column in 0..4 {
                        *matrix.index_mut(row, column) = incr;
                        assert!(*matrix.index(row, column) == incr);
                        incr += 1;
                    }
                }
            }
        }

        mod test_transpose {
            use super::*;

            #[test]
            fn transpose() {
                let transpose = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);

                let transposed = transpose.transpose();

                for row in 0..2 {
                    for column in 0..5 {
                        assert!(*transpose.index(row, column) == *transposed.index(column, row));
                    }
                }
            }
        }

        mod test_partial_eq {
            use super::*;

            #[test]
            fn partial_eq() {
                let matrix_a = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                let matrix_b = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                let matrix_c = Matrix::from_array([[1, 2, 0, 4, 5], [6, 7, 8, 9, 10]]);

                assert!(matrix_a == matrix_b);
                assert!(matrix_b != matrix_c);
            }
        }

        mod test_neg {
            use super::*;

            #[test]
            fn neg() {
                let matrix = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                let matrix_neg = Matrix::from_array([[-1, -2, -3, -4, -5], [-6, -7, -8, -9, -10]]);

                assert!(matrix == -&matrix_neg);
            }
        }

        mod test_add {
            use super::*;

            #[test]
            fn add() {
                let matrix = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                let matrix_neg = Matrix::from_array([[-1, -2, -3, -4, -5], [-6, -7, -8, -9, -10]]);

                assert!((&matrix + &matrix_neg) == Matrix::zeros::<2,5>());
            }

            #[test]
            #[should_panic]
            fn add_panic() {
                std::panic::set_hook(Box::new(|_info| {
                    // do nothing
                }));
            
                let _ = std::panic::catch_unwind(|| {
                    panic!("test panic");
                });

                let matrix_a = Matrix::from_array([[1, 2, 3, 4, 5, 11], [6, 7, 8, 9, 10, 11]]);
                let matrix_b = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                &matrix_a + &matrix_b;
            }
        }

        mod test_sub {
            use super::*;

            #[test]
            fn sub() {
                let matrix = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                let matrix_neg = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);

                assert!((&matrix - &matrix_neg) == Matrix::zeros::<2,5>());
            }

            #[test]
            #[should_panic]
            fn sub_panic() {
                std::panic::set_hook(Box::new(|_info| {
                    // do nothing
                }));
            
                let _ = std::panic::catch_unwind(|| {
                    panic!("test panic");
                });

                let matrix_a = Matrix::from_array([[1, 2, 3, 4, 5, 11], [6, 7, 8, 9, 10, 11]]);
                let matrix_b = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                &matrix_a - &matrix_b;
            }
        }

        mod test_mul {
            use super::*;

            #[test]
            fn mul() {
                let matrix_a= Matrix::from_array([[1, 2, 3], [4, 5, 6]]);
                let matrix_b = matrix_a.transpose();
                assert!((&matrix_a * &matrix_b) == Matrix::from_array([[(1*1 + 2*2 + 3*3), (1*4 + 2*5 + 3*6)],[(4*1 + 5*2 + 6*3), (4*4 + 5*5 + 6*6)]]));
            }

            #[test]
            #[should_panic]
            fn mul_panic() {
                std::panic::set_hook(Box::new(|_info| {
                    // do nothing
                }));
            
                let _ = std::panic::catch_unwind(|| {
                    panic!("test panic");
                });

                let matrix = Matrix::from_array([[1, 2, 3], [4, 5, 6]]);
                &matrix * &matrix;
            }
        }
    }
}

//         mod test_swap {
//             use super::*;

//             #[test]
//             fn swap_rows() {
//                 let mut matrix = macros::matrix![[1.0, 2.0], [3.0, 4.0],];

//                 matrix.swap_rows(0, 1);

//                 assert!(matrix[(0, 0)] - 3.0 < f64::EPSILON);
//                 assert!(matrix[(0, 1)] - 4.0 < f64::EPSILON);
//                 assert!(matrix[(1, 0)] - 1.0 < f64::EPSILON);
//                 assert!(matrix[(1, 1)] - 2.0 < f64::EPSILON);
//             }

//             #[test]
//             fn swap_columns() {
//                 let mut matrix = macros::matrix![[1.0, 2.0], [3.0, 4.0],];

//                 matrix.swap_columns(0, 1);

//                 assert!(matrix[(0, 0)] - 2.0 < f64::EPSILON);
//                 assert!(matrix[(0, 1)] - 1.0 < f64::EPSILON);
//                 assert!(matrix[(1, 0)] - 4.0 < f64::EPSILON);
//                 assert!(matrix[(1, 1)] - 3.0 < f64::EPSILON);
//             }
//         }

//         mod test_slice {
//             use super::*;

//             #[test]
//             fn slice() {
//                 let matrix = macros::matrix![
//                     [1.0, 2.0, 3.0, 4.0],
//                     [5.0, 6.0, 7.0, 8.0],
//                     [9.0, 10.0, 11.0, 12.0],
//                 ];

//                 assert!(matrix.slice(0..1, 0..4) == macros::matrix![[1.0, 2.0, 3.0, 4.0]]);
//                 assert!(matrix.slice(0..3, 0..1) == macros::matrix![[1.0], [5.0], [9.0]]);
//                 assert!(
//                     matrix.slice(1..3, 1..4)
//                         == macros::matrix![[6.0, 7.0, 8.0], [10.0, 11.0, 12.0]]
//                 );
//             }
//         }


// /// TODO:
// /// Implement a vector type.
// /// - Refactor and extract aspects from the matrix type as traits,
// /// and share between both vector and matrix.
// ///
// /// There is currently no NEED for vector (though it would be nice/better).
// mod vector {
//     use super::*;

//     /// [Vector] which internally uses a nx1 [matrix::Matrix].
//     pub struct Vector {
//         inner: matrix::Matrix,
//         len: usize,
//     }

//     impl Vector {
//         fn from_slice(slice: &[f64]) -> Self {
//             Self {
//                 inner: matrix::Matrix::from_slices(&[slice]),
//                 len: slice.len(),
//             }
//         }

//         fn inner(&self) -> &matrix::Matrix {
//             &self.inner
//         }
//     }

//     pub mod macros {
//         #[macro_export]
//         macro_rules! vector {
//             [ $( $row:expr ),* ] => {
//                 crate::vector::Vector::from_slice(
//                     &[$($row),*]
//                 )
//             };

//             ( $len:expr ) => {
//                 crate::vector::Vector::from_slice(($len))
//             };
//         }
//         pub use vector;
//     }

//     #[cfg(test)]
//     mod tests {
//         use super::*;

//         #[test]
//         fn main() {
//             let vector = macros::vector![1.0, 2.0, 3.0];
//         }
//     }
// }

// pub mod utility {
//     use super::*;

//     /// Last column in matrix is seen as the sum of the values to the left,
//     /// where the values to the left are parametric.
//     /// Only solves n by n+1 matrices.
//     pub fn gauss_elimination(mut matrix: &mut matrix::Matrix) -> Option<matrix::Matrix> {
//         enum Direction {
//             Down,
//             Up,
//         }

//         /// Find a 'pivotable' point in a row given a column.
//         fn find_next_pivot_row(
//             matrix: &mut matrix::Matrix,
//             start_row: usize,
//             column: usize,
//         ) -> Option<usize> {
//             for row in start_row..matrix.rows() {
//                 if matrix[(row, column)] > f64::EPSILON {
//                     return Some(row);
//                 }
//             }

//             None
//         }

//         /// Normalize the row for a given pivot coordinate.
//         fn normalize_row(matrix: &mut matrix::Matrix, row: usize, divisor_column: usize) {
//             let divisor = matrix[(row, divisor_column)];

//             for column in 0..matrix.columns() {
//                 matrix[(row, column)] /= divisor;
//             }
//         }

//         /// Eliminate all rows based on pivot.
//         fn eliminate_columns(
//             matrix: &mut matrix::Matrix,
//             pivot_row: usize,
//             pivot_column: usize,
//             start_row: usize,
//             direction: Direction,
//         ) {
//             match direction {
//                 Direction::Down => {
//                     for row in start_row..matrix.rows() {
//                         if matrix[(row, pivot_column)] < f64::EPSILON {
//                             continue;
//                         }

//                         let pivot_factor = matrix[(row, pivot_column)];

//                         for column in 0..matrix.columns() {
//                             matrix[(row, column)] -= matrix[(pivot_row, column)] * pivot_factor;
//                         }
//                     }
//                 }
//                 Direction::Up => {
//                     for row in (0..start_row).rev() {
//                         if matrix[(row, pivot_column)] < f64::EPSILON {
//                             continue;
//                         }

//                         let pivot_factor = matrix[(row, pivot_column)];

//                         for column in 0..matrix.columns() {
//                             matrix[(row, column)] -= matrix[(pivot_row, column)] * pivot_factor;
//                         }
//                     }
//                 }
//             }
//         }

//         if matrix.rows() + 1 != matrix.columns() {
//             return None;
//         }

//         // "Downward"
//         for row_and_column in 0..matrix.rows() - 1 {
//             if let Some(pivot_row) = find_next_pivot_row(matrix, row_and_column, row_and_column) {
//                 matrix.swap_rows(row_and_column, pivot_row);
//                 normalize_row(&mut matrix, row_and_column, row_and_column);
//                 eliminate_columns(
//                     &mut matrix,
//                     row_and_column,
//                     row_and_column,
//                     row_and_column + 1,
//                     Direction::Down,
//                 );
//             } else {
//                 return None;
//             }
//         }

//         // "Upward"
//         for row_and_column in (1..matrix.rows()).rev() {
//             normalize_row(&mut matrix, row_and_column, row_and_column);
//             eliminate_columns(
//                 &mut matrix,
//                 row_and_column,
//                 row_and_column,
//                 row_and_column,
//                 Direction::Up,
//             );
//         }

//         None
//     }

//     pub fn rotate_x(matrix: &matrix::Matrix, radians: f64) -> matrix::Matrix {
//         let rotation_matrix = matrix::macros::matrix![
//             [1.0, 0.0, 0.0],
//             [0.0, radians.cos(), -radians.sin()],
//             [0.0, radians.sin(), radians.cos()],
//         ];

//         matrix * &rotation_matrix
//     }

//     pub fn rotate_y(matrix: &matrix::Matrix, radians: f64) -> matrix::Matrix {
//         let rotation_matrix = matrix::macros::matrix![
//             [radians.cos(), 0.0, radians.sin()],
//             [0.0, 1.0, 0.0],
//             [-radians.sin(), 0.0, radians.cos()],
//         ];

//         matrix * &rotation_matrix
//     }

//     pub fn rotate_z(matrix: &matrix::Matrix, radians: f64) -> matrix::Matrix {
//         let rotation_matrix = matrix::macros::matrix![
//             [radians.cos(), -radians.sin(), 0.0],
//             [radians.sin(), radians.cos(), 0.0],
//             [0.0, 0.0, 1.0],
//         ];

//         matrix * &rotation_matrix
//     }

//     #[cfg(test)]
//     mod tests {
//         use super::*;

//         mod test_gauss_elimination {
//             use super::*;

//             #[test]
//             fn test_1() {
//                 let mut matrix = matrix::macros::matrix![
//                     [1.0, 0.0, 0.0, 1.0],
//                     [0.0, 1.0, 0.0, 2.0],
//                     [0.0, 0.0, 1.0, 3.0],
//                 ];

//                 gauss_elimination(&mut matrix);

//                 let expected = matrix::macros::matrix![
//                     [1.0, 0.0, 0.0, 1.0],
//                     [0.0, 1.0, 0.0, 2.0],
//                     [0.0, 0.0, 1.0, 3.0],
//                 ];

//                 check(matrix, expected);
//             }

//             #[test]
//             fn test_2() {
//                 let mut matrix = matrix::macros::matrix![
//                     [0.0, 1.0, 0.0, 2.0],
//                     [1.0, 0.0, 0.0, 1.0],
//                     [0.0, 0.0, 1.0, 3.0],
//                 ];

//                 gauss_elimination(&mut matrix);

//                 let expected = matrix::macros::matrix![
//                     [1.0, 0.0, 0.0, 1.0],
//                     [0.0, 1.0, 0.0, 2.0],
//                     [0.0, 0.0, 1.0, 3.0],
//                 ];

//                 check(matrix, expected);
//             }

//             #[test]
//             fn test_3() {
//                 let mut matrix = matrix::macros::matrix![
//                     [0.0, 0.0, 1.0, 3.0],
//                     [0.0, 1.0, 0.0, 2.0],
//                     [1.0, 0.0, 0.0, 1.0],
//                 ];

//                 gauss_elimination(&mut matrix);

//                 let expected = matrix::macros::matrix![
//                     [1.0, 0.0, 0.0, 1.0],
//                     [0.0, 1.0, 0.0, 2.0],
//                     [0.0, 0.0, 1.0, 3.0],
//                 ];

//                 check(matrix, expected);
//             }

//             #[test]
//             fn test_4() {
//                 let mut matrix = matrix::macros::matrix![
//                     [5.0, 3.0, 7.0, 1.0],
//                     [2.0, 4.0, 9.0, 3.0],
//                     [11.0, 7.0, 1.0, 4.0],
//                 ];

//                 gauss_elimination(&mut matrix);

//                 let expected = matrix::macros::matrix![
//                     [1.0, 0.0, 0.0, -75.0 / 214.0],
//                     [0.0, 1.0, 0.0, 243.0 / 214.0],
//                     [0.0, 0.0, 1.0, -10.0 / 107.0],
//                 ];

//                 check(matrix, expected);
//             }

//             fn check(matrix: matrix::Matrix, expected: matrix::Matrix) {
//                 assert!(matrix == expected);
//             }
//         }
//     }
// }
