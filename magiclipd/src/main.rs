extern crate tokio;

use anyhow::Result;
use magiclipd::Daemon;

#[tokio::main]
async fn main() -> Result<()> {
    Daemon::default().start().await
}
