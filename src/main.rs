extern crate tokio;

use magiclip::mdns::AvahiMdnsService;
use magiclip::serv;
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
