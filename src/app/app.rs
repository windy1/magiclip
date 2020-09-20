use super::AppServer;
use crate::mdns::{MdnsBrowser, MdnsService, ServiceRegistration, ServiceResolution};
use std::io;
use std::sync::Arc;

static SERVICE_TYPE: &'static str = "_magiclip._tcp";
#[cfg(target_os = "linux")]
static SERVICE_NAME: &'static str = "magiclip";
static PORT: u16 = 6060;

#[derive(Default)]
pub struct App {
    context: Arc<AppContext>,
}

impl App {
    pub async fn start(&mut self) -> Result<(), io::Error> {
        println!("App#start()");
        tokio::spawn(start_service());
        tokio::spawn(start_browser());
        println!();
        AppServer::new("0.0.0.0", PORT).start().await
    }
}

#[derive(Default)]
struct AppContext {
    service_name: Option<String>,
}

async fn start_service() {
    let mut service = MdnsService::new(SERVICE_TYPE, PORT);

    #[cfg(target_os = "linux")]
    service.set_name(SERVICE_NAME);

    #[cfg(target_os = "macos")]
    service.set_registered_callback(Box::new(on_service_registered));

    service.start().unwrap();
}

async fn start_browser() {
    let mut browser = MdnsBrowser::new(SERVICE_TYPE);

    browser.set_resolver_found_callback(Box::new(&on_service_discovered));

    browser.start().unwrap()
}

#[cfg(target_os = "macos")]
fn on_service_registered(service: ServiceRegistration) {
    println!("on_service_registered()");
    println!("service = {:?}\n", service);
}

fn on_service_discovered(service: ServiceResolution) {
    println!("on_service_discovered()");
    println!("service = {:?}\n", service);
}
