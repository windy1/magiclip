extern crate tokio;

use magiclip::App;
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    #[cfg(target_os = "linux")]
    println!("linux");
    #[cfg(target_os = "unix")]
    println!("unix");
    App::new().start().await
}
