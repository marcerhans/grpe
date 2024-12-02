pub mod pixel {
    pub struct Meta<'a> {
        /// Pixels cover both upper and lower part of a "real" pixel, so depth is represented for two pixels.
        pub depth_flag: (&'a mut bool, &'a mut bool),
        pub depth: (&'a mut f64, &'a mut f64),

        /// Temporary storage of depth information for polygon border. Upper, lower.
        pub polygon_border_flag: (&'a mut bool, &'a mut bool),
        pub polygon_border: (&'a mut f64, &'a mut f64),
    }

    impl<'a> Meta<'a> {
        pub fn from_slice(slice: &'a mut [u8]) -> Self {
            // Ensure the slice has the correct alignment for f64 (typically 8 bytes) and that it has a valid size.
            debug_assert!(
                slice.as_mut_ptr() as usize % std::mem::align_of::<f64>() == 0,
                "Slice is not properly aligned!"
            );
            debug_assert!(
                slice.len() == Self::required_buffer_size(),
                "Slice does not have a valid size!"
            );

            let f64_slice = slice.as_mut_ptr() as *mut f64;
            let bool_slice = unsafe { f64_slice.add(4) as *mut bool };

            unsafe {
                Self {
                    depth_flag: (&mut *bool_slice.add(0), &mut *bool_slice.add(1)),
                    depth: (&mut *f64_slice.add(0), &mut *f64_slice.add(1)),
                    polygon_border_flag: (&mut *bool_slice.add(2), &mut *bool_slice.add(3)),
                    polygon_border: (&mut *f64_slice.add(2), &mut *f64_slice.add(3)),
                }
            }
        }

        /// Calculates the minimum buffer size required for the struct to have proper references.
        /// Assumes data is packed in a way such that no INTERNAL padding is required, i.e:
        /// [f64, f64, f64, f64, bool, bool, bool, bool]
        /// EXTERNAL padding (after or before) is probably still required.
        #[inline]
        pub const fn required_buffer_size() -> usize {
            (4 * std::mem::size_of::<f64>()) + (4 * std::mem::size_of::<bool>())
        }
    }

    // This can be used instead if color is wanted. Though, it takes a lot of performance.
    pub const VALUE_LEN: usize = 20;
    pub const EMPTY: [char; VALUE_LEN] = [
        '\x1B',
        '[',
        '3',
        '8',
        ';',
        '2',
        ';',
        '2',
        '0',
        '0',
        ';',
        '0',
        '0',
        '0',
        ';',
        '0',
        '0',
        '0',
        'm',
        Char::Empty.value(),
    ];
    // pub const VALUE_LEN: usize = 1;
    // pub const EMPTY: [char; VALUE_LEN] = [Char::Empty.value()];

    pub struct Value<'a> {
        value: &'a mut [char; VALUE_LEN],
    }

    impl<'a> Value<'a> {
        pub fn from_slice(slice: &'a mut [char; VALUE_LEN]) -> Self {
            slice.copy_from_slice(&EMPTY);
            Self { value: slice }
        }

        pub fn get(&self) -> char {
            self.value[VALUE_LEN - 1]
        }

        pub fn set(&mut self, c: Char) {
            self.value[VALUE_LEN - 1] = c.value();
        }
    }

    /// The [Pixel] type only contains references to the owned buffer types, but adds a layer
    /// of abstraction to more easily manipulate the memory.
    pub struct Pixel<'a> {
        pub meta: Meta<'a>,
        pub value: Value<'a>,
    }

    #[derive(PartialEq, Clone)]
    pub enum Char {
        Upper,
        Lower,
        Full,
        Custom(char),
        Empty,
    }

    impl Char {
        pub const fn value(&self) -> char {
            match self {
                Self::Upper => '\u{2580}', // ▀
                Self::Lower => '\u{2584}', // ▄
                Self::Full => '\u{2588}',  // █
                Self::Custom(c) => *c,
                Self::Empty => ' ',
            }
        }

        /// Get appropriate character to use given a vertical position (z).
        pub fn at(z: usize) -> Self {
            if z % 2 != 0 {
                Self::Upper
            } else {
                Self::Lower
            }
        }
    }
}

/// The main purpose of [TerminalBuffer] is to keep continuous buffers for various data, to allow for fast IO and memory manipulation.
/// Editing values in the buffer should only be done via the [pixel::Pixel] (via [TerminalBuffer::pixel_mut]) type.
/// Batch memory manipulations however can be done via the [TerminalBuffer] and are faster.
pub struct TerminalBuffer<'a> {
    metas_bytes: Vec<u8>,
    chars_clear: Vec<char>,
    chars: Vec<char>,
    pixels: Vec<pixel::Pixel<'a>>,
    pixels_dimensions: (usize, usize),
}

