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
    //     PixelValue::Empty.value(),
    // ];
    pub const VALUE_MAX_LEN: usize = 1;
    pub const EMPTY: [char; VALUE_MAX_LEN] = [Value::Empty.value()];

    /////// TODO: Redo docs!
    // / [Pixel] exists mainly to lower the amount of memory allocations during tight loops.
    // / Due to text covering two pixels, the [Pixel] covers the upper and lower part of a true pixel.
    // / The first tuple value represents the upper portion, while the second the lower.
    pub struct Pixel {
        /// Pixels cover both upper and lower part of a "real" pixel, so depth is represented for two pixels.
        pub depth: (Option<f64>, Option<f64>),

        /// Used for defining the edges of a polygon to be filled. Is switched of when polygon has been filled.
        pub polygon_fill_border: (bool, bool),

        /// (Pointer to slice of a memory buffer, length of slice)
        /// Note that this struct should be regarded as the true owner of the buffer data. The only reason for it to be part
        /// of the [super::Canvas] struct is to allow for faster IO.
        slice: &'static mut [char],
    }

    impl Pixel {
        pub fn new(slice: &'static mut [char]) -> Self {
            assert!(slice.len() == VALUE_MAX_LEN);
            slice.copy_from_slice(&EMPTY);
            Self {
                depth: (None, None),
                polygon_fill_border: (false, false),
                slice,
            }
        }

        pub fn reset(&mut self) {
            self.depth = (None, None);
            self.polygon_fill_border = (false, false);
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

    #[derive(PartialEq)]
    pub enum Value {
        Upper,
        Lower,
        Full,
        Empty,
    }

    impl Value {
        pub const fn value(&self) -> char {
            match self {
                Value::Upper => '\u{2580}', // ▀
                Value::Lower => '\u{2584}', // ▄
                Value::Full => '\u{2588}',  // █
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

pub struct Buffer {
    data: Vec<char>,
    meta: Vec<pixel::Pixel>,
    meta_dimensions: (usize, usize),
}

impl Buffer {
    pub fn len(resolution: &(u64, u64)) -> usize {
        ((resolution.0 + 1) * (resolution.1 / 2)) as usize // +1 for all '\n'.
    }

    pub fn new(resolution: &(u64, u64)) -> Self {
        let data_len = Self::len(resolution);
        let resolution = (resolution.0 as usize, (resolution.1 / 2) as usize);
        let meta_len = resolution.0 * resolution.1;

        let mut data = vec![0 as char; data_len * pixel::VALUE_MAX_LEN];
        let mut meta: Vec<pixel::Pixel> = Vec::with_capacity(meta_len);

        let mut col = 0;
        let mut row = 0;
        let mut index = col + row * resolution.0;

        while meta.len() < meta_len {
            meta.push(pixel::Pixel::new(unsafe {
                std::slice::from_raw_parts_mut(data.as_mut_ptr().add(index), pixel::VALUE_MAX_LEN)
            }));

            col += pixel::VALUE_MAX_LEN;

            if col == resolution.0 as usize {
                // Go to next row.
                // We don't check for "resolution.0 + 1" because we don't want to include newline ('\n') in our [meta::Pixel]s.
                row += 1;
                col = 0;
            }

            index = col + row * resolution.0;
        }

        Self {
            data: data,
            meta: meta,
            meta_dimensions: resolution,
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(0 as char);
        self.meta.iter_mut().for_each(|pixel| pixel.reset());
    }

    pub fn pixel(&mut self, col: usize, row: usize) -> &mut pixel::Pixel {
        &mut self.meta[col + row * self.meta_dimensions.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lengths() {
        {
            let resolution = (10, 10);
            let buffer = Buffer::new(&resolution);
            assert!(
                buffer.data.len() == Buffer::len(&resolution),
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
            let buffer = Buffer::new(&resolution);
            assert!(
                buffer.data.len() == Buffer::len(&resolution),
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
            let buffer = Buffer::new(&resolution);
            assert!(
                buffer.data.len() == Buffer::len(&resolution),
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
            let mut buffer = Buffer::new(&resolution);

            buffer.pixel(0, 0).set_value(pixel::Value::Full);
            assert!(buffer.pixel(0, 0).value() == pixel::Value::Full.value());
            assert!(buffer.data[0] == pixel::Value::Full.value());

            for row in 0..buffer.meta_dimensions.1 {
                for col in 0..buffer.meta_dimensions.0 {
                    if row != 0 && col != 0 {
                        assert!(buffer.pixel(col, row).value() == pixel::Value::Empty.value());
                        assert!(
                            buffer.data[col + row * resolution.0 as usize]
                                == pixel::Value::Empty.value()
                        );
                    }
                }
            }
        }
        {
            let resolution = (10, 10);
            let mut buffer = Buffer::new(&resolution);

            buffer.pixel(3, 4).set_value(pixel::Value::Full);
            assert!(buffer.pixel(3, 4).value() == pixel::Value::Full.value());
            assert!(buffer.data[3 + 4 * resolution.0 as usize] == pixel::Value::Full.value());

            buffer.pixel(5, 2).set_value(pixel::Value::Full);
            assert!(buffer.pixel(5, 2).value() == pixel::Value::Full.value());
            assert!(buffer.data[5 + 2 * resolution.0 as usize] == pixel::Value::Full.value());

            for row in 0..buffer.meta_dimensions.1 {
                for col in 0..buffer.meta_dimensions.0 {
                    if (row == 4 && col == 3) || (row == 2 && col == 5) {
                        continue;
                    }

                    assert!(buffer.pixel(col, row).value() == pixel::Value::Empty.value());
                    assert!(
                        buffer.data[col + row * resolution.0 as usize]
                            == pixel::Value::Empty.value()
                    );
                }
            }
        }
    }

    #[test]
    fn reset_pixel_values() {
        {
            let resolution = (10, 10);
            let mut buffer = Buffer::new(&resolution);

            buffer.pixel(3, 4).set_value(pixel::Value::Full);
            assert!(buffer.pixel(3, 4).value() == pixel::Value::Full.value());
            assert!(buffer.data[3 + 4 * resolution.0 as usize] == pixel::Value::Full.value());

            buffer.pixel(5, 2).set_value(pixel::Value::Full);
            assert!(buffer.pixel(5, 2).value() == pixel::Value::Full.value());
            assert!(buffer.data[5 + 2 * resolution.0 as usize] == pixel::Value::Full.value());

            for row in 0..buffer.meta_dimensions.1 {
                for col in 0..buffer.meta_dimensions.0 {
                    if (row == 4 && col == 3) || (row == 2 && col == 5) {
                        continue;
                    }

                    assert!(buffer.pixel(col, row).value() == pixel::Value::Empty.value());
                    assert!(
                        buffer.data[col + row * resolution.0 as usize]
                            == pixel::Value::Empty.value()
                    );
                }
            }

            buffer.pixel(5, 2).reset();

            for row in 0..buffer.meta_dimensions.1 {
                for col in 0..buffer.meta_dimensions.0 {
                    if row == 4 && col == 3 {
                        continue;
                    }

                    assert!(buffer.pixel(col, row).value() == pixel::Value::Empty.value());
                    assert!(
                        buffer.data[col + row * resolution.0 as usize]
                            == pixel::Value::Empty.value()
                    );
                }
            }

            buffer.pixel(3, 4).reset();

            for row in 0..buffer.meta_dimensions.1 {
                for col in 0..buffer.meta_dimensions.0 {
                    assert!(buffer.pixel(col, row).value() == pixel::Value::Empty.value());
                    assert!(
                        buffer.data[col + row * resolution.0 as usize]
                            == pixel::Value::Empty.value()
                    );
                }
            }
        }
    }
}
