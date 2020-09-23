use super::DaemonContext;
use std::io;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use zeroconf::ServiceDiscovery;

#[derive(new)]
pub struct DaemonServer {
    host: String,
    port: u16,
    context: Arc<Mutex<DaemonContext>>,
}

impl DaemonServer {
    pub async fn start(&mut self) -> io::Result<()> {
        debug!("Starting daemon server on: {}:{}", self.host, self.port);

        let mut listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;

        loop {
            let (mut socket, addr) = listener.accept().await?;

            debug!("New daemon connection: {:?}", addr);

            let context = self.context.clone();

            tokio::spawn(async move {
                let response = serde_json::to_string(
                    &context
                        .lock()
                        .unwrap()
                        .discovered()
                        .values()
                        .collect::<Vec<&ServiceDiscovery>>(),
                )
                .unwrap();

                debug!("Response: {:?}", response);

                socket.write(response.as_bytes()).await
            });
        }
    }
}
