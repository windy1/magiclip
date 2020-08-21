// #[macro_use]
// extern crate derive_new;
#[macro_use]
extern crate derive_builder;
extern crate avahi_sys;
extern crate core_foundation;
extern crate clipboard;
extern crate libc;
extern crate tokio;

pub mod app;
pub mod mdns;
pub mod serv;
