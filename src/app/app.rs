use super::AppServer;
use crate::mdns::{MdnsBrowser, MdnsService, ServiceRegistration, ServiceResolution};
use std::io;
use std::thread;

static SERVICE_TYPE: &'static str = "_magiclip._tcp";
#[cfg(target_os = "linux")]
static SERVICE_NAME: &'static str = "magiclip";
static PORT: u16 = 6060;

#[derive(Default)]
pub struct App {}

impl App {
    pub async fn start(&mut self) -> Result<(), io::Error> {
        println!("App#start()\n");
        tokio::spawn(start_service());
        AppServer::new("0.0.0.0", PORT).start().await
    }
}

async fn start_service() {
    let mut service = MdnsService::new(SERVICE_TYPE, PORT);

    #[cfg(target_os = "linux")]
    service.set_name(SERVICE_NAME);

    #[cfg(target_os = "macos")]
    service.set_registered_callback(Box::new(on_service_registered));

    service.start().unwrap();
}

#[cfg(target_os = "macos")]
fn on_service_registered(service: ServiceRegistration) {
    println!("on_service_registered()");
    println!("service = {:?}\n", service);
    thread::spawn(|| start_browser());
}

fn start_browser() {
    println!("start_browser()\n");
    let mut browser = MdnsBrowser::new(SERVICE_TYPE);
    browser.set_resolver_found_callback(Box::new(on_service_discovered));
    browser.start().unwrap()
}

fn on_service_discovered(service: ServiceResolution) {
    println!("on_service_discovered()");
    println!("service = {:?}\n", service);
}
