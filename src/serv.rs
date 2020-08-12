use clipboard::{ClipboardContext, ClipboardProvider};
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn start(host: &str, port: u16) -> io::Result<()> {
    let mut listener = TcpListener::bind(format!("{}:{}", host, port)).await?;
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await?;
        let clipboard = ctx.get_contents().unwrap();

        println!("new connection: {}", addr);

        tokio::spawn(async move { socket.write(clipboard.as_bytes()).await });
    }
}
