use anyhow::Result;
use magiclip::DaemonClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = DaemonClient::new("127.0.0.1", 6061)?;

    let discovered_services = client.fetch_discovered_services().await?;

    println!("{:?}", discovered_services);

    Ok(())
}
