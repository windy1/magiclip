extern crate tokio;

use magiclip::App;
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    App::default().start().await
}