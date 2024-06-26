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
    }

    impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS> {
        fn default() -> Self {
            Self { data: [[T::zero(); COLS]; ROWS] }
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
