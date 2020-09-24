use super::DaemonContext;
use anyhow::{Context, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use magiclip_dtos::DaemonPayload;
use std::str;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use zeroconf::ServiceDiscovery;

#[derive(new)]
pub struct DaemonServer {
    host: String,
    port: u16,
    context: Arc<Mutex<DaemonContext>>,
}

impl DaemonServer {
    pub async fn start(&mut self) -> Result<()> {
        debug!("Starting daemon server on: {}:{}", self.host, self.port);

        let mut listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;

        loop {
            let (socket, addr) = listener.accept().await?;
            let context = self.context.clone();

            debug!("New daemon connection: {:?}", addr);

            tokio::spawn(async move { handle_conn(socket, context).await });
        }
    }
}

async fn handle_conn(mut socket: TcpStream, context: Arc<Mutex<DaemonContext>>) -> Result<()> {
    let mut buffer = [0; 1024];

    if socket.read(&mut buffer).await.unwrap() == 0 {
        return Err(anyhow!("no payload specified to DaemonServer"));
    }

    let payload: DaemonPayload = serde_json::from_str(
        str::from_utf8(&buffer)
            .context("could not decode payload")?
            .trim_matches(char::from(0)),
    )
    .context("invalid payload")?;

    debug!("Payload: {:?}", payload);

    match payload {
        DaemonPayload::ListDiscoveredServices => list_discovered_services(socket, context).await,
        DaemonPayload::SetClipboard(contents) => set_clipboard(socket, &contents).await,
    }
}

async fn list_discovered_services(
    socket: TcpStream,
    context: Arc<Mutex<DaemonContext>>,
) -> Result<()> {
    let response = serde_json::to_string(
        &context
            .lock()
            .unwrap()
            .discovered()
            .values()
            .collect::<Vec<&ServiceDiscovery>>(),
    )?;

    write_response(socket, &response)
        .await
        .context("could not write discovered services to socket")
}

async fn set_clipboard(socket: TcpStream, contents: &str) -> Result<()> {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    clipboard.set_contents(contents.to_string()).unwrap();

    write_response(socket, "OK")
        .await
        .context("could not write response to socket")
}

async fn write_response(mut socket: TcpStream, response: &str) -> Result<()> {
    debug!("Response: {:?}", response);
    socket.write(response.as_bytes()).await?;
    Ok(())
}
