extern crate tokio;

use magiclip::app;
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    app::start().await
}
