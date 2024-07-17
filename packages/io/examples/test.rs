use io::platform::unix::*;
use std::ffi::c_char;

fn main() {
    unsafe {
        setExitHandler();
        enablePartialRawMode();
    }

    // println!("\x1B[?1002h"); // Track button presses + movement while pressed.

    let mut buf: c_char = 0;
    let mut buf_ptr: *mut c_char = &mut buf as *mut c_char;

    loop {
        let success = unsafe { getNextChar(buf_ptr) };
        println!("{}", buf.to_char());
    }
}