use matrix::*;
// use point::*;
// use vector::*;

mod matrix {
    use super::*;
    use std::{
        cell::{Ref, RefCell},
        ops::Add,
    };

    struct Matrix<T> {
        matrix: RefCell<Vec<T>>,
        dimensions: Vec<usize>,

        /// Simply an optimization.
        dimensions_index_max: Vec<usize>,
    }

    impl Matrix<usize> {
        /// TODO: Better docs.
        /// Example: Vec = {max elements in x, max elements in y, max elements in z}.
        pub fn new(dimensions: Vec<usize>) -> Self {
            let as_index = dimensions.iter().map(|x| x - 1).collect();

            Self {
                matrix: RefCell::new(vec![0_usize; dimensions.iter().product()]),
                dimensions,
                dimensions_index_max: as_index,
            }
        }

        /// Get value at position [index].
        pub fn get(&self, coordinates: &[usize]) -> Result<usize, &'static str> {
            let index = self.coordinates_to_index(coordinates)?;
            Ok(self.matrix.borrow()[index])
        }

        /// Set [value] at position [index].
        pub fn set(&self, coordinates: &[usize], value: usize) -> Result<(), &'static str> {
            let index = self.coordinates_to_index(coordinates)?;
            self.matrix.borrow_mut()[index] = value;
            Ok(())
        }

        /// Returns the size of the matrix as a in terms of available spaces.
        pub fn size(&self) -> usize {
            self.matrix.borrow().len()
        }

        /// Flatten coordinates to a one-dimensional index value.
        /// Error when coordinates are outside valid range.
        fn coordinates_to_index(&self, coordinates: &[usize]) -> Result<usize, &'static str> {
            let mut index = 0;
            let mut dimensional_step_length = 1;

            if coordinates.len() != self.dimensions.len() {
                return Err("Given coordinates cannot be mapped to a valid location in the matrix (out of bounds). I.e. coordinates has too few or too many dimensions.");
            }

            for i in 0..self.dimensions.len() {
                if coordinates[i] > self.dimensions_index_max[i] {
                    return Err("Given coodinate(s) are out of maximum bounds for current matrix.");
                }

                index += coordinates[i] * dimensional_step_length;
                dimensional_step_length *= self.dimensions[i];
            }

            Ok(index)
        }
    }

    impl<'a, T> IntoIterator for &'a Matrix<T> {
        type Item = Ref<'a, T>;

        type IntoIter = MatrixIter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            let borrow = self.matrix.borrow();

            Self::IntoIter {
                item: Some(Ref::map(borrow, |t| t.as_slice())),
            }
        }
    }

    struct MatrixIter<'a, T> {
        item: Option<Ref<'a, [T]>>,
    }

    impl<'a, T> Iterator for MatrixIter<'a, T> {
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
            let matrix = Matrix::new(vec![1, 2, 3]);
            let index = matrix.coordinates_to_index(&[0, 1, 2]);
            assert!(index.is_ok_and(|x| x == 5));

            let matrix = Matrix::new(vec![1, 2, 3]);
            let index = matrix.coordinates_to_index(&[0, 1, 1]);
            assert!(index.is_ok_and(|x| x == 3));
        }

        #[test]
        fn test_coordinates_to_index_bounds() {
            let matrix = Matrix::new(vec![3, 7, 6]);
            assert!(matrix.coordinates_to_index(&[3, 7, 6]).is_err());
            assert!(matrix.coordinates_to_index(&[3, 6, 5]).is_err());
            assert!(matrix.coordinates_to_index(&[2, 7, 5]).is_err());
            assert!(matrix.coordinates_to_index(&[2, 6, 6]).is_err());
            assert!(matrix.coordinates_to_index(&[2, 6, 5]).is_ok());
        }

        #[test]
        fn test_set_and_get() {
            let matrix = Matrix::new(vec![3, 7, 6]);

            let mut id = 0;
            for i in 0..6 {
                for j in 0..7 {
                    for k in 0..3 {
                        assert!(matrix.set(&[k, j, i], id).is_ok());
                        id += 1;
                    }
                }
            }

            let mut id = 0;
            for i in 0..6 {
                for j in 0..7 {
                    for k in 0..3 {
                        assert!(matrix.get(&[k, j, i]).is_ok_and(|x| x == id));
                        id += 1;
                    }
                }
            }
        }

        #[test]
        fn test_matrix_iter() {
            let matrix = Matrix::new(vec![3, 7, 6]);

            let mut id = 1;

            for i in 0..6 {
                for j in 0..7 {
                    for k in 0..3 {
                        assert!(matrix.set(&[k, j, i], id).is_ok());
                        id += 1;
                    }
                }
            }

            for element in &matrix {
                assert!(*element != 0);
            }
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
