use super::MagiclipServer;
use crate::mdns::{MdnsBrowser, MdnsService, ServiceRegistration, ServiceResolution};
use std::io;
use std::sync::{Arc, Mutex};
use std::{any::Any, collections::HashMap, thread};

static SERVICE_TYPE: &'static str = "_magiclip._tcp";
static PORT: u16 = 6060;

#[derive(Default)]
pub struct Magiclip {}

#[derive(Default, Debug)]
struct MagiclipContext {
    service_name: String,
    discovered: HashMap<String, ServiceResolution>,
}

impl Magiclip {
    pub async fn start(&mut self) -> Result<(), io::Error> {
        env_logger::init();
        tokio::spawn(start_service(Arc::default()));
        MagiclipServer::new("0.0.0.0", PORT).start().await
    }
}

async fn start_service(context: Arc<Mutex<MagiclipContext>>) {
    let mut service = MdnsService::new(SERVICE_TYPE, PORT);
    service.set_registered_callback(Box::new(on_service_registered));
    service.set_context(Box::new(context));
    service.start().unwrap();
}

fn on_service_registered(service: ServiceRegistration, context: Option<Arc<dyn Any>>) {
    debug!("Service registered: {:?}", service);

    let context = context
        .as_ref()
        .unwrap()
        .downcast_ref::<Arc<Mutex<MagiclipContext>>>()
        .unwrap()
        .clone();

    context.lock().unwrap().service_name = service.name().clone();

    thread::spawn(move || start_browser(Box::new(context)));
}

fn start_browser(context: Box<dyn Any>) {
    let mut browser = MdnsBrowser::new(SERVICE_TYPE);
    browser.set_resolver_found_callback(Box::new(on_service_discovered));
    browser.set_context(context);
    browser.start().unwrap()
}

fn on_service_discovered(service: ServiceResolution, context: Option<Arc<dyn Any>>) {
    let context_mtx = context
        .unwrap()
        .downcast_ref::<Arc<Mutex<MagiclipContext>>>()
        .unwrap()
        .clone();

    let mut context = context_mtx.lock().unwrap();

    if &context.service_name == service.name() {
        debug!("Ignoring {:?}", context.service_name);
        return;
    }

    debug!("Service discovered: {:?}", &service);

    context.discovered.insert(service.name().clone(), service);
}
