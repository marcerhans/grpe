#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/io_bindings.rs"));

use std::ffi::c_char;

/// Trait to convert a [c_char] to a [char].
pub trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for c_char {
    fn to_char(&self) -> char {
        *self as u8 as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        unsafe {
            setExitHandler();
            enablePartialRawMode();

            let mut buf: i8 = 0;

            loop {
                let success = getNextChar(&mut buf as *mut i8);
            }
        }
    }
}