impl<'a> TerminalBuffer<'a> {
    pub fn pixels_required(resolution: &(u64, u64)) -> usize {
        ((resolution.0) * (resolution.1 / 2)) as usize
    }

    /// Notice:
    /// "+ (resolution.1 / 2)" is needed to add space for '\n' on every row.
    /// "- 1" to not incude a newline for last line.
    /// Required if resolution != terminal size.
    pub fn chars_required(resolution: &(u64, u64)) -> usize {
        Self::pixels_required(resolution) * pixel::VALUE_LEN + (resolution.1 / 2) as usize - 1
    }

    /// Bytes required for metadata depends on the [pixel::Meta] struct and its memory layout.
    pub fn metas_bytes_required(resolution: &(u64, u64)) -> usize {
        let len = Self::pixels_required(resolution);
        let no_padding = len * pixel::Meta::required_buffer_size();
        let with_padding = no_padding + (len * 4); // +4 for alignment.
        with_padding
    }

    pub fn new(resolution: &(u64, u64)) -> Self {
        debug_assert!(resolution.0 > 0 && resolution.1 > 0);

        let metas_len = Self::metas_bytes_required(resolution);
        let chars_len = Self::chars_required(resolution);
        let pixels_len = Self::pixels_required(resolution);
        let pixels_dimensions = (resolution.0 as usize, (resolution.1 / 2) as usize);

        let mut metas_bytes: Vec<u8> = vec![0; metas_len];
        let mut chars: Vec<char> = vec!['\n' as char; chars_len]; // Might as well inject newlines here.
        let mut pixels: Vec<pixel::Pixel> = Vec::with_capacity(pixels_len);

        let meta_step = pixel::Meta::required_buffer_size() + 4; // +4 for alignment.
        let char_step = pixel::VALUE_LEN;

        let mut col = 0;
        let mut row = 0;
        let index = |col: usize, row: usize| -> usize { col + row * resolution.0 as usize };

        while pixels.len() < pixels_len {
            let index = index(col, row);

            let meta_start = index * meta_step;
            let char_start = index * char_step + row; // +row because we want to skip injected newlines '\n'.

            pixels.push(pixel::Pixel {
                meta: pixel::Meta::from_slice(unsafe {
                    std::slice::from_raw_parts_mut(
                        metas_bytes.as_mut_ptr().add(meta_start),
                        pixel::Meta::required_buffer_size(),
                    )
                }),
                value: pixel::Value::from_slice(unsafe {
                    std::slice::from_raw_parts_mut(chars.as_mut_ptr().add(char_start), char_step)
                        .try_into()
                        .unwrap()
                }),
            });

            col += 1;
            if col == pixels_dimensions.0 {
                // Go to next row.
                // Has to be incremented here (before setting values) to set correct row offset.
                row += 1;
                col = 0;
            }
        }

        Self {
            metas_bytes,
            chars_clear: chars.clone(),
            chars,
            pixels,
            pixels_dimensions,
        }
    }

    pub fn chars(&self) -> &[char] {
        &self.chars
    }

    pub fn clear(&mut self) {
        self.metas_bytes.fill(0);
        self.chars.clone_from_slice(&self.chars_clear);
    }

