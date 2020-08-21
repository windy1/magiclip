#[cfg(target_os = "unix")]
mod avahi;

#[cfg(target_os = "unix")]
pub use avahi::*;
