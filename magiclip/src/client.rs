use anyhow::{Context, Result};
use std::net::SocketAddr;
use std::str;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use zeroconf::ServiceDiscovery;

pub struct DaemonClient {
    address: SocketAddr,
}

impl DaemonClient {
    pub fn new(host: &str, port: u16) -> Result<Self> {
        Ok(Self {
            address: SocketAddr::new(host.parse().context("could not parse IP address")?, port),
        })
    }

    pub async fn fetch_discovered_services(&self) -> Result<Vec<ServiceDiscovery>> {
        let mut conn = TcpStream::connect(self.address)
            .await
            .context("could not connect to daemon")?;

        let mut buffer = [0; 1024];

        if conn.read(&mut buffer).await? == 0 {
            Ok(vec![])
        } else {
            let payload = str::from_utf8(&buffer)
                .context("could not decode payload")?
                .trim_matches(char::from(0));

            Ok(serde_json::from_str(payload).context("could not deserialize daemon payload")?)
        }
    }
}
