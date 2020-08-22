use crate::mdns::{MdnsBrowser, MdnsService, ServiceResolution};
use super::AppServer;
use std::io;

static SERVICE_TYPE: &'static str = "_magiclip._tcp";

#[derive(new)]
pub struct App {}

impl App {
    pub async fn start(&mut self) -> Result<(), io::Error> {
        tokio::spawn(async {
            MdnsService::new("test", SERVICE_TYPE, 42069)
                .unwrap()
                .start();
        });

        tokio::spawn(async {
            MdnsBrowser::new(SERVICE_TYPE, Box::new(&on_service_discovered))
                .unwrap()
                .start()
        });

        AppServer::new("0.0.0.0", 42069).start().await
    }
}

pub fn on_service_discovered(service: ServiceResolution) {
    if service.is_local() {
        return;
    }

    println!("service discovered {:?}", service);
}
