use matrix::*;
use point::*;
use vector::*;

mod matrix {
    use super::*;

    struct Matrix<T> {
        matrix: Vec<T>,
        dimensions: Vec<usize>,
    }

    impl Matrix<usize> {
        pub fn new(dimensions: Vec<usize>) -> Self {
            Self {
                matrix: vec![0_usize, dimensions.iter().product()],
                dimensions,
            }
        }

        /// Get value at position [index].
        pub fn get(&self, index: &[usize]) -> Result<usize, &'static str> {
            if index.len() != self.dimensions.len() {
                return Result::Err("Given index does not match size of dimensions.");
            }

            // Result::Ok(*self.matrix.get(Self::coordinate_to_index(&index)).unwrap())
            todo!()
        }

        /// Set [value] at position [index].
        pub fn set(index: Vec<usize>, value: usize) -> Result<usize, ()> {
            todo!()
        }

        fn coordinate_to_index(index: &[usize]) -> usize {
            0
        }
    }
}

mod vector {
    use super::*;

    struct Vector<T> {
        inner: Vec<T>,
    }

    impl Vector<usize> {
        fn new(n_dimensions: usize) -> Self {
            Self { inner: Vec::new() }
        }
    }
}

mod point {
    use super::*;

    /// Same structure as Vector.
    pub struct Point<T> {
        inner: Vec<T>,
    }

    impl Point<usize> {
        fn new(length: usize) -> Self {
            Self {
                dim: length,
                position: vec![0; length],
                matrix_translation: vec![0; (length + 1) * (length + 1)],
            }
        }

        /// Translates a point.
        ///
        /// 2D Example:
        ///       [translation argument]
        ///                |
        ///                v
        /// | 1 0 t1 |   | p1 |   | p1 + t1 |
        /// | 0 1 t2 | * | p2 | = | p2 + t2 |
        /// | 0 0 1  |   | 1  |   | 1       |
        ///
        /// Though... we cheat and translate using simple additionf or each component.
        fn translate(&mut self, translation: &[usize]) {
            if translation.len() > self.dim {
                panic!("Cannot translate using a matrix of higher dimension than the point itself.")
            }

            for (pos, trans) in self.position.iter_mut().zip(translation) {
                *pos += trans;
            }
        }

        fn rotate(axis: usize, degrees: f64) {}
    }
}

// mod matrix {
//     use super::*;

//     /// Matrix holding vectors.
//     pub struct Matrix {
//         vectors: Vec<NVector>,
//     }

//     impl Matrix {
//         fn new(dimensions: usize) -> Self {
//             Self {
//                 vectors: Vec::new(),
//             }
//         }
//     }
// }

pub fn rotate() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
