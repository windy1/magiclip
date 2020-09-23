#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate zeroconf_macros;
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
mod registration;

pub mod builder;
pub mod ffi;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;

pub use discovery::*;
pub use registration::*;

#[cfg(target_os = "linux")]
pub use linux::{browser::*, service::*};
#[cfg(target_os = "macos")]
pub use macos::{browser::*, service::*};
