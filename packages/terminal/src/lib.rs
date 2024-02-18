use linear_algebra::matrix::*;
use std::{io::Write, ops::Range};

mod symbol {
    pub static UPPER: char = '\u{2580}';
    pub static LOWER: char = '\u{2584}';
    pub static FULL: char = '\u{2588}';
    pub static EMPTY: char = ' ';
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

        write!(lock, "{}", Terminal::ansi_clear()).unwrap();

        for row in 0..matrix.rows() {
            if !Self::is_within_range(matrix[(row, 0)], &self.width_range)
                || !Self::is_within_range(matrix[(row, 1)], &self.height_range)
            {
                continue;
            }

            let x = (matrix[(row,0)] as isize + self.width as isize / 2) as usize;
            let y = (matrix[(row,1)] as isize + self.height as isize / 2) as usize;
            let symbol = if y % 2 == 0 {
                symbol::UPPER
            } else {
                symbol::LOWER
            };

            // print!("{}", Self::ansi_set_character_at_pos(symbol, x, y));
            // println!("");
            write!(lock, "{}", Self::ansi_set_character_at_pos(symbol, x, y)).unwrap();
            writeln!(lock).unwrap();
        }
    }

    pub fn ansi_set_character_at_pos(c: char, x: usize, y: usize) -> String {
        format!("\x1b[{};{}H{}", y, x, c)
    }

    pub fn ansi_clear() -> String {
        format!("\x1b[2J")
    }

    fn is_within_range(value: f64, range: &Range<f64>) -> bool {
        value > range.start && value < range.end
    }
}
