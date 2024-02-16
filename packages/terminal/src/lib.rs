use linear_algebra::matrix::*;
use std::io::Write;

mod symbol {
    pub static UPPER: &str = "u{2580}";
    pub static LOWER: &str = "u{2584}";
    pub static FULL: &str = "u{2588}";
    pub static EMPTY: &str = " ";
}

pub struct Terminal {
    width: usize,
    height: usize,
}

impl Terminal {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn draw(matrix: &Matrix) {
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();

        for row in 0..matrix.rows() {
            for column in 0..matrix.columns() {
                let mut symbol = symbol::EMPTY;

                if row % 2 == 0 {
                    symbol = symbol::UPPER;
                } else {
                    symbol = symbol::LOWER;
                }

                write!(lock, "{}", symbol::LOWER).unwrap();
            }

            writeln!(lock);
        }
    }
}