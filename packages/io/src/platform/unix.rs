#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/io_bindings.rs"));

use std::sync::atomic::AtomicBool;
/// Documentation regarding escape codes and control sequences
/// can be found in the Xterm Control Sequence Documentation.
use std::{ffi::c_char, sync::atomic::Ordering};

use crate::{ansi_interpretor, Event, EventHandlerTrait};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct EventHandler;

impl EventHandlerTrait for EventHandler {
    fn init() -> Result<Self, &'static str> {
        if INITIALIZED.load(Ordering::Relaxed) {
            return Err("Already initialized.");
        }

        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            unsafe {
                terminate();
            }
            prev_hook(info);
        }));

        unsafe {
            initialize();
        }

        INITIALIZED.store(true, Ordering::Relaxed);
        Ok(Self)
    }

    fn latest_event(&self) -> Result<Event, &'static str> {
        return ansi_interpretor::interpret(|blocking: bool| self.latest_character(blocking));
    }

    fn latest_character(&self, blocking: bool) -> Result<char, &'static str> {
        let mut buf: c_char = 0;

        unsafe {
            let buf_p = &mut buf as *mut c_char;
            let result = getChar(buf_p, blocking);

            if result == 1 {
                return Err("Failed to read.");
            } else if result == 2 {
                return Err("Nothing to read.");
            }
        }

        Ok(buf.to_char())
    }

    fn running(&self) -> bool {
        unsafe { running() }
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        unsafe {
            terminate();
        }
        let _ = std::panic::take_hook(); // Remove custom panic hook.
        INITIALIZED.store(false, Ordering::Relaxed);
    }
}

pub trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for c_char {
    fn to_char(&self) -> char {
        *self as u8 as char
    }
}

pub trait ToU8 {
    fn to_u8(&self) -> u8;
}

impl ToU8 for char {
    fn to_u8(&self) -> u8 {
        *self as u8 - 48
    }
}
