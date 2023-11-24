use tensor::*;

/// Today I learned that tensors are the more general form of a matrix. I.e. tensors are matrices of higher dimension.
mod tensor {
    use super::*;

    use matrix::*;
    use std::{
        cell::{Ref, RefCell, RefMut},
        fmt,
        ops::{Add, Mul, Sub},
    };

    pub trait TensorTraits: Clone + Default {}
    impl<T: Clone + Default> TensorTraits for T {}

    pub trait Transpose {
        type Output;

        fn transpose(self) -> Self::Output;
    }

    pub trait Inverse {
        fn inverse(self);
    }

    pub struct Tensor<T> {
        /// Elements in this vector are laid out sequentually, i.e. the higher the index, the higher the dimension. This makes it easier to iterate over.
        tensor: RefCell<Vec<T>>,

        /// Length of each dimension.
        dimensions: Vec<usize>,

        /// Memoized max indexing values.
        dimensions_index_max: Vec<usize>,
    }

    impl<T: TensorTraits> Tensor<T> {
        /// TODO: Better docs.
        /// Example: Vec = {max elements in x, max elements in y, max elements in z}.
        pub fn new(dimensions: &[usize]) -> Self {
            let total_size = dimensions.iter().product();

            if total_size < 1 {
                panic!("Illegal to have a tensor where a dimension is less than 1!");
            }

            // Truncate any unnecessary "1-sized" dimensions.
            let mut truncation_point = dimensions.len();

            for i in (0..dimensions.len()).rev() {
                if dimensions[i] == 1 && i != 0 {
                    truncation_point = i;
                } else {
                    break;
                }
            }

            let dimensions = &dimensions[0..truncation_point];
            let dimensions_index_max = dimensions.iter().map(|&x| x - 1).collect::<Vec<usize>>();

            Self {
                tensor: RefCell::new(vec![T::default(); total_size]),
                dimensions: dimensions.to_vec(),
                dimensions_index_max,
            }
        }

        /// Get value at position [index].
        pub fn get(&self, coordinates: &[usize]) -> Result<T, &'static str> {
            let index = self.coordinates_to_index(coordinates)?;
            Ok(self.tensor.borrow()[index].clone())
        }

