use linear_algebra::matrix::*;
use std::{cell::RefCell, io::Write, ops::Range};

mod symbol {
    pub static LINE_HORIZONTAL: char = '\u{254c}';
    pub static LINE_VERTICAL: char = '\u{2506}';
    pub static CENTER: char = '\u{253c}';
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
    state: RefCell<Vec<Vec<char>>>,
}

impl Terminal {
    pub fn new(width: usize, height: usize) -> Self {
        if width % 2 == 0 || height % 2 == 0 {
            panic!("With and height have to be uneven.");
        }

        let mut state = vec![vec![symbol::EMPTY; width]; height / 2 + 1];

        for row in 0..state.len() {
            if row == height / 4 && (row + 1) * 4 != height {
                for column in 0..state[0].len() {
                    if column == width / 2 && (column + 1) * 2 != width {
                        state[row][column] = symbol::CENTER;
                    } else {
                        state[row][column] = symbol::LINE_HORIZONTAL;
                    }
                }
            } else {
                state[row][width / 2] = symbol::LINE_VERTICAL;
            }
        }

        Self {
            width,
            height,
            width_range: -((width / 2) as f64)..(width / 2) as f64,
            height_range: -((height / 2) as f64)..(height / 2) as f64,
            state: RefCell::new(state),
        }
    }

    pub fn update(&self, matrix: &Matrix) {
        let mut state = self.state.borrow_mut();

        for row in 0..state.len()*2 {
            for column in 0..state[0].len() {
                if row % 2 == 0 {
                    state[row / 2][column] = symbol::EMPTY;
                }

                for row_m in 0..matrix.rows() {
                    if matrix[(row_m, 0)] as usize == column && matrix[(row_m, 1)] as usize == row {
                        let symbol;

                        if state[row / 2][column] == symbol::UPPER {
                            symbol = symbol::FULL;
                        } else if row % 2 == 0 {
                            symbol = symbol::UPPER;
                        } else {
                            symbol = symbol::LOWER;
                        }

                        state[row / 2][column] = symbol;
                        break;
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        let state = self.state.borrow();

        for row in 0..state.len() {
            for column in 0..state[0].len() {
                print!("{}", state[row][column]);
            }
            println!();
        }

        // let stdout = std::io::stdout();
        // let mut lock = stdout.lock();

        // write!(lock, "{}", Terminal::ansi_clear()).unwrap();
        // println!("{}", Terminal::ansi_clear());
        // let mut state = self.state.borrow_mut();

        // for row in 0..state.len() {
        //     let mut row_string = vec![symbol::EMPTY; state[0].len()];

        //     for column in 0..(state[0].len()*2) {
        //         let mut symbol = symbol::EMPTY;

        //         for row_m in 0..matrix.rows() {
        //             if matrix[(row_m,0)] as usize == column && matrix[(row_m,1)] as usize == row {
        //                 if row % 2 != 0 && symbol != symbol::EMPTY {
        //                     symbol = symbol::FULL;
        //                 } else if row % 2 != 0 {
        //                     symbol = symbol::LOWER;
        //                 } else {
        //                     symbol = symbol::UPPER;
        //                 }

        //                 break;
        //             }
        //         }

        //         state[row][column/2] = symbol;
        //         row_string.push(symbol);
        //     }

        //     println!("{}", row_string.iter().collect::<String>());
        // }

        // for row in 0..matrix.rows() {
        //     // if !Self::is_within_range(matrix[(row, 0)], &self.width_range)
        //     //     || !Self::is_within_range(matrix[(row, 1)], &self.height_range)
        //     // {
        //     //     continue;
        //     // }

        //     // let x = (matrix[(row,0)] as isize + self.width as isize / 2) as usize;
        //     // let y = (matrix[(row,1)] as isize + self.height as isize / 2) as usize;
        //     let x = matrix[(row,0)] as usize;
        //     let y = matrix[(row,0)] as usize;

        //     // let symbol = if y % 2 == 0 {
        //     //     symbol::UPPER
        //     // } else {
        //     //     if y > 0 && self.state.borrow()[y][x] == symbol::LOWER {

        //     //     }
        //     //     symbol::LOWER
        //     // };
        //     let symbol = symbol::FULL;

        //     // Update state and draw (print)
        //     self.state.borrow_mut()[x][y] = symbol;
        //     // println!("{:?}", self.state);

        //     print!("{}", Self::ansi_set_character_at_pos(symbol, x, y));
        //     println!();

        //     // write!(lock, "{}", Self::ansi_set_character_at_pos(symbol, x, y)).unwrap();
        //     // writeln!(lock).unwrap();
        // }
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
