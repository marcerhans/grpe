use linear_algebra::matrix::*;
use std::{io::Write, ops::Range};

mod symbol {
    pub static UPPER: &str = "u{2580}";
    pub static LOWER: &str = "u{2584}";
    pub static FULL: &str = "u{2588}";
    pub static EMPTY: &str = " ";
}

pub struct Terminal {
    width: usize,
    height: usize,
    width_range: Range<f64>,
    height_range: Range<f64>,
}

impl Terminal {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            width_range: -((width / 2) as f64)..(width / 2) as f64,
            height_range: -((height / 2) as f64)..(height / 2) as f64,
        }
    }

    pub fn draw(&self, matrix: &Matrix) {
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();

        for row in 0..matrix.rows() {
            if !Self::is_within_range(matrix[(row, 0)], &self.width_range)
                || !Self::is_within_range(matrix[(row, 1)], &self.height_range)
            {
                continue;
            }

            for column in 0..matrix.columns()-1 {
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

    fn is_within_range(value: f64, range: &Range<f64>) -> bool {
        value > range.start && value < range.end
    }
}
