#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate getset;
#[cfg(target_os = "linux")]
extern crate avahi_sys;
#[cfg(target_os = "macos")]
extern crate bonjour_sys;
extern crate clipboard;
extern crate libc;
extern crate tokio;

mod app;
pub mod mdns;

pub use app::*;
