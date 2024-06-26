pub mod matrix2 {
    use std::{
        fmt::Debug,
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
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
        + Clone
        + Copy
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

    #[derive(Default, Clone)]
    pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
        data: T,
    }

    /// This implementation owns the data.
    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Matrix<[[T; COLS]; ROWS], ROWS, COLS> {
        pub fn from_array(data: [[T; COLS]; ROWS]) -> Self {
            Self {
                data,
            }
        }
    }

    /// This implementation does NOT own the data.
    impl<'a, T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Matrix<&'a [&'a [T; COLS]; ROWS], ROWS, COLS> {
        pub fn from_slice(data: &'a [&'a [T; COLS]; ROWS]) -> Self {
            Self {
                data,
            }
        }
    }

    /// This implemntation provides shared behaviour for all implemntations of [Matrix].
    impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {

    }
}

#[cfg(test)]
mod matrix_ref_tests {
    use super::matrix2::Matrix;

    #[test]
    fn from_slice_test() {
        let matrix = Matrix::from_slice(&[
            &[1, 2, 3],
        ]);
    }
}
