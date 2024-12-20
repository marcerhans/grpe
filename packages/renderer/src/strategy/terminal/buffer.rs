pub mod pixel {
    // pub const VALUE_MAX_LEN: usize = 20;
    // pub const EMPTY: [char; VALUE_MAX_LEN] = [
    //     '\x1B',
    //     '[',
    //     '3',
    //     '8',
    //     ';',
    //     '2',
    //     ';',
    //     '0',
    //     '0',
    //     '0',
    //     ';',
    //     '0',
    //     '0',
    //     '0',
    //     ';',
    //     '0',
    //     '0',
    //     '0',
    //     'm',
    //     Value::Empty.value(),
    // ];
    pub const VALUE_MAX_LEN: usize = 1;
    pub const EMPTY: [char; VALUE_MAX_LEN] = [Value::Empty.value()];

    pub struct Pixel {
        /// Pixels cover both upper and lower part of a "real" pixel, so depth is represented for two pixels.
        pub depth: (Option<f64>, Option<f64>),

        /// Temporary storage of depth information for polygon border. Upper, lower.
        pub polygon_border: (Option<f64>, Option<f64>),

        /// Slice of buffer in [super::TerminalBuffer].
        slice: &'static mut [char],
    }

    impl Pixel {
        pub fn new(slice: &'static mut [char]) -> Self {
            assert!(slice.len() == VALUE_MAX_LEN);
            slice.copy_from_slice(&EMPTY);
            Self {
                depth: (None, None),
                polygon_border: (None, None),
                slice,
            }
        }

        pub fn reset(&mut self) {
            self.depth = (None, None);
            self.polygon_border = (None, None);
            self.slice.copy_from_slice(&EMPTY);
        }

        pub fn value(&self) -> char {
            self.slice[VALUE_MAX_LEN - 1]
        }

        pub fn set_value(&mut self, value: Value) {
            self.slice[VALUE_MAX_LEN - 1] = value.value();
        }

        // pub fn set_color(&mut self, rgb: &RGB) {
        //     self.value[7..].copy_from_slice(&rgb.0);
        //     self.value[11..].copy_from_slice(&rgb.1);
        //     self.value[15..].copy_from_slice(&rgb.2);
        // }
    }

    #[derive(PartialEq, Clone)]
    pub enum Value {
        Upper,
        Lower,
        Full,
        Custom(char),
        Empty,
    }

    impl Value {
        pub const fn value(&self) -> char {
            match self {
                Value::Upper => '\u{2580}', // ▀
                Value::Lower => '\u{2584}', // ▄
                Value::Full => '\u{2588}',  // █
                Value::Custom(c) => *c,
                Value::Empty => ' ',
            }
        }

        /// Get appropriate character to use for given vertical position.
        pub fn at(z: usize) -> Self {
            if z % 2 != 0 {
                Value::Upper
            } else {
                Value::Lower
            }
        }
    }

    // pub struct RGB([char; 3], [char; 3], [char; 3]);

    // impl Default for RGB {
    //     fn default() -> Self {
    //         Self(['0'; 3], ['0'; 3], ['0'; 3])
    //     }
    // }
}

/// The main purpose of [TerminalBuffer] is to keep a continuous buffer such that fast IO can be achieved.
/// Editing values in the buffer should only be done via the [pixel::Pixel] (via [TerminalBuffer::pixel_mut]).
pub struct TerminalBuffer {
    data: Vec<char>,
    meta: Vec<pixel::Pixel>,
    meta_dimensions: (usize, usize),
}

impl TerminalBuffer {
    pub fn data_len(resolution: &(u64, u64)) -> usize {
        ((resolution.0) * (resolution.1 / 2)) as usize * pixel::VALUE_MAX_LEN
            + (resolution.1 / 2) as usize // "+ (resolution.1 / 2)" for all '\n'. Essentially it adds space for '\n' on every row.
    }

    pub fn new(resolution: &(u64, u64)) -> Self {
        assert!(resolution.0 > 0 && resolution.1 > 0);

        let data_len = Self::data_len(resolution);
        let meta_dimensions = (resolution.0 as usize, (resolution.1 / 2) as usize);
        let meta_len = meta_dimensions.0 * meta_dimensions.1;

        let mut data = vec![0 as char; data_len];
        let mut meta: Vec<pixel::Pixel> = Vec::with_capacity(meta_len);

        let mut col = 0;
        let mut row = 0;
        let mut index = (col + row * resolution.0 as usize) * pixel::VALUE_MAX_LEN;

        while meta.len() < meta_len {
            meta.push(pixel::Pixel::new(unsafe {
                std::slice::from_raw_parts_mut(data.as_mut_ptr().add(index), pixel::VALUE_MAX_LEN)
            }));

            col += 1;

            if col == meta_dimensions.0 {
                // Go to next row.
                data[(col + row * resolution.0 as usize) * pixel::VALUE_MAX_LEN + row] = '\n';
                row += 1;
                col = 0;
            }

            // "+ row" to include extra newlines ('\n').
            index = (col + row * resolution.0 as usize) * pixel::VALUE_MAX_LEN + row;
        }

        Self {
            data: data,
            meta: meta,
            meta_dimensions,
        }
    }

    pub fn data(&self) -> &Vec<char> {
        &self.data
    }

    pub fn clear(&mut self) {
        self.meta.iter_mut().for_each(|pixel| pixel.reset());
    }

