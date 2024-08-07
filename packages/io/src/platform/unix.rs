#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/io_bindings.rs"));

use std::{
    ffi::c_char,
    panic,
    sync::{Arc, Mutex, OnceLock, RwLock},
    thread::{self, JoinHandle},
};

use crate::{
    ansi_interpretor::{ascii, is_csi_sequence, is_escape_sequence, read_csi_sequence},
    Event, EventHandlerTrait,
};

/// Panic hook
static PANIC_HOOK_FLAG: OnceLock<()> = OnceLock::new();

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
        let _ = PANIC_HOOK_FLAG.set({
            let old_hook = panic::take_hook();

            panic::set_hook(Box::new(move |panic_info| {
                unsafe {
                    disablePartialRawMode();
                }
                old_hook(panic_info);
            }));
        });

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
                        if is_escape_sequence(buf) {
                            let success = unsafe { getNextChar(buf_ptr) };

                            if is_csi_sequence(buf) {
                                let event = read_csi_sequence(|| {
                                    let mut buf: c_char = 0;
                                    let buf_ptr: *mut c_char = &mut buf as *mut c_char;

                                    if !(unsafe { getNextChar(buf_ptr) }) { // TODO: getNextChar might need to be update to fulfill its contract/description.
                                        panic!("opsie"); // TODO: Possible to handle? I think there WILL be characters in buffer by this point.
                                    }

                                    buf
                                });

                                if let Some(event) = event {
                                    *io_thread_buf.lock().unwrap() = Some(event);
                                }
                            } else if buf == ascii::ESC {
                                // Since CTRL-C is disabled, have a "always-on" exit.
                                unsafe {
                                    disablePartialRawMode();
                                }
                                println!("{}", "\x1B[2J"); // TODO: This clears the screen before exit, but it should not be HERE.
                                std::process::exit(0);
                            }
                            continue;
                        }

                        if buf == 'P' as i8 {
                            panic!("Manually triggered panic by pressing 'P'!")
                        }

                        *io_thread_buf.lock().unwrap() = Some(Event::Letter(buf.to_char()));
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
    }
}
