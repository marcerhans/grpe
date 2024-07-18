#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/io_bindings.rs"));

use std::{
    ffi::c_char,
    sync::{Arc, Mutex, OnceLock, RwLock},
    thread::{self, JoinHandle},
};

use crate::{Event, EventHandlerTrait};

/// Singleton worker thread.
static IO_THREAD: OnceLock<Arc<Mutex<Option<JoinHandle<()>>>>> = OnceLock::new();

/// Manual reference counting of users for worker io thread.
static IO_THREAD_REFS: OnceLock<Arc<RwLock<isize>>> = OnceLock::new();

/// Buffer for [Event]s.
static IO_THREAD_BUF: OnceLock<Arc<Mutex<Option<Event>>>> = OnceLock::new();

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
    pub event_buf: Arc<Mutex<Option<Event>>>,
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        let mut io_thread = IO_THREAD.get().unwrap().lock().unwrap();
        let mut io_thread_refs = IO_THREAD_REFS.get().unwrap().write().unwrap();

        *io_thread_refs -= 1;

        if *io_thread_refs < 0 {
            panic!();
        }

        if *io_thread_refs == 0 {
            unsafe {
                disablePartialRawMode();
            }
            drop(io_thread_refs);
            io_thread.take().unwrap().join().unwrap();
        }
    }
}

impl EventHandlerTrait for EventHandler {
    fn init() -> Self {
        let io_thread_refs = IO_THREAD_REFS.get_or_init(|| Arc::new(RwLock::new(0)));
        let mut io_thread_refs_lock = io_thread_refs.write().unwrap();

        *io_thread_refs_lock += 1;

        if *io_thread_refs_lock == 1 {
            unsafe {
                // setExitHandler(); // TODO: Does not work currently :(
                enablePartialRawMode();
            }
        }
        drop(io_thread_refs_lock);

        let io_thread_buf = IO_THREAD_BUF.get_or_init(|| Arc::new(Mutex::new(None)));
        let io_thread = IO_THREAD.get_or_init(|| Arc::new(Mutex::new(None)));
        let mut io_thread_lock = io_thread.lock().unwrap();

        if io_thread_lock.is_none() {
            *io_thread_lock = Some(thread::spawn(move || {
                let mut buf: c_char = 0;
                let buf_ptr: *mut c_char = &mut buf as *mut c_char;

                while *IO_THREAD_REFS.get().unwrap().read().unwrap() > 0 {
                    let success = unsafe { getNextChar(buf_ptr) };

                    if success {
                        // Since CTRL-C is disabled, have a "always-on" exit.
                        if buf.to_char() == 'q' {
                            unsafe {
                                disablePartialRawMode();
                            }
                            std::process::exit(0);
                        }

                        *io_thread_buf.lock().unwrap() =
                            Some(Event::Letter(buf.to_char()));
                    }
                }
            }));
        }

        Self {
            event_buf: io_thread_buf.clone(),
        }
    }

    fn get_latest_event(&self) -> Option<Event> {
        self.event_buf.lock().unwrap().take()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use thread::sleep;

    use super::*;

    #[test]
    fn main() {
        let event_handler = EventHandler::init();
        sleep(Duration::from_secs(2));
        let event_handler = EventHandler::init();
    }
}
