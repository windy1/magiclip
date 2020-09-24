use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[derive(new)]
pub struct ClipboardServer {
    host: String,
    port: u16,
}

impl ClipboardServer {
    pub async fn start(&mut self) -> Result<()> {
        debug!("Starting clipboard server on: {}:{}", self.host, self.port);

        let mut listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;

        loop {
            let (mut socket, addr) = listener.accept().await?;

            debug!("New clipboard connection: {:?}", addr);

            tokio::spawn(async move {
                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                let contents: String = clipboard.get_contents().unwrap();
                socket.write(contents.as_bytes()).await
            });
        }
    }
}