    pub fn pixel(&self, row: usize, col: usize) -> &pixel::Pixel {
        &self.meta[col + row * self.meta_dimensions.0]
    }

    pub fn pixel_mut(&mut self, row: usize, col: usize) -> &mut pixel::Pixel {
        &mut self.meta[col + row * self.meta_dimensions.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_and_check(buffer: &mut TerminalBuffer, value: pixel::Value, at: &[(usize, usize)]) {
        for (row, col) in at {
            buffer.pixel_mut(*row, *col).set_value(value.clone());
            assert!(buffer.pixel(*row, *col).value() == value.value());
            assert!(
                buffer.data[((col + row * buffer.meta_dimensions.0) * pixel::VALUE_MAX_LEN + row)
                    + (pixel::VALUE_MAX_LEN - 1)]
                    == value.value()
            );
        }
    }

    fn check_for_value_in_buffer(
        buffer: &TerminalBuffer,
        value: pixel::Value,
        except_values_at: &[(usize, usize)],
    ) {
        for row in 0..buffer.meta_dimensions.1 {
            'a: for col in 0..buffer.meta_dimensions.0 {
                for exception in except_values_at {
                    if row == exception.0 && col == exception.1 {
                        continue 'a;
                    }
                }

                assert!(buffer.pixel(row, col).value() == value.value());
                assert!(
                    buffer.data[((col + row * buffer.meta_dimensions.0) * pixel::VALUE_MAX_LEN
                        + row)
                        + (pixel::VALUE_MAX_LEN - 1)]
                        == value.value()
                );
            }
        }
    }

    fn newlines_are_present(buffer: &TerminalBuffer) {
        for row in 0..buffer.meta_dimensions.1 {
            assert!(
                buffer.data[(row * buffer.meta_dimensions.0) * pixel::VALUE_MAX_LEN
                    + pixel::VALUE_MAX_LEN * buffer.meta_dimensions.0
                    + row]
                    == '\n'
            );
        }
    }

    #[test]
    fn lengths() {
        {
            let resolution = (10, 10);
            let buffer = TerminalBuffer::new(&resolution);
            assert!(
                buffer.data.len() == TerminalBuffer::data_len(&resolution),
                "Actual: {}",
                buffer.data.len()
            );
            assert!(
                buffer.meta.len() == (resolution.0 * (resolution.1 / 2)) as usize,
                "Actual: {} | Expected: {}",
                buffer.meta.len(),
                (resolution.0 * (resolution.1 / 2)) as usize
            );
        }
        {
            let resolution = (10, 9);
            let buffer = TerminalBuffer::new(&resolution);
            assert!(
                buffer.data.len() == TerminalBuffer::data_len(&resolution),
                "Actual: {}",
                buffer.data.len()
            );
            assert!(
                buffer.meta.len() == (resolution.0 * (resolution.1 / 2)) as usize,
                "Actual: {} | Expected: {}",
                buffer.meta.len(),
                (resolution.0 * (resolution.1 / 2)) as usize
            );
        }
        {
            let resolution = (742, 393);
            let buffer = TerminalBuffer::new(&resolution);
            assert!(
                buffer.data.len() == TerminalBuffer::data_len(&resolution),
                "Actual: {}",
                buffer.data.len()
            );
            assert!(
                buffer.meta.len() == (resolution.0 * (resolution.1 / 2)) as usize,
                "Actual: {} | Expected: {}",
                buffer.meta.len(),
                (resolution.0 * (resolution.1 / 2)) as usize
            );
        }
    }

    #[test]
    fn set_pixel_value() {
        {
            let resolution = (10, 10);
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Value::Full, &[(0, 0)]);
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &[(0, 0)]);
            newlines_are_present(&buffer);
        }
        {
            let resolution = (10, 10);
            let row_col: [(usize, usize); 2] = [(4, 3), (2, 5)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Value::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &row_col);
            newlines_are_present(&buffer);
        }
    }

    #[test]
    fn reset_pixel_values() {
        {
            let resolution = (10, 10);
            let row_col: [(usize, usize); 2] = [(4, 3), (2, 5)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Value::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.pixel_mut(2, 5).reset();
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &[(4, 3)]);
            newlines_are_present(&buffer);
            buffer.pixel_mut(4, 3).reset();
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &[]);
            newlines_are_present(&buffer);
        }
    }

    #[test]
    fn clear_buffer() {
        {
            let resolution = (10, 10);
            let row_col: [(usize, usize); 2] = [(4, 3), (2, 5)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Value::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.clear();
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &[]);
            newlines_are_present(&buffer);
        }
    }

    #[test]
    fn newline_handling() {
        {
            let resolution = (2, 4);
            let row_col: [(usize, usize); 2] = [(0, 0), (1, 0)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Value::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.clear();
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &[]);
            newlines_are_present(&buffer);
        }

        {
            let resolution = (2, 4);
            let row_col: [(usize, usize); 2] = [(0, 1), (1, 1)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Value::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.clear();
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &[]);
            newlines_are_present(&buffer);
        }

        {
            let resolution = (2, 4);
            let row_col: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Value::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.clear();
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &[]);
            newlines_are_present(&buffer);
        }

        {
            let resolution = (131, 749);
            let row_col: [(usize, usize); 5] = [(0, 0), (1, 0), (2, 0), (300, 0), (0, 130)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Value::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.clear();
            check_for_value_in_buffer(&buffer, pixel::Value::Empty, &[]);
            newlines_are_present(&buffer);
        }
    }
}
