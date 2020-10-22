use anyhow::{Context, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::{env, fs};
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

                let contents = get_clipboard_contents().unwrap();
                debug!("Sending: `{:?}`", contents);
                socket.write(contents.as_bytes()).await
            });
        }
    }
}

fn get_clipboard_contents() -> Result<String> {
    let clipboard: Option<ClipboardContext> = ClipboardProvider::new().ok();
    match clipboard {
        Some(mut clp) => Ok(clp.get_contents().unwrap_or_else(|_| String::new())),
        None => {
            let fname = format!("{}/.magiclip/clipboard.txt", env::var("HOME").unwrap());
            fs::read_to_string(&fname).context("could not read clipboard file")
        }
    }
}
