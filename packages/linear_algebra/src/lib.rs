pub mod matrix {
    use std::{
        fmt::Debug,
        ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Range, Sub, SubAssign},
    };

    pub trait DataTrait:
        Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + SubAssign
        + Mul<Output = Self>
        + MulAssign
        + Div<Output = Self>
        + Neg<Output = Self>
        + PartialEq
        + Clone
        + Copy
        + Default
        + Debug
    {
        fn zero() -> Self;
        fn one() -> Self;

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
    }

    impl DataTrait for f64 {
        fn zero() -> Self {
            0.0
        }

        fn one() -> Self {
            1.0
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

        pub fn scalar(&mut self, scalar: Data) {
            for val in self.data.iter_mut() {
                *val *= scalar;
            }
        }

        pub fn slice(&self, range_row: Range<usize>, range_column: Range<usize>) -> Vec<&Data> {
            let range_row_len = range_row.len();
            let range_column_len = range_column.len();

            if range_row_len > self.rows || range_column_len > self.columns {
                panic!("Argument ranges are outside of base matrix scope.");
            }

            let mut slice_vec = Vec::new();

            for i in range_row {
                for j in range_column.clone() {
                    slice_vec.push(&self.data[i * self.columns + j]);
                }
            }

            slice_vec
        }

        pub fn data(&self) -> &[Data] {
            &self.data
        }

        pub fn row(&self, row: usize) -> Vec<&Data> {
            self.slice(row..row + 1, 0..self.columns)
        }

        pub fn rows(&self) -> usize {
            self.rows
        }

        pub fn column(&self, column: usize) -> Vec<&Data> {
            self.slice(0..self.rows, column..column + 1)
        }

        pub fn columns(&self) -> usize {
            self.columns
        }

        pub fn swap_rows(&mut self, this: usize, that: usize) {
            let mut buf;

            for column in 0..self.columns {
                buf = self.index(this, column).clone();
                *self.index_mut(this, column) = *self.index(that, column);
                *self.index_mut(that, column) = buf;
            }
        }

        pub fn swap_columns(&mut self, this: usize, that: usize) {
            let mut buf;

            for row in 0..self.rows {
                buf = self.index(row, this).clone();
                *self.index_mut(row, this) = *self.index(row, that);
                *self.index_mut(row, that) = buf;
            }
        }

        pub fn set_row(&mut self, row: usize, matrix: &Self) {
            for column in 0..self.columns {
                *self.index_mut(row, column) = *matrix.index(0, column);
            }
        }

        pub fn set_column(&mut self, column: usize, matrix: &Self) {
            for row in 0..self.rows {
                *self.index_mut(row, column) = *matrix.index(row, 0);
            }
        }

        pub fn push_row(&mut self, mut matrix: Self) {
            if matrix.columns != self.columns && self.rows != 0 {
                panic!("Matrix dimensions are not compatible.");
            }

            if self.rows == 0 {
                self.columns = matrix.columns;
            }

            self.data.append(&mut matrix.data);
            self.rows += 1;
        }
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
                neg.data[index] = -self.data[index];
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
                sum.data[index] = self.data[index] + rhs.data[index];
            }

            sum
        }
    }

    impl<Data: DataTrait> AddAssign<&Matrix<Data>> for Matrix<Data> {
        fn add_assign(&mut self, rhs: &Matrix<Data>) {
            if self.rows != rhs.rows || self.columns != rhs.columns {
                panic!("Addition cannot be performed on matrices of different dimensions.")
            }

            for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
                *lhs = *lhs + *rhs;
            }
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
                sum.data[index] = self.data[index] - rhs.data[index];
            }

            sum
        }
    }

    impl<Data: DataTrait> SubAssign<&Matrix<Data>> for Matrix<Data> {
        fn sub_assign(&mut self, rhs: &Matrix<Data>) {
            if self.rows != rhs.rows || self.columns != rhs.columns {
                panic!("Subtraction cannot be performed on matrices of different dimensions.")
            }

            for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
                *lhs = *lhs - *rhs;
            }
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
                        *product.index_mut(product_index_row, product_index_column) = *product
                            .index(product_index_row, product_index_column)
                            + (*self.index(product_index_row, index_column_row)
                                * *rhs.index(index_column_row, product_index_column));
                    }
                }
            }

            product
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn should_panic() {
            std::panic::set_hook(Box::new(|_info| {
                // do nothing
            }));

            let _ = std::panic::catch_unwind(|| {
                panic!("test panic");
            });
        }

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

        mod test_scalar {
            use super::*;

            #[test]
            fn scalar() {
                let mut matrix = Matrix::from_array([[1, 2, 3], [4, 5, 6]]);

                matrix.scalar(2);

                assert!(matrix == Matrix::from_array([[2, 4, 6], [8, 10, 12],]))
            }
        }

        mod test_slice {
            use super::*;

            #[test]
            fn slice() {
                let matrix = Matrix::from_array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);

                let slice = matrix.slice(1..3, 1..3);
                let check = vec![6, 7, 10, 11];
                let check_ref = check.iter().collect::<Vec<&i64>>();

                assert!(slice == check_ref);
            }
        }

        mod test_row_column {
            use super::*;

            #[test]
            fn row() {
                let matrix = Matrix::from_array([
                    [1, 2, 3, 4],
                    [5, 6, 7, 8],
                    [9, 10, 11, 12],
                    [13, 14, 15, 16],
                ]);

                let check = vec![9, 10, 11, 12];
                let check_ref = check.iter().collect::<Vec<&i64>>();

                assert!(matrix.row(2) == check_ref);
            }

            #[test]
            fn column() {
                let matrix = Matrix::from_array([
                    [1, 2, 3, 4],
                    [5, 6, 7, 8],
                    [9, 10, 11, 12],
                    [13, 14, 15, 16],
                ]);

                let check = vec![3, 7, 11, 15];
                let check_ref = check.iter().collect::<Vec<&i64>>();

                assert!(matrix.column(2) == check_ref);
            }
        }

        mod test_swap_rows_columns {
            use super::*;

            #[test]
            fn swap_rows() {
                let mut matrix = Matrix::from_array([
                    [1, 2, 3, 4],
                    [5, 6, 7, 8],
                    [9, 10, 11, 12],
                    [13, 14, 15, 16],
                ]);
                let check = Matrix::from_array([
                    [1, 2, 3, 4],
                    [9, 10, 11, 12],
                    [5, 6, 7, 8],
                    [13, 14, 15, 16],
                ]);

                matrix.swap_rows(1, 2);

                assert!(matrix == check);
            }

            #[test]
            fn swap_columns() {
                let mut matrix = Matrix::from_array([
                    [1, 2, 3, 4],
                    [5, 6, 7, 8],
                    [9, 10, 11, 12],
                    [13, 14, 15, 16],
                ]);
                let check = Matrix::from_array([
                    [1, 3, 2, 4],
                    [5, 7, 6, 8],
                    [9, 11, 10, 12],
                    [13, 15, 14, 16],
                ]);

                matrix.swap_columns(1, 2);

                assert!(matrix == check);
            }
        }

        mod set_row_column {
            use super::*;

            #[test]
            fn set_row() {
                let mut matrix = Matrix::from_array([
                    [1, 2, 3, 4],
                    [5, 6, 7, 8],
                    [9, 10, 11, 12],
                    [13, 14, 15, 16],
                ]);
                let check = Matrix::from_array([
                    [1, 2, 3, 4],
                    [5, 6, 7, 8],
                    [-1, -2, -3, -4],
                    [13, 14, 15, 16],
                ]);

                matrix.set_row(2, &Matrix::from_array([[-1, -2, -3, -4]]));

                assert!(matrix == check, "{matrix:?}");
            }

            #[test]
            fn set_column() {
                let mut matrix = Matrix::from_array([
                    [1, 2, 3, 4],
                    [5, 6, 7, 8],
                    [9, 10, 11, 12],
                    [13, 14, 15, 16],
                ]);
                let check = Matrix::from_array([
                    [1, 2, -1, 4],
                    [5, 6, -2, 8],
                    [9, 10, -3, 12],
                    [13, 14, -4, 16],
                ]);

                matrix.set_column(2, &Matrix::from_array([[-1], [-2], [-3], [-4]]));

                assert!(matrix == check, "{matrix:?}");
            }
        }

        mod test_push_row {
            use super::*;

            #[test]
            fn push_row() {
                let mut matrix = Matrix::from_array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
                let check = Matrix::from_array([
                    [1, 2, 3, 4],
                    [5, 6, 7, 8],
                    [9, 10, 11, 12],
                    [13, 14, 15, 16],
                ]);

                matrix.push_row(Matrix::from_array([[13, 14, 15, 16]]));

                assert!(matrix == check, "{matrix:?}");
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

                assert!((&matrix + &matrix_neg) == Matrix::zeros::<2, 5>());
            }

            #[test]
            #[should_panic]
            fn add_panic() {
                should_panic();
                let matrix_a = Matrix::from_array([[1, 2, 3, 4, 5, 11], [6, 7, 8, 9, 10, 11]]);
                let matrix_b = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                &matrix_a + &matrix_b;
            }

            #[test]
            fn add_assign() {
                let mut matrix = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                let matrix_neg = Matrix::from_array([[-1, -2, -3, -4, -5], [-6, -7, -8, -9, -10]]);
                matrix += &matrix_neg;

                assert!(matrix == Matrix::zeros::<2, 5>());
            }
        }

        mod test_sub {
            use super::*;

            #[test]
            fn sub() {
                let matrix = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                let matrix_neg = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);

                assert!((&matrix - &matrix_neg) == Matrix::zeros::<2, 5>());
            }

            #[test]
            #[should_panic]
            fn sub_panic() {
                should_panic();
                let matrix_a = Matrix::from_array([[1, 2, 3, 4, 5, 11], [6, 7, 8, 9, 10, 11]]);
                let matrix_b = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                &matrix_a - &matrix_b;
            }

            #[test]
            fn sub_assign() {
                let mut matrix = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                let matrix_neg = Matrix::from_array([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]]);
                matrix -= &matrix_neg;

                assert!(matrix == Matrix::zeros::<2, 5>());
            }
        }

        mod test_mul {
            use super::*;

            #[test]
            fn mul() {
                let matrix_a = Matrix::from_array([[1, 2, 3], [4, 5, 6]]);
                let matrix_b = matrix_a.transpose();
                assert!(
                    (&matrix_a * &matrix_b)
                        == Matrix::from_array([
                            [(1 * 1 + 2 * 2 + 3 * 3), (1 * 4 + 2 * 5 + 3 * 6)],
                            [(4 * 1 + 5 * 2 + 6 * 3), (4 * 4 + 5 * 5 + 6 * 6)]
                        ])
                );
            }

            #[test]
            #[should_panic]
            fn mul_panic() {
                should_panic();
                let matrix = Matrix::from_array([[1, 2, 3], [4, 5, 6]]);
                &matrix * &matrix;
            }
        }
    }
}

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
