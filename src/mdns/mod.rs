#[cfg(target_os = "linux")]
mod avahi;

#[cfg(target_os = "linux")]
pub use avahi::*;
