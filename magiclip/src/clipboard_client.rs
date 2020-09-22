use anyhow::{Context, Result};
use std::net::SocketAddr;
use std::str;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct ClipboardClient {
    address: SocketAddr,
}

impl ClipboardClient {
    pub fn new(host: &str, port: u16) -> Result<Self> {
        Ok(Self {
            address: SocketAddr::new(host.parse().context("could not parse IP address")?, port),
        })
    }

    pub async fn fetch_clipboard(&self) -> Result<Option<String>> {
        let mut conn = TcpStream::connect(self.address)
            .await
            .context("could not connect to clipboard server")?;

        let mut buffer = [0; 1024];

        if conn.read(&mut buffer).await? == 0 {
            Ok(None)
        } else {
            Ok(Some(
                str::from_utf8(&buffer)
                    .context("could not decode payload")?
                    .trim_matches(char::from(0))
                    .to_string(),
            ))
        }
    }
}
