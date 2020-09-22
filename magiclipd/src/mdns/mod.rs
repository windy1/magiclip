mod discovery;
#[cfg_attr(target_os = "linux", path = "avahi/mod.rs")]
#[cfg_attr(target_os = "macos", path = "bonjour/mod.rs")]
mod os;
mod registration;

pub use discovery::*;
pub use os::*;
pub use registration::*;
