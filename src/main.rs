extern crate clap;
extern crate tokio;

use clap::Clap;
use magiclip::serv;
use magiclip::service::AvahiMdnsService;
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let opts = Opts::parse();
    let port = opts.port;

    tokio::spawn(async move {
        AvahiMdnsService::new("test", "_magiclip._tcp", port)
            .unwrap()
            .start();
    });

    serv::start(&opts.host, opts.port).await
}

#[derive(Clap)]
#[clap(version = "1.0", author = "Walker Crouse <walkercrouse@hotmail.com>")]
struct Opts {
    #[clap(short, long, default_value = "127.0.0.1")]
    host: String,
    #[clap(short, long, default_value = "1337")]
    port: u16,
}
