#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/unix_x11_bindings.rs"));

use crate::*;

pub struct IO {
    initialized: bool,
}

impl IO {
    fn initialize(&mut self) {
        self.initialized = true;
    }
}

impl IOTrait for IO {
    fn read_input(&mut self) -> Key {
        Key::KeyDown
        // if !self.initialized {

        // }
    }
}

#[cfg(test)]
mod tests {
}
