extern crate tokio;

use magiclip::serv;
use magiclip::service::AvahiMdnsService;
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    tokio::spawn(async {
        AvahiMdnsService::new("test", "_magiclip._tcp", 42069)
            .unwrap()
            .start();
    });

    serv::start("192.168.0.4", 42069).await
}
