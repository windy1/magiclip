#[cfg(target_os = "linux")]
use crate::mdns::{MdnsBrowser, MdnsService, ServiceResolution};
use crate::serv;
use std::io;

static SERVICE_TYPE: &'static str = "_magiclip._tcp";

#[cfg(target_os = "linux")]
pub async fn start() -> Result<(), io::Error> {
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

    serv::start("0.0.0.0", 42069).await
}

#[cfg(target_os = "macos")]
pub async fn start() -> Result<(), io::Error> {
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn on_service_discovered(service: ServiceResolution) {
    if service.is_local() {
        return;
    }

    println!("service discovered {:?}", service);
}