    pub fn pixel(&self, row: usize, col: usize) -> &pixel::Pixel<'a> {
        &self.pixels[col + row * self.pixels_dimensions.0]
    }

    pub fn pixel_mut(&mut self, row: usize, col: usize) -> &mut pixel::Pixel<'a> {
        &mut self.pixels[col + row * self.pixels_dimensions.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_and_check(buffer: &mut TerminalBuffer, c: pixel::Char, at: &[(usize, usize)]) {
        for (row, col) in at {
            let pixel = buffer.pixel_mut(*row, *col);
            pixel.value.set(c.clone());
            assert!(pixel.value.get() == c.value());
            assert!(
                buffer.chars[(col + row * buffer.pixels_dimensions.0) * pixel::VALUE_LEN
                    + (pixel::VALUE_LEN - 1) // ("+ pixel::VALUE_LEN - 1" | Ignore pixel ANSI formatting data, and just look at the actual value.).
                    + row] // ("+ row" |  Take newlines into consideration, i.e. ignore them).
                    == c.value()
            );
        }
    }

    fn check_for_value_in_buffer(
        buffer: &TerminalBuffer,
        c: pixel::Char,
        except_values_at: &[(usize, usize)],
    ) {
        for row in 0..buffer.pixels_dimensions.1 {
            'a: for col in 0..buffer.pixels_dimensions.0 {
                for exception in except_values_at {
                    if row == exception.0 && col == exception.1 {
                        continue 'a;
                    }
                }

                assert!(buffer.pixel(row, col).value.get() == c.value());
                assert!(
                    buffer.chars[(col + row * buffer.pixels_dimensions.0) * pixel::VALUE_LEN
                    + (pixel::VALUE_LEN - 1) // ("+ pixel::VALUE_LEN - 1" | Ignore pixel ANSI formatting data, and just look at the actual value.).
                    + row] // ("+ row" |  Take newlines into consideration, i.e. ignore them).
                        == c.value()
                );
            }
        }
    }

    fn newlines_are_present(buffer: &TerminalBuffer) {
        for row in 0..(buffer.pixels_dimensions.1 - 1) {
            assert!(
                buffer.chars[(buffer.pixels_dimensions.0 + (row * buffer.pixels_dimensions.0))
                    * pixel::VALUE_LEN
                    + row]
                    == '\n'
            );
        }
    }

    #[test]
    fn lengths() {
        let tester = |width: u64, height: u64| {
            let resolution = (width, height);
            let buffer = TerminalBuffer::new(&resolution);
            assert!(
                buffer.metas_bytes.len() == TerminalBuffer::metas_bytes_required(&resolution),
                "Actual: {}",
                buffer.metas_bytes.len()
            );
            assert!(
                buffer.chars.len() == TerminalBuffer::chars_required(&resolution),
                "Actual: {}",
                buffer.chars.len()
            );
            assert!(
                buffer.pixels.len() == TerminalBuffer::pixels_required(&resolution),
                "Actual: {}",
                buffer.pixels.len()
            );
        };
        for (i, j) in (1..=100).zip(2..=100) {
            tester(i, j);
        }
        tester(742, 393);
    }

    #[test]
    fn set_pixel_value() {
        {
            let resolution = (10, 10);
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Char::Full, &[(0, 0)]);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &[(0, 0)]);
            newlines_are_present(&buffer);
        }
        {
            let resolution = (10, 10);
            let row_col: [(usize, usize); 2] = [(4, 3), (2, 5)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Char::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &row_col);
            newlines_are_present(&buffer);
        }
        {
            let resolution = (735, 92);
            let row_col: [(usize, usize); 2] = [(45, 69), (30, 500)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Char::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &row_col);
            newlines_are_present(&buffer);
        }
    }

    #[test]
    fn reset_pixel_values() {
        {
            let resolution = (10, 10);
            let row_col: [(usize, usize); 2] = [(4, 3), (2, 5)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Char::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.pixel_mut(2, 5).value.set(pixel::Char::Empty);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &[(4, 3)]);
            newlines_are_present(&buffer);
            buffer.pixel_mut(4, 3).value.set(pixel::Char::Empty);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &[]);
            newlines_are_present(&buffer);
        }
    }

    #[test]
    fn clear_buffer() {
        {
            let resolution = (10, 10);
            let row_col: [(usize, usize); 2] = [(4, 3), (2, 5)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Char::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.clear();
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &[]);
            newlines_are_present(&buffer);
        }
    }

    #[test]
    fn newline_handling() {
        {
            let resolution = (2, 4);
            let row_col: [(usize, usize); 2] = [(0, 0), (1, 0)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Char::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.clear();
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &[]);
            newlines_are_present(&buffer);
        }

        {
            let resolution = (2, 4);
            let row_col: [(usize, usize); 2] = [(0, 1), (1, 1)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Char::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.clear();
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &[]);
            newlines_are_present(&buffer);
        }

        {
            let resolution = (2, 4);
            let row_col: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Char::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.clear();
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &[]);
            newlines_are_present(&buffer);
        }

        {
            let resolution = (131, 749);
            let row_col: [(usize, usize); 5] = [(0, 0), (1, 0), (2, 0), (300, 0), (0, 130)];
            let mut buffer = TerminalBuffer::new(&resolution);
            set_and_check(&mut buffer, pixel::Char::Full, &row_col);
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &row_col);
            newlines_are_present(&buffer);
            buffer.clear();
            check_for_value_in_buffer(&buffer, pixel::Char::Empty, &[]);
            newlines_are_present(&buffer);
        }
    }
}
