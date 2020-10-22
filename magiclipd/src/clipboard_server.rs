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
                let display = crate::env::var("DISPLAY");
                let user = crate::env::var("USER");

                debug!("USER={:?}", user);
                debug!("DISPLAY={:?}", display);

                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                let contents = clipboard.get_contents().unwrap_or_default();

                debug!("Sending: `{:?}`", contents);
                socket.write(contents.as_bytes()).await
            });
        }
    }
}
