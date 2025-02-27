use crate::{misc, Event};
use util::{Ansi, CharArray};

mod util {
    use crate::{mouse, platform::unix::ToU8, Modifier, misc};

    pub struct CharArray<const SIZE: usize, F: Fn(bool) -> Result<char, &'static str>> {
        array: [char; SIZE],
        pos: usize,
        reader: F,
    }

    impl<const SIZE: usize, F: Fn(bool) -> Result<char, &'static str>> CharArray<SIZE, F> {
        pub fn new(reader: F) -> Self {
            Self {
                array: [Default::default(); SIZE],
                pos: 0,
                reader,
            }
        }

        pub fn read(&mut self, blocking: bool) -> Result<char, &'static str> {
            if self.pos > SIZE {
                return Err("Array is full.");
            }

            if let Ok(c) = (self.reader)(blocking) {
                self.array[self.pos] = c;
                self.pos += 1;
                return Ok(c);
            }

            Err("Failed to read character.")
        }

        pub fn last(&self) -> Result<char, &'static str> {
            if self.pos == 0 {
                return Err("No character read yet.");
            }

            Ok(self.array[self.pos - 1])
        }
    }

    impl<const SIZE: usize, Idx, F: Fn(bool) -> Result<char, &'static str>> std::ops::Index<Idx>
        for CharArray<SIZE, F>
    where
        Idx: std::slice::SliceIndex<[char]>,
    {
        type Output = Idx::Output;

        fn index(&self, index: Idx) -> &Self::Output {
            &self.array[index]
        }
    }

    pub trait Ansi {
        fn is_escape(&mut self) -> Result<bool, &'static str>;
        fn is_sequence(&mut self) -> Result<bool, &'static str>;
        fn is_mouse_tracking(&mut self) -> Result<(Modifier, mouse::Event), &'static str>;
        fn is_resize(&mut self) -> Result<misc::CurrentSize, &'static str>;
    }

    impl<const SIZE: usize, F: Fn(bool) -> Result<char, &'static str>> Ansi for CharArray<SIZE, F> {
        fn is_escape(&mut self) -> Result<bool, &'static str> {
            if self.pos != 1 {
                return Err("Not in correct state to check for escape sequence.");
            }

            if let Ok(c) = self.last() {
                return Ok(c == '\x1b');
            }

            Err("Failed to parse.")
        }

        fn is_sequence(&mut self) -> Result<bool, &'static str> {
            if self.pos != 2 {
                return Err("Not in correct state to check for sequence start symbol ('[').");
            }

            if let Ok(c) = self.last() {
                return Ok(c == '[');
            }

            Err("Failed to parse.")
        }

        fn is_mouse_tracking(&mut self) -> Result<(Modifier, mouse::Event), &'static str> {
            if self.pos != 3 {
                return Err("Not in correct state to check for mouse tracking sequence.");
            }

            if let Ok(c) = self.last() {
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
            while match self.read(true) {
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
            // These are just some examples of what is parsed. Note that 0 is the CHARACTER for 0. Not the integer value 0.
            // None + 00110000 = Click left (0)
            // None + 00110001 = Click middle (1)
            // None + 00110010 = Click right (2)
            // 00110011 + 00110000 = Move left (32)
            // 00110011 + 00110001 = Move middle (33)
            // 00110011 + 00110010 = Move right (34)
            // 00110110 + 00110100 = Scroll up (64)
            // 00110110 + 00110101 = Scroll down (65)
            let mut meb = mouse::EventBuilder::default();

            if self[m_position] == 'M' {
                meb.motion = Some(mouse::Motion::Down);
            } else {
                meb.motion = Some(mouse::Motion::Up);
            }

            let button_type: usize = if let Ok(button_type) = self[3..semicolon_positions[0]].iter().collect::<String>().parse() {
                button_type
            } else {
                return Err("Badly formatted sequence.");
            };

            // let modifier = Modifier::None;
            let modifier = match button_type % 32 {
                bt if bt < 0+3 => Modifier::None,
                bt if bt < 8+3 => Modifier::Alt,
                bt if bt < 16+3 => Modifier::Ctrl,
                _ => Modifier::None,
            };

            // This is ugly :)
            if button_type < 64 {
                // Either MB 1,2, or 3 (probably).
                meb.button = match button_type % 8 {
                    0 => Some(mouse::Button::Left),
                    1 => Some(mouse::Button::Middle),
                    2 => Some(mouse::Button::Right),
                    _ => return Err("Not supported."),
                }
            } else {
                // Scroll
                meb.button = Some(mouse::Button::Scroll);
                meb.direction = match (button_type - 64) % 8 {
                    0 => Some(mouse::Direction::Up),
                    1 => Some(mouse::Direction::Down),
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


            Ok((modifier, meb.build())) // (modifiers not supported yet)
        }

        fn is_resize(&mut self) -> Result<misc::CurrentSize, &'static str> {
            if self.pos != 3 {
                return Err("Not in correct state to check for sequence start symbol ('[').");
            }

            if let Ok(c) = self.last() {
                if c != '8' {
                    return Err("Not a resize sequence.")
                }
            }

            let mut semicolons_left: usize = 2;
            let mut semicolon_positions: [usize; 2] = [0; 2];
            let mut t_position: usize = 0;
            while match self.read(true) {
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
                        if c == 't' {
                            t_position = self.pos - 1;
                            false
                        } else {
                            true
                        }
                    }
                }
                Err(_) => false,
            } {}

            let mut width = 0;
            let mut height = 0;

            // Parse height.
            let x = &self[(semicolon_positions[0] + 1)..semicolon_positions[1]];
            if let Ok(val) = x.iter().collect::<String>().parse() {
                height = val;
            }

            // Parse width.
            let y = &self[(semicolon_positions[1] + 1)..t_position];
            if let Ok(val) = y.iter().collect::<String>().parse() {
                width = val;
            }

            Ok(misc::CurrentSize(width, height))
        }
    }
}

pub fn interpret<F: Fn(bool) -> Result<char, &'static str>>(
    reader: F,
) -> Result<Event, &'static str> {
    let mut chars = CharArray::<64, F>::new(reader);

    if let Err(msg) = chars.read(false) {
        return Err(msg);
    }

    match chars.is_escape() {
        Ok(is_escape) => {
            if !is_escape {
                // Not an escape sequence. Just a character.
                return Ok(Event::Character(chars.last().unwrap()));
            }

            if let Err(msg) = chars.read(true) {
                return Err(msg);
            }

            match chars.is_sequence() {
                Ok(is_sequence) => {
                    if is_sequence {
                        if let Err(msg) = chars.read(true) {
                            return Err(msg);
                        }

                        if let Ok((modifier, event)) = chars.is_mouse_tracking() {
                            return Ok(Event::Mouse(modifier, event));
                        }

                        if let Ok(current_size) = chars.is_resize() {
                            return Ok(Event::Misc(misc::Event::CurrentSize(current_size)));
                        }
                    }

                    return Err("Unsupported escape format.");
                }
                Err(msg) => return Err(msg),
            }
        }
        Err(msg) => return Err(msg),
    }
}
