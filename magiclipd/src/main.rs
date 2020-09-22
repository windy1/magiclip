extern crate tokio;

use magiclipd::Daemon;
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    Daemon::default().start().await
}
