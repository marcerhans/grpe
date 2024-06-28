use std::{
    fmt::Debug,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
    },
};

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
    pub(crate) data: [[T; COLS]; ROWS],
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

impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Debug for Matrix<T, ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix {{{:?}}}", self.data)
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

impl<T: MatrixDataTrait, const LHS_ROWS: usize, const LHS_COLS_RHS_ROWS: usize, const RHS_COLS: usize> Mul<Matrix<T, LHS_COLS_RHS_ROWS, RHS_COLS>> for Matrix<T, LHS_ROWS, LHS_COLS_RHS_ROWS> {
    type Output = Matrix<T, LHS_ROWS, RHS_COLS>;

    fn mul(self, rhs: Matrix<T, LHS_COLS_RHS_ROWS, RHS_COLS>) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        fn debug_test() {
            // Manual (visual) test.
            let matrix = Matrix::from([
                [1,2,3],
                [4,5,6],
                [7,8,9],
            ]);
            println!("{:?}", matrix);
        }
    }
}
