pub mod platform;

/// [IO] for managing input and output.
pub trait IOTrait {
    fn read_input(&mut self) -> Key;
}

pub enum Key {
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
}

impl Key {
    pub fn as_str(&self) -> &'static str {
        match self {
            Key::KeyUp => "Arrow up",
            Key::KeyDown => "Arrow down",
            Key::KeyLeft => "Arrow left",
            Key::KeyRight => "Arrow right",
            _ => unimplemented!("as_str() for enum variant is not implemented!"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() -> std::io::Result<()> {
        Ok(())
    }
}