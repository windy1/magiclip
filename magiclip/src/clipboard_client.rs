use anyhow::{Context, Result};
use magiclip_dtos::net;
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
                net::decode_buffer(&buffer)
                    .context("could not decode clipboard contents")?
                    .to_string(),
            ))
        }
    }
}
