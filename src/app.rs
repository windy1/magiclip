use crate::mdns::{AvahiMdnsBrowser, AvahiMdnsService, ServiceResolution};
use crate::serv;
use std::io;

static SERVICE_TYPE: &'static str = "_magiclip._tcp";

pub async fn start() -> Result<(), io::Error> {
    tokio::spawn(async {
        AvahiMdnsService::new("test", SERVICE_TYPE, 42069)
            .unwrap()
            .start();
    });

    tokio::spawn(async {
        AvahiMdnsBrowser::new(SERVICE_TYPE, Box::new(&on_service_discovered))
            .unwrap()
            .start()
    });

    serv::start("0.0.0.0", 42069).await
}

pub fn on_service_discovered(service: ServiceResolution) {
    if service.is_local() {
        return;
    }

    println!("service discovered {:?}", service);
}
