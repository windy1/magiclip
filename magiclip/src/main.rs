extern crate tokio;

use magiclip::Magiclip;
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    Magiclip::default().start().await
}
