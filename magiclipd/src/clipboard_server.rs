use clipboard::{ClipboardContext, ClipboardProvider};
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub struct ClipboardServer {
    host: String,
    port: u16,
}

impl ClipboardServer {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }

    pub async fn start(&mut self) -> io::Result<()> {
        let mut listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;

        loop {
            let (mut socket, addr) = listener.accept().await?;

            debug!("New connection: {}", addr);

            tokio::spawn(async move {
                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                let contents: String = clipboard.get_contents().unwrap();
                socket.write(contents.as_bytes()).await
            });
        }
    }
}
