use matrix::*;
// use point::*;
// use vector::*;

mod matrix {

    use std::cell::RefCell;

    use super::*;

    struct Matrix<T> {
        matrix: RefCell<Vec<T>>,
        dimensions: Vec<usize>,
    }

    impl Matrix<usize> {
        /// TODO: Better docs.
        /// Example: Vec = {max elements in x, max elements in y, max elements in z}.
        pub fn new(dimensions: Vec<usize>) -> Self {
            Self {
                matrix: RefCell::new(vec![0_usize, dimensions.iter().product()]),
                dimensions,
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

            if coordinates.len() != self.dimensions.len() {
                return Err("Given coordinates cannot be mapped to a valid location in the matrix (out of bounds). I.e. coordinates has too few or too many dimensions.");
            }

            if coordinates[0] > self.dimensions[0] {
                return Err("Given coodinate(s) are out of maximum bounds for current matrix.");
            }

            index += coordinates[0];

            for (coordinate, dimension) in coordinates.iter().skip(1).zip(&self.dimensions) {
                if *coordinate > *dimension {
                    return Err("Given coodinate(s) are out of maximum bounds for current matrix.");
                }

                index += coordinate * dimension;
            }

            Ok(index)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_coordinates_to_index() {
            // let matrix = Matrix::new(vec![1, 2, 3]);
            // let coordinates = [1, 2, 3];
            // let index = matrix.coordinates_to_index(&coordinates);
            // assert!(index.is_ok_and(|x| x == 321));

            // let matrix = Matrix::new(vec![1, 2, 3]);
            // let coordinates = [321, 0, 0];
            // let index = matrix.coordinates_to_index(&coordinates);
            // assert!(index.is_err());

            // let matrix = Matrix::new(vec![1, 2, 3]);
            // let coordinates = [2, 2, 3];
            // let index = matrix.coordinates_to_index(&coordinates);
            // assert!(index.is_err());

            // let matrix = Matrix::new(vec![1, 2, 3]);
            // let coordinates = [1, 3, 3];
            // let index = matrix.coordinates_to_index(&coordinates);
            // assert!(index.is_err());

            // let matrix = Matrix::new(vec![1, 2, 3]);
            // let coordinates = [1, 2, 4];
            // let index = matrix.coordinates_to_index(&coordinates);
            // assert!(index.is_err());

            // let matrix = Matrix::new(vec![1, 2, 3]);
            // let coordinates = [1, 2, 3, 4];
            // let index = matrix.coordinates_to_index(&coordinates);
            // assert!(index.is_err());
        }

        #[test]
        fn test_set_and_get() {
            let matrix = Matrix::new(vec![3, 7, 6]);

            assert!(matrix.set(&[1, 3, 2], 0).is_ok());
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
