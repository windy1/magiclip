use anyhow::{Context, Result};
use magiclip_dtos::net;
use magiclip_dtos::{DaemonPayload, UniqueService};
use std::net::SocketAddr;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct DaemonClient {
    address: SocketAddr,
}

impl DaemonClient {
    pub fn new(host: &str, port: u16) -> Result<Self> {
        Ok(Self {
            address: SocketAddr::new(host.parse().context("could not parse IP address")?, port),
        })
    }

    pub async fn list_discovered_services(&self) -> Result<Vec<UniqueService>> {
        serde_json::from_str(&self.connect(DaemonPayload::ListDiscoveredServices).await?)
            .context("could not deserialize daemon payload")
    }

    pub async fn set_clipboard(&self, contents: &str) -> Result<()> {
        let response = self
            .connect(DaemonPayload::SetClipboard(contents.to_string()))
            .await?;

        match response.as_str() {
            "OK" => Ok(()),
            e => Err(anyhow!("daemon responded with error: `{}`", e)),
        }
    }

    async fn connect(&self, payload: DaemonPayload) -> Result<String> {
        let mut conn = TcpStream::connect(self.address)
            .await
            .context("could not connect to daemon")?;

        conn.write(
            serde_json::to_string(&payload)
                .context("could not encode payload")?
                .as_bytes(),
        )
        .await
        .context("could not write payload to daemon")?;

        self.read_response(&mut conn).await
    }

    async fn read_response(&self, conn: &mut TcpStream) -> Result<String> {
        let mut buffer = [0; 1024];

        if conn.read(&mut buffer).await? == 0 {
            Err(anyhow!("daemon did not respond to request"))
        } else {
            Ok(net::decode_buffer(&buffer)
                .context("could not decode response")?
                .to_string())
        }
    }
}
