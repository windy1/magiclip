#[cfg_attr(target_os = "linux", path = "avahi/mod.rs")]
#[cfg_attr(target_os = "macos", path = "bonjour/mod.rs")]
mod os;

pub use os::*;
