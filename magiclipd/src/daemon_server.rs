use super::DaemonContext;
use anyhow::{Context, Result};
use magiclip_dtos::{net, DaemonPayload};
use std::str;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(new)]
pub struct DaemonServer {
    host: String,
    port: u16,
    context: Arc<Mutex<DaemonContext>>,
}

impl DaemonServer {
    pub async fn start(&mut self) -> Result<()> {
        let addr = format!("{}:{}", self.host, self.port);

        debug!("Starting daemon server on: {}", addr);

        let mut listener = TcpListener::bind(&addr)
            .await
            .context(format!("could bind DaemonServer to: `{}`", addr))?;

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

    let len = socket
        .read(&mut buffer)
        .await
        .context("could not read from socket")?;

    if len == 0 {
        return Err(anyhow!("no payload specified to DaemonServer"));
    }

    let payload: DaemonPayload =
        serde_json::from_str(net::decode_buffer(&buffer).context("could not decode payload")?)
            .context("invalid payload")?;

    debug!("Payload: {:?}", payload);

    match payload {
        DaemonPayload::ListDiscoveredServices => list_discovered_services(socket, context).await,
    }
}

async fn list_discovered_services(
    socket: TcpStream,
    context: Arc<Mutex<DaemonContext>>,
) -> Result<()> {
    let response = serde_json::to_string(&context.lock().unwrap().discovered())?;

    write_response(socket, &response)
        .await
        .context("could not write discovered services to socket")
}

async fn write_response(mut socket: TcpStream, response: &str) -> Result<()> {
    debug!("Response: {:?}", response);
    socket.write(response.as_bytes()).await?;
    Ok(())
}
