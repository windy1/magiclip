use super::AppServer;
use crate::mdns::{MdnsBrowser, MdnsService, ServiceResolution};
use std::io;

static SERVICE_TYPE: &'static str = "_magiclip._tcp";
static SERVICE_NAME: &'static str = "magiclip";
static PORT: u16 = 6060;

#[derive(new)]
pub struct App {}

impl App {
    pub async fn start(&mut self) -> Result<(), io::Error> {
        tokio::spawn(start_service());
        tokio::spawn(start_browser());
        AppServer::new("0.0.0.0", PORT).start().await
    }
}

async fn start_service() {
    let mut service = MdnsService::new(SERVICE_TYPE, PORT);

    #[cfg(target_os = "linux")]
    service.set_name(SERVICE_NAME);

    service.start().unwrap();
}

async fn start_browser() {
    MdnsBrowser::new(SERVICE_TYPE, Box::new(&on_service_discovered))
        .start()
        .unwrap();
}

fn on_service_discovered(service: ServiceResolution) {
    if service.is_local() {
        return;
    }

    println!("service discovered {:?}", service);
}
