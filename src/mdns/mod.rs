#[cfg_attr(target_os = "linux", path = "avahi/mod.rs")]
#[cfg_attr(target_os = "macos", path = "bonjour/mod.rs")]
mod os;
mod service_resolution;

pub use os::*;
pub use service_resolution::*;
