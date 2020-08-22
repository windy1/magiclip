#[cfg_attr(target_os = "linux", path = "avahi/mod.rs")]
mod os;

pub use os::*;
