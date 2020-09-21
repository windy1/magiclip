use super::MagiclipServer;
use crate::mdns::{MdnsBrowser, MdnsService, ServiceRegistration, ServiceResolution};
use std::io;
use std::thread;

static SERVICE_TYPE: &'static str = "_magiclip._tcp";
static PORT: u16 = 6060;

#[derive(Default)]
pub struct Magiclip {}

impl Magiclip {
    pub async fn start(&mut self) -> Result<(), io::Error> {
        env_logger::init();
        tokio::spawn(start_service());
        MagiclipServer::new("0.0.0.0", PORT).start().await
    }
}

async fn start_service() {
    let mut service = MdnsService::new(SERVICE_TYPE, PORT);
    service.set_registered_callback(Box::new(on_service_registered));
    service.start().unwrap();
}

fn on_service_registered(service: ServiceRegistration) {
    debug!("Service registered: {:?}", service);
    thread::spawn(move || start_browser(service.name().clone()));
}

fn start_browser(name: String) {
    let mut browser = MdnsBrowser::new(SERVICE_TYPE);
    browser.set_resolver_found_callback(Box::new(move |s| on_service_discovered(&name, s)));
    browser.start().unwrap()
}

fn on_service_discovered(name: &str, service: ServiceResolution) {
    if name == service.name() {
        debug!("Ignoring {:?}", name);
        return;
    }

    debug!("Service discovered: {:?}", service);
}
