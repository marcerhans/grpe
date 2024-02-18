#[macro_export]
macro_rules! log {
        ($($args: tt)*) => {
            print!("[{}:{}] ", file!(), line!());
            println!($($args)*);
        }
    }

#[macro_export]
macro_rules! inactive_log {
    ($($args: tt)*) => {};
}

#[cfg(any(feature = "debug", feature = "all"))]
pub use log as debug;
#[cfg(any(feature="info", feature="all"))]
pub use log as info;
#[cfg(any(feature = "warn", feature = "all"))]
pub use log as warn;
#[cfg(any(feature = "error", feature = "all"))]
pub use log as error;

#[cfg(not(any(feature = "debug", feature = "all")))]
pub use inactive_log as debug;
#[cfg(not(any(feature = "info", feature = "all")))]
pub use inactive_log as info;
#[cfg(not(any(feature = "warn", feature = "all")))]
pub use inactive_log as warn;
#[cfg(not(any(feature = "error", feature = "all")))]
pub use inactive_log as error;
