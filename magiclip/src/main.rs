use anyhow::Result;
use clap::Clap;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opts = Opts::parse();

    match opts.operation {
        Some(operation) => match operation {
            Operation::Push => magiclip::ops::push_clipboard(),
        },
        None => magiclip::ops::fetch_clipboard().await,
    }
}

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Walker Crouse <walkercrouse@hotmail.com>")]
struct Opts {
    #[clap(subcommand)]
    operation: Option<Operation>,
}

#[derive(Clap)]
enum Operation {
    Push,
}