        /// Set [value] at position [index].
        pub fn set(&self, coordinates: &[usize], value: T) -> Result<(), &'static str> {
            let index = self.coordinates_to_index(coordinates)?;
            self.tensor.borrow_mut()[index] = value;
            Ok(())
        }

        /// Returns the size of the [Tensor] in terms of available spaces.
        pub fn size(&self) -> usize {
            self.tensor.borrow().len()
        }

        /// Returns dimensions of the [Tensor].
        pub fn dimensions<'a>(&'a self) -> &'a [usize] {
            self.dimensions.as_slice()
        }

        /// Flatten coordinates to a one-dimensional index value.
        /// Error when coordinates are outside valid range.
        fn coordinates_to_index(&self, coordinates: &[usize]) -> Result<usize, &'static str> {
            let mut index = 0;
            let mut dimensional_step_length = 1;

            if coordinates.len() != self.dimensions.len() {
                return Err("Given coordinates cannot be mapped to a valid location in the tensor (out of bounds). I.e. coordinates has too few or too many dimensions.");
            }

            for i in 0..self.dimensions.len() {
                if coordinates[i] > self.dimensions_index_max[i] {
                    return Err("Given coodinate(s) are out of maximum bounds for current tensor.");
                }

                index += coordinates[i] * dimensional_step_length;
                dimensional_step_length *= self.dimensions[i];
            }

            Ok(index)
        }
    }

    impl<T: TensorTraits> Clone for Tensor<T> {
        fn clone(&self) -> Self {
            Self {
                tensor: self.tensor.clone(),
                dimensions: self.dimensions.clone(),
                dimensions_index_max: self.dimensions_index_max.clone(),
            }
        }
    }

    impl<T: TensorTraits + PartialEq + Eq> PartialEq for Tensor<T> {
        fn eq(&self, other: &Self) -> bool {
            self.tensor == other.tensor
                && self.dimensions == other.dimensions
                && self.dimensions_index_max == other.dimensions_index_max
        }
    }

    impl<T: TensorTraits + PartialEq + Eq> Eq for Tensor<T> {}

    impl<T: TensorTraits + Add<Output = T>> Add for Tensor<T> {
        type Output = Result<Self, &'static str>;

        fn add(self, rhs: Self) -> Self::Output {
            if self.dimensions.len() != rhs.dimensions.len() {
                return Err("Tensors are not defined in the same dimensions.");
            }

            for (&lhs, &rhs) in self.dimensions.iter().zip(&rhs.dimensions) {
                if lhs != rhs {
                    return Err("Tensors dimensions are not of the same size.");
                }
            }

            let mut lhs = self.clone();

            for (mut lhs, rhs) in (&mut lhs).into_iter().zip(&rhs) {
                *lhs = (*lhs).clone() + (*rhs).clone();
            }

            Ok(lhs)
        }
    }

    impl<T: TensorTraits + Sub<Output = T>> Sub for Tensor<T> {
        type Output = Result<Self, &'static str>;

        fn sub(self, rhs: Self) -> Self::Output {
            if self.dimensions.len() != rhs.dimensions.len() {
                return Err("Tensors are not defined in the same dimensions.");
            }

            for (&lhs, &rhs) in self.dimensions.iter().zip(&rhs.dimensions) {
                if lhs != rhs {
                    return Err("Tensors dimensions are not of the same size.");
                }
            }

            let mut lhs = self.clone();

            for (mut lhs, rhs) in (&mut lhs).into_iter().zip(&rhs) {
                *lhs = (*lhs).clone() - (*rhs).clone();
            }

            Ok(lhs)
        }
    }

    impl<T: fmt::Debug> fmt::Debug for Tensor<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Tensor")
                .field("tensor", &self.tensor)
                .field("dimensions", &self.dimensions)
                .field("dimensions_index_max", &self.dimensions_index_max)
                .finish()
        }
    }

    pub struct TensorIter<'a, T> {
        item: Option<Ref<'a, [T]>>,
    }

    impl<'a, T> Iterator for TensorIter<'a, T> {
        type Item = Ref<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.item.take() {
                Some(borrow) => match *borrow {
                    [_, ..] => {
                        let (head, tail) = Ref::map_split(borrow, |slice| (&slice[0], &slice[1..]));
                        // Replace the item with the "rest" (tail) of the slice.
                        self.item.replace(tail);
                        Some(head)
                    }
                    [] => None,
                },
                None => None,
            }
        }
    }

    impl<'a, T> DoubleEndedIterator for TensorIter<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self.item.take() {
                Some(borrow) => match *borrow {
                    [_, ..] => {
                        let (head, tail) = Ref::map_split(borrow, |slice| {
                            (&slice[slice.len() - 1], &slice[..slice.len() - 1])
                        });
                        // Replace the item with the "rest" (tail) of the slice.
                        self.item.replace(tail);
                        Some(head)
                    }
                    [] => None,
                },
                None => None,
            }
        }
    }

    impl<'a, T> IntoIterator for &'a Tensor<T> {
        type Item = Ref<'a, T>;
        type IntoIter = TensorIter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            let borrow = self.tensor.borrow();

            Self::IntoIter {
                item: Some(Ref::map(borrow, |t| t.as_slice())),
            }
        }
    }

    pub struct TensorIterMut<'a, T> {
        item: Option<RefMut<'a, [T]>>,
    }

    impl<'a, T> Iterator for TensorIterMut<'a, T> {
        type Item = RefMut<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.item.take() {
                Some(borrow) => match *borrow {
                    [_, ..] => {
                        let (head, tail) = RefMut::map_split(borrow, |slice| slice.split_at_mut(1));
                        let head_what = RefMut::map(head, |reff| &mut reff.as_mut()[0]);
                        // Replace the item with the "rest" (tail) of the slice.
                        self.item.replace(tail);
                        Some(head_what)
                    }
                    [] => None,
                },
                None => None,
            }
        }
    }

    impl<'a, T> IntoIterator for &'a mut Tensor<T> {
        type Item = RefMut<'a, T>;
        type IntoIter = TensorIterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            let borrow = self.tensor.borrow_mut();

            Self::IntoIter {
                item: Some(RefMut::map(borrow, |t| t.as_mut_slice())),
            }
        }
    }

    /// These are matrix only operations/functions separate from the generic [Tensor].
    mod matrix {
        use super::*;

        type Matrix = Tensor<usize>;

        impl Matrix {
            pub fn zeros(self) -> Self {
                {
                    let mut tensor = self.tensor.borrow_mut();

                    for element in tensor.iter_mut() {
                        *element = 0;
                    }
                }

                self
            }

            pub fn identity(self) -> Self {
                {
                    let rows = self.dimensions[0];
                    let columns = self.dimensions[1];
                    let mut tensor = self.tensor.borrow_mut();
                    let mut index = 0;

                    for i in 0..self.size() {
                        tensor[i] = 0;

                        if index == i {
                            tensor[i] = 1;

                            index += rows + columns + 1;
                        }
                    }
                }

                self
            }
        }

        /// # Note:
        /// This currently is just matrix multiplication, and not tensor "whatever the operation should be called".
        /// General tensor multiplication was hard, and since I don't need it currently,
        /// I just skipped it, and let it be at most "2 by 2" only.
        impl<T: TensorTraits + Mul<Output = T> + Add<Output = T>> Mul for Tensor<T> {
            type Output = Result<Tensor<T>, &'static str>;

            fn mul(self, rhs: Self) -> Self::Output {
                // if self.dimensions.len() < 1
                //     || self.dimensions.len() > 2
                //     || rhs.dimensions.len() < 1
                //     || rhs.dimensions.len() > 2
                // {
                //     return Err("Tensor dimensions do not allow for matrix multiplication!");
                // }

                // let product = Tensor::<T>::new(&[columns, rows]);

                // for row in 0..rows {
                //     for column in 0..columns {
                //         let mut column_by_row_product_sum = T::default();

                //         for i in 0..self.dimensions[1] {
                //             column_by_row_product_sum = column_by_row_product_sum
                //                 + self.get(&[row, i]).unwrap() * rhs.get(&[i, column]).unwrap();
                //         }

                //         product
                //             .set(&[column, row], column_by_row_product_sum)
                //             .unwrap();
                //     }
                // }

                // Ok(product)
                todo!()
            }
        }

        // TODO
        // impl<T: TensorTraits> Transpose for Tensor<T> {
        //     type Output = Result<Self, &'static str>;

        //     fn transpose(self) -> Self::Output {
        //         let transposed = Tensor::new(&[self.dimensions[1], self]);
        //         let rows = self.dimensions[0];
        //         let columns = self.dimensions[1];

        //         for row in 0..rows {
        //             for column in 0..columns {
        //                 transposed.set(&[column, row])
        //             }
        //         }

        //         todo!()
        //     }
        // }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_mul() {
                // let lhs = Tensor::<usize>::new(&[]);
                // let rhs = Tensor::<usize>::new(&[]);
                // assert!((lhs * rhs).is_err(), "Empty matrices are not allowed.");

                // let lhs = Tensor::<usize>::new(&[1, 1, 1]);
                // let rhs = Tensor::<usize>::new(&[1, 1, 1]);
                // assert!(
                //     (lhs * rhs).is_err(),
                //     "Matrices of higher dimension that 2 are not allowed."
                // );

                // let lhs = Tensor::<usize>::new(&[1]);
                // let rhs = Tensor::<usize>::new(&[1]);
                // lhs.set(&[0, 0], 1);
                // rhs.set(&[0, 0], 2);
                // assert!(
                //     (lhs * rhs).is_ok_and(|matrix| {
                //         assert!(matrix.size() == 1);
                //         assert!(matrix.get(&[0, 0]).unwrap() == 2);
                //         true
                //     }),
                //     "Singleton matrixes should work."
                // );

                // let lhs = Tensor::<usize>::new(&[1, 2]);
                // let rhs = Tensor::<usize>::new(&[2]);
                // lhs.set(&[0, 0], 1);
                // lhs.set(&[0, 1], 2);
                // rhs.set(&[0], 2);
                // rhs.set(&[1], 3);
                // assert!(
                //     (lhs * rhs).is_ok_and(|matrix| {
                //         assert!(matrix.size() == 2);
                //         assert!(matrix.get(&[0, 0]).unwrap() == 2);
                //         assert!(matrix.get(&[0, 1]).unwrap() == 4);
                //         true
                //     }),
                //     "Varying matrix sizes."
                // );

                // let lhs = Tensor::<usize>::new(&[2]);
                // let rhs = Tensor::<usize>::new(&[2, 1]);
                // lhs.set(&[0], 1);
                // lhs.set(&[1], 2);
                // rhs.set(&[0, 1], 2);
                // rhs.set(&[0, 0], 3);
                // assert!(
                //     (lhs * rhs).is_ok_and(|matrix| {
                //         assert!(matrix.size() == 2);
                //         assert!(matrix.get(&[0, 0]).unwrap() == 2);
                //         assert!(matrix.get(&[0, 1]).unwrap() == 4);
                //         true
                //     }),
                //     "Varying matrix sizes."
                // );

                // let lhs = Tensor::<usize>::new(&[3, 7]);
                // let rhs = Tensor::<usize>::new(&[3, 6]);
                // assert!(
                //     (lhs * rhs).is_err(),
                //     "Matrix row-length of lhs must match the column-length of rhs."
                // );

                // let lhs = Tensor::<usize>::new(&[3, 6]);
                // let rhs = Tensor::<usize>::new(&[3, 7]);
                // assert!(
                //     (lhs * rhs).is_err(),
                //     "Matrix row-length of lhs must match the column-length of rhs."
                // );

                // let lhs = Tensor::<usize>::new(&[3, 7]);
                // let rhs = Tensor::<usize>::new(&[3, 7]);
                // assert!((lhs * rhs).is_err());

                // let lhs = Tensor::<usize>::new(&[3]);
                // let mut rhs = Tensor::<usize>::new(&[3, 2]);
                // let mut id = 1;

                // for mut element_rhs in &mut rhs {
                //     *element_rhs = id;
                //     id += 1;
                // }

                // lhs.set(&[0], 1).unwrap();
                // lhs.set(&[1], 2).unwrap();
                // lhs.set(&[2], 3).unwrap();

                // let new = lhs * rhs;
                // assert!(new.is_ok_and(|matrix| {
                //     assert!(matrix.size() == 6);
                //     // assert!(matrix.get(&[0, 0]).unwrap() == 22);
                //     // assert!(matrix.get(&[0, 1]).unwrap() == 28);
                //     // assert!(matrix.get(&[1, 0]).unwrap() == 49);
                //     // assert!(matrix.get(&[1, 1]).unwrap() == 64);
                //     true
                // }));

                // let mut lhs = Tensor::<usize>::new(&[2, 3]);
                // let mut rhs = Tensor::<usize>::new(&[3, 2]);
                // let mut id = 1;

                // for (mut element_lhs, mut element_rhs) in (&mut lhs).into_iter().zip(&mut rhs) {
                //     *element_lhs = id;
                //     *element_rhs = id;
                //     id += 1;
                // }

                // let new = lhs * rhs;
                // assert!(new.is_ok_and(|matrix| {
                //     assert!(matrix.size() == 4);
                //     assert!(matrix.get(&[0, 0]).unwrap() == 22);
                //     assert!(matrix.get(&[0, 0]).unwrap() == 22);
                //     assert!(matrix.get(&[0, 1]).unwrap() == 28);
                //     assert!(matrix.get(&[1, 0]).unwrap() == 49);
                //     assert!(matrix.get(&[1, 1]).unwrap() == 64);
                //     true
                // }));
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let tensor = Tensor::<isize>::new(&[1, 1, 1]);
            assert!(tensor.dimensions.len() == 1);

            let tensor = Tensor::<isize>::new(&[1, 2, 3]);
            assert!(tensor.dimensions.len() == 3);

            let tensor = Tensor::<isize>::new(&[1, 2, 3, 1, 1]);
            assert!(tensor.dimensions.len() == 3);
        }

        #[test]
        fn test_coordinates_to_index() {
            let tensor = Tensor::<isize>::new(&[1, 2, 3]);
            let index = tensor.coordinates_to_index(&[0, 1, 2]);
            assert!(index.is_ok_and(|x| x == 5));

            let tensor = Tensor::<isize>::new(&[1, 2, 3]);
            let index = tensor.coordinates_to_index(&[0, 1, 1]);
            assert!(index.is_ok_and(|x| x == 3));
        }

        #[test]
        fn test_coordinates_to_index_bounds() {
            let tensor = Tensor::<isize>::new(&[3, 7, 6]);
            assert!(tensor.coordinates_to_index(&[3, 7, 6]).is_err());
            assert!(tensor.coordinates_to_index(&[3, 6, 5]).is_err());
            assert!(tensor.coordinates_to_index(&[2, 7, 5]).is_err());
            assert!(tensor.coordinates_to_index(&[2, 6, 6]).is_err());
            assert!(tensor.coordinates_to_index(&[2, 6, 5]).is_ok());
        }

        #[test]
        fn test_set_and_get() {
            let tensor = Tensor::new(&[3, 7, 6]);

            let mut id = 0;
            for i in 0..6 {
                for j in 0..7 {
                    for k in 0..3 {
                        assert!(tensor.set(&[k, j, i], id).is_ok());
                        id += 1;
                    }
                }
            }

            let mut id = 0;
            for i in 0..6 {
                for j in 0..7 {
                    for k in 0..3 {
                        assert!(tensor.get(&[k, j, i]).is_ok_and(|x| x == id));
                        id += 1;
                    }
                }
            }
        }

        #[test]
        fn test_iter() {
            let mut tensor = Tensor::new(&[3, 7, 6]);

            // Set and check with regular iteration first.
            let mut id = 1;
            for i in 0..6 {
                for j in 0..7 {
                    for k in 0..3 {
                        assert!(tensor.set(&[k, j, i], id).is_ok());
                        id += 1;
                    }
                }
            }

            let mut id = 1;
            for i in 0..6 {
                for j in 0..7 {
                    for k in 0..3 {
                        assert!(tensor.get(&[k, j, i]).is_ok_and(|x| x == id));
                        id += 1;
                    }
                }
            }

            // Check with iterators
            for element in &tensor {
                assert!(*element != 0);
            }

            // Mutate with iterators
            for mut element in &mut tensor {
                *element = 1;
            }

            for element in &tensor {
                assert!(*element == 1);
            }
        }

        #[test]
        fn test_iter_reverse() {
            let mut tensor = Tensor::new(&[3, 7, 6]);
            let mut id = 0;

            for mut element in &mut tensor {
                *element = id;
                id += 1;
            }

            for element in (&tensor).into_iter().rev() {
                id -= 1;
                assert!(*element == id);
            }
        }

        #[test]
        fn test_equal() {
            let mut tensor = Tensor::new(&[3, 7, 6]);

            let mut id = 0;

            for mut element in &mut tensor {
                *element = id;
                id += 1;
            }

            assert!(tensor == tensor.clone());
        }

        #[test]
        fn test_add() {
            let mut lhs = Tensor::<isize>::new(&[3, 7, 6]);
            let mut id = 0;

            for mut x in &mut lhs {
                *x = id;
                id += 1;
            }

            let rhs = lhs.clone();

            let sum = (lhs + rhs).expect("Failed to run addition on tensors.");
            let mut id = 0;

            for x in &sum {
                assert!(*x == id * 2);
                id += 1;
            }
        }

        #[test]
        fn test_sub() {
            let mut lhs = Tensor::<usize>::new(&[3, 7, 6]);
            let mut id = 0;

            for mut x in &mut lhs {
                *x += id;
                id += 1;
            }

            let rhs = lhs.clone();
            let sum = (lhs - rhs).expect("Failed to run subtraction on tensors.");

            for x in sum.into_iter().rev() {
                assert!(*x == 0);
            }
        }
    }
}
