use crate::Event;
use util::{Ansi, CharArray};

mod util {
    use crate::{
        platform::unix::ToU8, Direction, Modifier, Motion, MouseButton, MouseEvent, MouseEventBuilder
    };

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
                return Ok(c);
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

    impl<const SIZE: usize, Idx, F: Fn() -> Option<char>> std::ops::Index<Idx> for CharArray<SIZE, F> 
    where Idx: std::slice::SliceIndex<[char]> {
        type Output = Idx::Output;
    
        fn index(&self, index: Idx) -> &Self::Output {
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
            if self.pos != 0 {
                return Err("Not in correct state to check for escape sequence.");
            }

            if let Ok(c) = self.read() {
                return Ok(c == '\x1b');
            }

            Err("Failed to read.")
        }

        fn is_sequence(&mut self) -> Result<bool, &'static str> {
            if self.pos != 1 {
                return Err("Not in correct state to check for sequence start symbol ('[').");
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
                    return Err("Not a mouse tracking sequence.");
                }
            }

            // Read data and check formatting
            // Note: If parsing took place here too, it would be faster
            //       but I'd rather have the separation.
            let mut semicolons_left: usize = 2;
            let mut semicolon_positions: [usize; 2] = [0; 2];
            let mut m_position: usize = 0;
            while match self.read() {
                Ok(c) => {
                    if c == ';' {
                        if semicolons_left > 0 {
                            semicolon_positions[2 - semicolons_left] = self.pos - 1;
                        }

                        if let Some(new) = semicolons_left.checked_sub(1) {
                            semicolons_left = new;
                        } else {
                            return Err("Badly formatted sequence.");
                        }

                        true
                    } else {
                        if c == 'm' || c == 'M' {
                            m_position = self.pos - 1;
                            false
                        } else {
                            true
                        }
                    }
                }
                Err(_) => false,
            } {}

            // Parsing of read data.
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
            let mut meb = MouseEventBuilder::default();

            if self[m_position] == 'M' {
                meb.motion = Some(Motion::Down);
            } else {
                meb.motion = Some(Motion::Up);
            }

            if semicolon_positions[0] == 4 {
                // Parse single character for button type.
                meb.button = match self[3].to_u8() {
                    0 => Some(MouseButton::Left),
                    1 => Some(MouseButton::Middle),
                    2 => Some(MouseButton::Right),
                    _ => return Err("Not supported."),
                }
            } else {
                meb.button = match self[3].to_u8() {
                    3 => match self[4].to_u8() {
                        2 => Some(MouseButton::Left),
                        3 => Some(MouseButton::Middle),
                        4 => Some(MouseButton::Right),
                        _ => return Err("Not supported."),
                    },
                    6 => match self[4].to_u8() {
                        4 => {
                            meb.direction = Some(Direction::Up);
                            Some(MouseButton::Scroll)
                        }
                        5 => {
                            meb.direction = Some(Direction::Down);
                            Some(MouseButton::Scroll)
                        }
                        _ => return Err("Not supported."),
                    },
                    _ => return Err("Not supported."),
                }
            }

            // Parse X-coordinate.
            let x = &self[(semicolon_positions[0] + 1)..semicolon_positions[1]];
            if let Ok(val) = x.iter().collect::<String>().parse() {
                meb.x = Some(val);
            }

            // Parse Y-coordinate.
            let y = &self[(semicolon_positions[1] + 1)..m_position];
            if let Ok(val) = y.iter().collect::<String>().parse() {
                meb.y = Some(val);
            }

            Ok((Modifier::None, meb.build())) // (modifiers not supported yet)
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
