#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate anyhow;

mod clipboard_server;
mod daemon;
mod daemon_server;

pub use clipboard_server::*;
pub use daemon::*;
pub use daemon_server::*;

pub(crate) mod env {
    pub fn var(k: &str) -> String {
        std::env::var(k).unwrap_or_else(|_| "<none>".to_string())
    }
}
