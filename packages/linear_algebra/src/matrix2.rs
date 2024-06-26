pub mod matrix2 {
    use std::{
        fmt::Debug,
        ops::{
            Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
        },
    };

    use super::vector::{VectorRow, VectorColumn};

    pub trait MatrixDataTrait:
        Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + SubAssign
        + Mul<Output = Self>
        + MulAssign
        + Div<Output = Self>
        + DivAssign
        + Neg<Output = Self>
        + PartialEq
        + PartialOrd
        + Copy
        + Clone
        + Default
        + Debug
    {
        fn zero() -> Self;
        fn one() -> Self;
        fn sqrt(&self) -> Self;
    }

    impl MatrixDataTrait for i64 {
        fn zero() -> Self {
            0
        }

        fn one() -> Self {
            1
        }

        fn sqrt(&self) -> Self {
            (*self as f64).sqrt() as Self
        }
    }

    impl MatrixDataTrait for f64 {
        fn zero() -> Self {
            0.0
        }

        fn one() -> Self {
            1.0
        }

        fn sqrt(&self) -> Self {
            (*self as f64).sqrt()
        }
    }

    #[derive(Clone, PartialEq, PartialOrd)]
    pub struct Matrix<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> {
        data: [[T; COLS]; ROWS],
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
        pub fn zeros() -> Self {
            Self {
                data: [[T::zero(); COLS]; ROWS],
            }
        }

        pub fn identity() -> Self {
            if ROWS != COLS {
                panic!("Identity matrix requires a quadratic form.")
            }

            let mut identity = Self {
                data: [[T::zero(); COLS]; ROWS],
            };

            for (i, row) in identity.data.iter_mut().enumerate() {
                row[i] = T::one();
            }

            identity
        }

        pub fn transpose(&self) -> Matrix<T, COLS, ROWS> {
            let mut matrix = Matrix::<T, COLS, ROWS>::zeros();

            for (i, &row) in self.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    matrix[j][i] = cell;
                }
            }

            matrix
        }

        pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, [T; COLS]> {
            self.into_iter()
        }

        pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, [T; COLS]> {
            self.into_iter()
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS> {
        fn default() -> Self {
            Self {
                data: [[T::zero(); COLS]; ROWS],
            }
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]> for Matrix<T, ROWS, COLS> {
        fn from(data: [[T; COLS]; ROWS]) -> Self {
            Self {
                data,
            }
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> From<&[Matrix<T, 1, COLS>; ROWS]> for Matrix<T, ROWS, COLS> {
        fn from(matrices: &[Matrix<T, 1, COLS>; ROWS]) -> Self {
            let mut data = [[T::zero(); COLS]; ROWS];

            for (i, matrix) in matrices.iter().enumerate() {
                for row in matrix.iter() {
                    for (j, &cell) in row.iter().enumerate() {
                        data[i][j] = cell;
                    }
                }
            }

            Self {
                data,
            }
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> From<&[VectorRow<T, COLS>; ROWS]> for Matrix<T, ROWS, COLS> {
        fn from(vectors: &[VectorRow<T, COLS>; ROWS]) -> Self {
            let mut data = [[T::zero(); COLS]; ROWS];

            for (i, vector) in vectors.iter().enumerate() {
                for row in vector.0.iter() {
                    for (j, &cell) in row.iter().enumerate() {
                        data[i][j] = cell;
                    }
                }
            }

            Self {
                data,
            }
        }
    }

    impl<T: MatrixDataTrait, const LENGTH: usize> From<VectorRow<T, LENGTH>> for Matrix<T, 1, LENGTH> {
        fn from(vector: VectorRow<T, LENGTH>) -> Self {
            vector.0
        }
    }

    impl<T: MatrixDataTrait, const LENGTH: usize> From<VectorColumn<T, LENGTH>> for Matrix<T, LENGTH, 1> {
        fn from(vector: VectorColumn<T, LENGTH>) -> Self {
            vector.0
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Index<usize>
        for Matrix<T, ROWS, COLS>
    {
        type Output = [T; COLS];

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> IndexMut<usize>
        for Matrix<T, ROWS, COLS>
    {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> IntoIterator
        for Matrix<T, ROWS, COLS>
    {
        type Item = [T; COLS];
        type IntoIter = core::array::IntoIter<Self::Item, ROWS>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }

    impl<'a, T: MatrixDataTrait, const ROWS: usize, const COLS: usize> IntoIterator
        for &'a Matrix<T, ROWS, COLS>
    {
        type Item = &'a [T; COLS];
        type IntoIter = std::slice::Iter<'a, [T; COLS]>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }

    impl<'a, T: MatrixDataTrait, const ROWS: usize, const COLS: usize> IntoIterator
        for &'a mut Matrix<T, ROWS, COLS>
    {
        type Item = &'a mut [T; COLS];
        type IntoIter = std::slice::IterMut<'a, [T; COLS]>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.iter_mut()
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::matrix2::Matrix;

    mod unit_tests {
        use super::*;

        #[test]
        fn new_test() {
            let matrix = Matrix::from([[1, 2, 3]]);

            // Check using iterators.
            for i in 0..3 {
                assert!(matrix[0][i] == (i as i64) + 1);
            }

            // Check using [PartialEq].
            assert!(matrix == Matrix::from([[1, 2, 3]]));
        }

        #[test]
        fn zeros_test() {
            let matrix = Matrix::<i64, 3, 3>::zeros();

            // Check using [Index].
            for row in matrix.iter() {
                for &cell in row.iter() {
                    assert!(cell == 0);
                }
            }

            // Check using [PartialEq].
            assert!(matrix == Matrix::from([
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0],
            ]));
        }

        #[test]
        fn default_test() {
            zeros_test();
        }

        #[test]
        fn identity_test() {
            let matrix = Matrix::<i64, 3, 3>::identity();

            // Check using [Index].
            for (i, row) in matrix.iter().enumerate() {
                for (j, _) in row.iter().enumerate() {
                    if i == j {
                        assert!(row[i] == 1);
                    } else {
                        assert!(row[j] == 0);
                    }
                }
            }

            // Check using [PartialEq].
            assert!(matrix == Matrix::from([
                [1, 0, 0],
                [0, 1, 0],
                [0, 0, 1],
            ]));
        }

        #[test]
        #[should_panic]
        fn identity_test_panic() {
            let _ = Matrix::<i64, 3, 4>::identity();
        }

        #[test]
        fn transpose_test() {
            let matrix = Matrix::from([
                [1, 2, 3, 4],
                [5, 6, 7, 8]
            ]);
            let matrix = matrix.transpose();

            // Only checking using [PartialEq].
            assert!(matrix == Matrix::from([
                [1, 5],
                [2, 6],
                [3, 7],
                [4, 8],
            ]));
        }

        #[test]
        fn from_array_test() {
            let matrix = Matrix::from([
                [1,2,3],
                [4,5,6],
                [7,8,9],
                [10,11,12],
            ]);

            assert!(matrix == Matrix::from([
                [1,2,3],
                [4,5,6],
                [7,8,9],
                [10,11,12],
            ]));
        }

        #[test]
        fn from_slice_of_row_matrices_test() {
            let matrices = [
                Matrix::from([[1,2,3]]),
                Matrix::from([[4,5,6]]),
                Matrix::from([[7,8,9]]),
                Matrix::from([[10,11,12]]),
            ];

            let matrix = Matrix::<i64, 4, 3>::from(&matrices);

            assert!(matrix == Matrix::from([
                [1,2,3],
                [4,5,6],
                [7,8,9],
                [10,11,12],
            ]));

            // let matrix = Matrix::<i64, 4, 4>::from(&matrices); // Will not even compile due to wrong dimensions (manual check)
            // let matrix = Matrix::<i64, 3, 3>::from(&matrices); // Will not even compile due to wrong dimensions (manual check)
        }

        #[test]
        fn from_slice_of_row_vectors_test() {
            use crate::matrix2::vector::VectorRow;

            let vectors = [
                VectorRow::new([[1,2,3]]),
                VectorRow::new([[4,5,6]]),
                VectorRow::new([[7,8,9]]),
                VectorRow::new([[10,11,12]]),
            ];

            let matrix = Matrix::<i64, 4, 3>::from(&vectors);

            assert!(matrix == Matrix::from([
                [1,2,3],
                [4,5,6],
                [7,8,9],
                [10,11,12],
            ]));

            // let matrix = Matrix::<i64, 4, 4>::from(&vectors); // Will not even compile due to wrong dimensions (manual check)
            // let matrix = Matrix::<i64, 3, 3>::from(&vectors); // Will not even compile due to wrong dimensions (manual check)
        }
    }
}

/// TODO: Some duplicate code for row and column vector. Not the best,
/// but it will make the type system happy.
mod vector {
    use super::matrix2::{Matrix, MatrixDataTrait};

    pub struct VectorRow<T: MatrixDataTrait, const LENGTH: usize>(pub Matrix<T, 1, LENGTH>);
    pub struct VectorColumn<T: MatrixDataTrait, const LENGTH: usize>(pub Matrix<T, LENGTH, 1>);

    impl<T: MatrixDataTrait, const LENGTH: usize> VectorRow<T, LENGTH> {
        pub fn new(data: [[T; LENGTH]; 1]) -> Self {
            Self (
                Matrix::from(data),
            )
        }

        pub fn length(&self) -> T {
            let mut len = T::zero();

            for &row in self.0.iter() {
                for &cell in row.iter() {
                    len += (cell) * (cell);
                }
            }

            len.sqrt()
        }
    }

    impl<T: MatrixDataTrait, const LENGTH: usize> VectorColumn<T, LENGTH> {
        pub fn new(data: [[T; 1]; LENGTH]) -> Self {
            Self (
                Matrix::from(data),
            )
        }

        pub fn length(&self) -> T {
            let mut len = T::zero();

            for &row in self.0.iter() {
                for &cell in row.iter() {
                    len += (cell) * (cell);
                }
            }

            len.sqrt()
        }
    }
}

#[cfg(test)]
mod vector_tests {
    use crate::matrix2::vector::{VectorColumn, VectorRow};

    #[test]
    fn length_test() {
        let vector_row = VectorRow::<i64, 4>::new([
            [1, 2, 3, 4]
        ]);

        let vector_col = VectorColumn::<i64, 4>::new([
            [1],
            [2],
            [3],
            [4],
        ]);

        // floor(sqrt(1^2 + 2^2 + 3^3 + 4^3)) = floor(5.477225575) = 5
        assert!(vector_row.length() == 5);
        assert!(vector_col.length() == 5);
    }

    #[test]
    fn into_test() {
        use super::matrix2::Matrix;

        let vector_row = VectorRow::<i64, 4>::new([
            [1, 2, 3, 4]
        ]);
        let matrix_row: Matrix<i64, 1, 4> = vector_row.into();
        assert!(matrix_row == Matrix::from([
            [1, 2, 3, 4]
        ]));

        let vector_col = VectorColumn::<i64, 4>::new([
            [1],
            [2],
            [3],
            [4],
        ]);
        let matrix_col: Matrix<i64, 4, 1> = vector_col.into();
        assert!(matrix_col == Matrix::from([
            [1],
            [2],
            [3],
            [4],
        ]));
    }
}
