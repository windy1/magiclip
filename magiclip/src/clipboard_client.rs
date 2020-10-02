use anyhow::{Context, Result};
use magiclip_dtos::net;
use std::net::{SocketAddr, ToSocketAddrs};
use std::str;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct ClipboardClient {
    address: SocketAddr,
}

impl ClipboardClient {
    pub fn new(host: &str, port: u16) -> Result<Self> {
        Ok(Self {
            address: format!("{}:{}", host, port)
                .as_str()
                .to_socket_addrs()
                .context("could not parse socket address")?
                .next()
                .unwrap(),
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
