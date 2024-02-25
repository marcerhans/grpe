#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use crate::*;

    #[test]
    fn hello_world() {
        let string = CString::new("Hello world").unwrap();

        unsafe {
            ncurses::initscr();
            ncurses::printw(string.as_ptr());
            ncurses::refresh();
            ncurses::getch();
            ncurses::endwin();
        }
    }
}
