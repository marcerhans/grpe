use point::*;
// use matrix::*;

mod matrix {
    use super::*;

    enum InnerMatrix {
        Inner(Box<[InnerMatrix]>),
        Matrix(Box<[usize]>),
    }

    /// N-dim matrix
    pub struct Matrix {
        matrix: Option<Box<Matrix>>,
        array: [usize],
    }

    impl Matrix {
        fn new(n_dimensions: usize) {
            // let matrix = Matrix(Box::new([0; n_dimensions]));
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn nested_matrix() {
        }
    }
}

mod point {
    use super::*;

    /// Point in N:th dimension.
    #[derive(Debug)]
    pub struct Point {
        dim: usize,
        position: Vec<usize>,
        matrix_translation: Vec<usize>,
    }

    impl Point {
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

pub fn rotate() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
