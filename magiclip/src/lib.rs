// #[macro_use]
// extern crate derive_new;
#[macro_use]
extern crate derive_builder;
// #[macro_use]
// extern crate getset;
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

mod app;
pub mod mdns;
pub mod util;

pub use app::*;
