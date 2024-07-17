#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/io_bindings.rs"));

use std::{ffi::c_char, sync::{atomic::{AtomicU8, Ordering}, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}};

use crate::{Event, EventHandlerTrait, Letter};

const HANDLER_COUNT_MAX: u8 = 1;
static HANDLER_COUNT_CURRENT: AtomicU8 = AtomicU8::new(0);

/// Trait to convert a [c_char] to a [char].
pub trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for c_char {
    fn to_char(&self) -> char {
        *self as u8 as char
    }
}

pub struct EventHandler {
    pub receiver: Receiver<Event>,
    io_thread: JoinHandle<()>,
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        unsafe {
            disablePartialRawMode();
        }
    }
}

impl EventHandlerTrait for EventHandler {
    fn init() -> Self {
        if HANDLER_COUNT_CURRENT.fetch_add(1, Ordering::SeqCst) == HANDLER_COUNT_MAX {
            panic!("Only initialize event handler once!");
        }

        unsafe {
            setExitHandler();
            enablePartialRawMode();
        }

        let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();

        // Each thread will send its id via the channel
        Self {
            receiver,
            io_thread: {
                let sender = sender.clone();

                thread::spawn(move || {
                    let mut buf: c_char = 0;
                    let buf_ptr: *mut c_char = &mut buf as *mut c_char;

                    loop {
                        let success = unsafe { getNextChar(buf_ptr) };
                        let _ = sender.send(Event::Letter(Letter(buf.to_char())));
                    }
                })
            },
        }
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