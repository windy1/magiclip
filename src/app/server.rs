use clipboard::{ClipboardContext, ClipboardProvider};
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub struct AppServer {
    host: String,
    port: u16,
    clipboard: ClipboardContext,
}

impl AppServer {
    pub fn new(host: &str, port: u16) -> Self {
        Self { host: host.to_string(), port, clipboard: ClipboardProvider::new().unwrap() }
    }

    pub async fn start(&mut self) -> io::Result<()> {
        let mut listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;

        loop {
            let (mut socket, addr) = listener.accept().await?;
            let contents = self.clipboard.get_contents().unwrap();

            println!("new connection: {}", addr);

            tokio::spawn(async move { socket.write(contents.as_bytes()).await });
        }
    }
}
