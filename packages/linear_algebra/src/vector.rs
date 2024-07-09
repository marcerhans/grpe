use std::{fmt::Debug, ops::{Index, IndexMut}};

use crate::matrix::{Matrix, MatrixDataTrait};

/// [VectorRow] and [VectorColumn] are sub-types to [Matrix].
/// TODO: Some duplicate code for row and column vector. Not the best,
/// but it will make the type system happy.

#[derive(Clone, PartialEq, PartialOrd)]
pub struct VectorRow<T: MatrixDataTrait, const LENGTH: usize>(pub Matrix<T, 1, LENGTH>);
#[derive(Clone, PartialEq, PartialOrd)]
pub struct VectorColumn<T: MatrixDataTrait, const LENGTH: usize>(pub Matrix<T, LENGTH, 1>);

impl<T: MatrixDataTrait, const LENGTH: usize> VectorRow<T, LENGTH> {
    pub fn length(&self) -> T {
        let mut len = T::zero();

        for &row in self.0.iter() {
            for &cell in row.iter() {
                len += (cell) * (cell);
            }
        }

        len.sqrt()
    }

    /// Calculate the dot-product of two compatible [VectorRow]s.
    pub fn dot(&self, rhs: &Self) -> T {
        let mut sum = T::zero();

        for (row_a, row_b) in self.0.iter().zip(rhs.0.iter()) {
            for (&cell_a, &cell_b) in row_a.iter().zip(row_b.iter()) {
                sum += cell_a * cell_b;
            }
        }

        sum
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> VectorColumn<T, LENGTH> {
    pub fn length(&self) -> T {
        let mut len = T::zero();

        for &row in self.0.iter() {
            for &cell in row.iter() {
                len += (cell) * (cell);
            }
        }

        len.sqrt()
    }

    /// Calculate the dot-product of two compatible [VectorColumn]s.
    pub fn dot(&self, rhs: &Self) -> T {
        let mut sum = T::zero();

        for (row_a, row_b) in self.0.iter().zip(rhs.0.iter()) {
            for (&cell_a, &cell_b) in row_a.iter().zip(row_b.iter()) {
                sum += cell_a * cell_b;
            }
        }

        sum
    }
}

impl<T: MatrixDataTrait> VectorRow<T, 3> {
    /// Calculate the cross-product of two [VectorRow]s of [LENGTH] 3.
    pub fn cross(&self, rhs: &Self) -> Self {
        VectorRow::from([
            self.0[0][1] * rhs.0[0][2] - self.0[0][2] * rhs.0[0][1],
            self.0[0][2] * rhs.0[0][0] - self.0[0][0] * rhs.0[0][2],
            self.0[0][0] * rhs.0[0][1] - self.0[0][1] * rhs.0[0][0],
        ])
    }
}

impl<T: MatrixDataTrait> VectorColumn<T, 3> {
    /// Calculate the cross-product of two [VectorColumn]s of [LENGTH] 3.
    pub fn cross(&self, rhs: &Self) -> Self {
        VectorColumn::from([
            [self.0[1][0] * rhs.0[2][0] - self.0[2][0] * rhs.0[1][0]],
            [self.0[2][0] * rhs.0[0][0] - self.0[0][0] * rhs.0[2][0]],
            [self.0[0][0] * rhs.0[1][0] - self.0[1][0] * rhs.0[0][0]],
        ])
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> Debug for VectorRow<T, LENGTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VectorRow {{{:?}}}", self.0.data[0])
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> Debug for VectorColumn<T, LENGTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VectorColumn {{{:?}}}", self.0.data)
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> From<[T; LENGTH]> for VectorRow<T, LENGTH> {
    fn from(data: [T; LENGTH]) -> Self {
        Self(
            Matrix::from([data]),
        )
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> From<[[T; 1]; LENGTH]> for VectorColumn<T, LENGTH> {
    fn from(data: [[T; 1]; LENGTH]) -> Self {
        Self(
            Matrix::from(data),
        )
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> From<&[T; LENGTH]> for VectorRow<T, LENGTH> {
    fn from(data: &[T; LENGTH]) -> Self {
        Self(
            Matrix::from([*data]),
        )
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> From<&[[T; 1]; LENGTH]> for VectorColumn<T, LENGTH> {
    fn from(data: &[[T; 1]; LENGTH]) -> Self {
        Self(
            Matrix::from(data),
        )
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> From<Matrix<T, 1, LENGTH>> for VectorRow<T, LENGTH> {
    fn from(matrix: Matrix<T, 1, LENGTH>) -> Self {
        Self(
            matrix
        )
    }
}

// TODO: Um, why did I have to implement this? Is this not supposed to be automatically implemented?
impl<T: MatrixDataTrait, const LENGTH: usize> From<Matrix<T, LENGTH, 1>> for VectorColumn<T, LENGTH> {
    fn from(matrix: Matrix<T, LENGTH, 1>) -> Self {
        Self(
            matrix
        )
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> Into<Matrix<T, 1, LENGTH>> for VectorRow<T, LENGTH> {
    fn into(self) -> Matrix<T, 1, LENGTH> {
        self.0
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> Into<Matrix<T, LENGTH, 1>> for VectorColumn<T, LENGTH> {
    fn into(self) -> Matrix<T, LENGTH, 1> {
        self.0
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> Index<usize>
    for VectorRow<T, LENGTH>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[0][index]
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> IndexMut<usize>
    for VectorRow<T, LENGTH>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[0][index]
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> Index<usize>
    for VectorColumn<T, LENGTH>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index][0]
    }
}

impl<T: MatrixDataTrait, const LENGTH: usize> IndexMut<usize>
    for VectorColumn<T, LENGTH>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_test() {
        let vector_row = VectorRow::<i64, 4>::from([1, 2, 3, 4]);
        let vector_col = VectorColumn::<i64, 4>::from([
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
        let vector_row = VectorRow::<i64, 4>::from([1, 2, 3, 4]);
        let matrix_row: Matrix<i64, 1, 4> = vector_row.into();
        assert!(matrix_row == Matrix::from([
            [1, 2, 3, 4]
        ]));

        let vector_col = VectorColumn::<i64, 4>::from([
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

    #[test]
    fn from_test() {
        let matrix_row: Matrix<i64, 1, 4> = Matrix::from([
            [1, 2, 3, 4]
        ]);
        let vector_row = VectorRow::<i64, 4>::from(matrix_row);
        assert!(vector_row.0 == Matrix::from([
            [1, 2, 3, 4]
        ]));

        let matrix_col: Matrix<i64, 4, 1> = Matrix::from([
            [1],
            [2],
            [3],
            [4],
        ]);
        let vector_col = VectorColumn::<i64, 4>::from(matrix_col);
        assert!(vector_col.0 == Matrix::from([
            [1],
            [2],
            [3],
            [4],
        ]));
    }

    #[test]
    fn debug_test() {
        // Manual (visual) test.
        let vector_row = VectorRow::<i64, 4>::from([1, 2, 3, 4]);
        let vector_col = VectorColumn::<i64, 4>::from([
            [1],
            [2],
            [3],
            [4],
        ]);
        println!("{:?}", vector_row);
        println!("{:?}", vector_col);
    }

    #[test]
    fn dot_product_test() {
        let vector_row = VectorRow::<i64, 4>::from([1, 2, 3, 4]);
        let dot_product_row = vector_row.dot(&VectorRow::from([1, 2, 3, 4]));
        assert!(dot_product_row == 1 + 4 + 9 + 16, "{}", dot_product_row);

        let vector_col = VectorColumn::<i64, 4>::from([[1], [2], [3], [4]]);
        let dot_product_col = vector_col.dot(&VectorColumn::from([[1], [2], [3], [4]]));
        assert!(dot_product_col == 1 + 4 + 9 + 16, "{}", dot_product_col);

        // Would fail to compile.
        // let vector_row = VectorRow::<i64, 4>::from([1, 2, 3, 4]);
        // let dot_product_col = vector_row.dot(&VectorColumn::from([[1], [2], [3], [4]]));
        // assert!(dot_product_col == 1 + 4 + 9 + 16, "{}", dot_product_col);
    }

    #[test]
    fn cross_product_test() {
        let vector_row = VectorRow::from([1, 2, 3]);
        let cross_product_row = vector_row.cross(&VectorRow::from([4, 5, 6]));
        assert!(cross_product_row == VectorRow::from([-3, 6, -3]));

        let vector_col = VectorColumn::from([[1], [2], [3]]);
        let cross_product_col = vector_col.cross(&VectorColumn::from([[4], [5], [6]]));
        assert!(cross_product_col == VectorColumn::from([[-3], [6], [-3]]));

        // Would fail to compile due to dimensions beings wrong.
        // let vector_row = VectorRow::from([1, 2, 3, 10]);
        // let cross_product_row = vector_row.cross(&VectorRow::from([4, 5, 6, 10]));
    }
}