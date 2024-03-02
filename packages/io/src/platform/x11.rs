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
    use super::*;

    #[test]
    fn main() {
        unsafe {
            let display: *mut Display = XOpenDisplay(std::ptr::null());

            if !display.is_null() {
                println!("Display!");

                let window = XCreateWindow(
                    display,
                    XDefaultRootWindow(display),
                    0,
                    0,
                    200,
                    100,
                    0,
                    CopyFromParent as i32,
                    CopyFromParent as u32,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                );

                XMapWindow(display, window);
                XFlush(display);

            }
        }

        std::thread::sleep(std::time::Duration::from_millis(10000));
    }
}
