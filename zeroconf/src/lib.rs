#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate magiclip_macros;
#[cfg(target_os = "linux")]
extern crate avahi_sys;
#[cfg(target_os = "macos")]
extern crate bonjour_sys;
#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate log;
extern crate libc;

mod discovery;
#[cfg_attr(target_os = "linux", path = "avahi/mod.rs")]
#[cfg_attr(target_os = "macos", path = "bonjour/mod.rs")]
mod os;
mod registration;

pub mod builder;
pub mod ffi;

pub use discovery::*;
pub use os::*;
pub use registration::*;
