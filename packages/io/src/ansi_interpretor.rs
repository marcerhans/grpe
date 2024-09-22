use crate::{platform::unix::ToChar, Event, MouseEvent, Modifier, Motion};
use util::{CharArray, Ansi};

mod util {
    use crate::{Modifier, MouseEvent, Motion};

    pub struct CharArray<const SIZE: usize, F: Fn() -> Option<char>> {
        array: [char; SIZE],
        pos: usize,
        reader: F,
    }

    impl<const SIZE: usize, F: Fn() -> Option<char>> CharArray<SIZE, F> {
        pub fn new(reader: F) -> Self {
            Self {
                array: [Default::default(); SIZE],
                pos: 0,
                reader,
            }
        }

        pub fn read(&mut self) -> Result<char, &'static str> {
            if self.pos > SIZE {
                return Err("Array is full.");
            }

            if let Some(c) = (self.reader)() {
                self.array[self.pos] = c;
                self.pos += 1;
                return Ok(c)
            }

            Err("Failed to read character.")
        }

        pub fn last(&self) -> Option<char> {
            if self.pos == 0 {
                return None;
            }

            Some(self.array[self.pos - 1])
        }
    }

    impl<const SIZE: usize, F: Fn() -> Option<char>> std::ops::Index<usize> for CharArray<SIZE, F> {
        type Output = char;
    
        fn index(&self, index: usize) -> &Self::Output {
            &self.array[index]
        }
    }

    pub trait Ansi {
        fn is_escape(&mut self) -> Result<bool, &'static str>;
        fn is_sequence(&mut self) -> Result<bool, &'static str>;
        fn is_mouse_tracking(&mut self) -> Result<(Modifier, MouseEvent), &'static str>;
    }

    impl<const SIZE: usize, F: Fn() -> Option<char>> Ansi for CharArray<SIZE, F> {
        fn is_escape(&mut self) -> Result<bool, &'static str> {
            if let Ok(c) = self.read() {
                return Ok(c == '\x1b');
            }

            Err("Failed to read.")
        }

        fn is_sequence(&mut self) -> Result<bool, &'static str> {
            if self.pos != 1 {
                return Err("Not in correct state to check for CSI symbols.");
            }

            if let Ok(c) = self.read() {
                return Ok(c == '[');
            }

            Err("Failed to read.")
        }

        fn is_mouse_tracking(&mut self) -> Result<(Modifier, MouseEvent), &'static str> {
            if self.pos != 2 {
                return Err("Not in correct state to check for mouse tracking sequence.");
            }

            if let Ok(c) = self.read() {
                if c != '<' {
                    return Err("Not a mouse tracking sequence.")
                }
            }

            // Parse (motion + button type).
            // Easiest way to figure this out is to run unix_test.c or look at docs for xterm control sequences.
            // None + 00110000 = Click left (0)
            // None + 00110001 = Click middle (1)
            // None + 00110010 = Click right (2)
            // 00110011 + 00110000 = Move left (32)
            // 00110011 + 00110001 = Move middle (33)
            // 00110011 + 00110010 = Move right (34)
            // 00110110 + 00110100 = Scroll up (64)
            // 00110110 + 00110101 = Scroll down (65)
            let mut expected_semicolons_left = 2;
            let mut expected_m_left = 1;

            // while match self.read() {
            //     Ok(c) => match c {
            //         ';' | 'm' | 'M' => {
            //             true
            //         }
            //         _ => false,
            //     }
            //     Err(_) => return None,
            // } {}

            Err("Failed to read.")
        }
    }


}

pub fn interpret<F: Fn() -> Option<char>>(reader: F) -> Option<Event> {
    let mut chars = CharArray::<64, F>::new(reader);

    if let Ok(is_escape) = chars.is_escape() {
        if !is_escape {
            // Not an escape sequence. Just a character.
            return Some(Event::Character(chars.last().unwrap()));
        }
    }

    if let Ok(is_sequence) = chars.is_sequence() {
        if is_sequence {
            if let Ok((modifier, event)) = chars.is_mouse_tracking() {
                return Some(Event::Mouse(modifier, event));
            }
        }
    }

    None
}