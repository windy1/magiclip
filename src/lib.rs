#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate derive_builder;
extern crate avahi_sys;
extern crate clipboard;
extern crate core_foundation;
extern crate libc;
extern crate tokio;

mod app;
pub mod mdns;

pub use app::*;
