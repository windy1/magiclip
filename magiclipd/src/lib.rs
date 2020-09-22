#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate magiclip_macros;
#[cfg(target_os = "linux")]
extern crate avahi_sys;
#[cfg(target_os = "macos")]
extern crate bonjour_sys;
extern crate clipboard;
extern crate libc;
extern crate tokio;

mod clipboard_server;
mod daemon;
mod daemon_server;

pub mod builder;
pub mod ffi;
pub mod mdns;

pub use clipboard_server::*;
pub use daemon::*;
pub use daemon_server::*;
