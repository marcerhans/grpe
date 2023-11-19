use tensor::*;
// use point::*;
// use vector::*;

/// Today I learned that tensors are the more general form of a matrix. I.e. tensors are matrices of higher dimension.
mod tensor {
    use super::*;
    use std::{
        cell::{Ref, RefCell, RefMut},
        fmt,
        ops::{Add, Mul, Sub},
    };

    trait TensorTraits: Clone + Default {}
    impl<T: Clone + Default> TensorTraits for T {}

    struct Tensor<T> {
        /// Elements in this vector are laid out sequentually, i.e. the higher the index, the higher the dimension. This makes it easier to iterate over.
        tensor: RefCell<Vec<T>>,
        dimensions: Vec<usize>,

        /// Simply an optimization.
        dimensions_index_max: Vec<usize>,
    }

    impl<T: TensorTraits> Tensor<T> {
        /// TODO: Better docs.
        /// Example: Vec = {max elements in x, max elements in y, max elements in z}.
        pub fn new(dimensions: &[usize]) -> Self {
            let as_index = dimensions.iter().map(|x| x - 1).collect();

            Self {
                tensor: RefCell::new(vec![T::default(); dimensions.iter().product()]),
                dimensions: dimensions.to_vec(),
                dimensions_index_max: as_index,
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

        /// Returns the size of the tensor as a in terms of available spaces.
        pub fn size(&self) -> usize {
            self.tensor.borrow().len()
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

    impl Mul for Tensor<usize> {
        type Output = Result<Tensor<usize>, &'static str>;

        fn mul(mut self, rhs: Self) -> Self::Output {
            if self.dimensions != rhs.dimensions {
                return Err("Right hand side of multiplication does not have the same amount of dimensions.");
            }

            for (mut left, right) in (&mut self).into_iter().zip(rhs.into_iter()) {
                *left += *right;
            }

            Ok(self)
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

    struct TensorIter<'a, T> {
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

    struct TensorIterMut<'a, T> {
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

    #[cfg(test)]
    mod foo {
        #[test]
        fn main() {}
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
        fn test_tensor_iter() {
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
            println!("{tensor:?}");

            // Mutate with iterators
            for mut element in &mut tensor {
                *element = 1;
            }

            for element in &tensor {
                assert!(*element == 1);
            }
            println!("{tensor:?}");
        }

        #[test]
        fn test_add() {
            // let mut lhs = Tensor::new(&[3, 7, 6]);
            // let mut rhs = Tensor::new(&[3, 7, 6]);
        }

        #[test]
        fn test_sub() {
            // let mut lhs = Tensor::new(&[3, 7, 6]);
            // let mut rhs = Tensor::new(&[3, 7, 6]);
        }
    }
}

// mod vector {
//     use super::*;

//     struct Vector<T> {
//         inner: Vec<T>,
//     }

//     impl Vector<usize> {
//         fn new(n_dimensions: usize) -> Self {
//             Self { inner: Vec::new() }
//         }
//     }
// }

// mod point {
//     use super::*;

//     /// Same structure as Vector.
//     pub struct Point<T> {
//         inner: Vec<T>,
//     }

//     impl Point<usize> {
//         fn new(length: usize) -> Self {
//             Self {
//                 dim: length,
//                 position: vec![0; length],
//                 matrix_translation: vec![0; (length + 1) * (length + 1)],
//             }
//         }

//         /// Translates a point.
//         ///
//         /// 2D Example:
//         ///       [translation argument]
//         ///                |
//         ///                v
//         /// | 1 0 t1 |   | p1 |   | p1 + t1 |
//         /// | 0 1 t2 | * | p2 | = | p2 + t2 |
//         /// | 0 0 1  |   | 1  |   | 1       |
//         ///
//         /// Though... we cheat and translate using simple additionf or each component.
//         fn translate(&mut self, translation: &[usize]) {
//             if translation.len() > self.dim {
//                 panic!("Cannot translate using a matrix of higher dimension than the point itself.")
//             }

//             for (pos, trans) in self.position.iter_mut().zip(translation) {
//                 *pos += trans;
//             }
//         }

//         fn rotate(axis: usize, degrees: f64) {}
//     }
// }

// // mod matrix {
// //     use super::*;

// //     /// Matrix holding vectors.
// //     pub struct Matrix {
// //         vectors: Vec<NVector>,
// //     }

// //     impl Matrix {
// //         fn new(dimensions: usize) -> Self {
// //             Self {
// //                 vectors: Vec::new(),
// //             }
// //         }
// //     }
// // }

// pub fn rotate() {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
