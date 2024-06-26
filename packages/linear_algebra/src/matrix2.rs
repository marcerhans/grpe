pub mod matrix2 {
    use std::{
        fmt::Debug,
        ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
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

    #[derive(Clone)]
    pub struct Matrix<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> {
        data: [[T; COLS]; ROWS],
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>
        Matrix<T, ROWS, COLS>
    {
        pub fn new(data: [[T; COLS]; ROWS]) -> Self {
            Self { data }
        }

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

        pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, [T; COLS]> {
            self.into_iter()
        }

        pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, [T; COLS]> {
            self.into_iter()
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS> {
        fn default() -> Self {
            Self { data: [[T::zero(); COLS]; ROWS] }
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS> {
        type Output = [T; COLS];
    
        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<T, ROWS, COLS> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> IntoIterator for Matrix<T, ROWS, COLS> {
        type Item = [T; COLS];
        type IntoIter = core::array::IntoIter<Self::Item, ROWS>;
    
        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }

    impl<'a, T: MatrixDataTrait, const ROWS: usize, const COLS: usize> IntoIterator for &'a Matrix<T, ROWS, COLS> {
        type Item = &'a [T; COLS];
        type IntoIter = std::slice::Iter<'a, [T; COLS]>;
    
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }

    impl<'a, T: MatrixDataTrait, const ROWS: usize, const COLS: usize> IntoIterator for &'a mut Matrix<T, ROWS, COLS> {
        type Item = &'a mut [T; COLS];
        type IntoIter = std::slice::IterMut<'a, [T; COLS]>;
    
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter_mut()
        }
    }
}

#[cfg(test)]
mod matrix_owned_tests {
    use super::matrix2::Matrix;

    mod unit_tests {
        use super::*;

        #[test]
        fn new_test() {
            let matrix = Matrix::new([[1, 2, 3]]);

            // assert!(matrix[0] == 1)
        }

        #[test]
        fn zeros_test() {
            let matrix = Matrix::<f64, 1, 1>::zeros();
        }

        #[test]
        fn identity_test() {
            let matrix = Matrix::<f64, 1, 1>::identity();
        }

        #[test]
        fn default_test() {
            let matrix = Matrix::<f64, 1, 1>::default();
        }
    }
}
