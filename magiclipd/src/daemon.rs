use super::{ClipboardServer, DaemonServer};
use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{any::Any, collections::HashMap, thread};
use zeroconf::prelude::*;
use zeroconf::{MdnsBrowser, MdnsService, ServiceDiscovery, ServiceRegistration};

static SERVICE_TYPE: &str = "_magiclip._tcp";
static CLIPBOARD_HOST: &str = "0.0.0.0";
static CLIPBOARD_PORT: u16 = 6060;
static DAEMON_HOST: &str = "127.0.0.1";
static DAEMON_PORT: u16 = 6061;

#[derive(Default)]
pub struct Daemon {
    context: Arc<Mutex<DaemonContext>>,
}

#[derive(Default, Debug, Getters)]
pub struct DaemonContext {
    service_name: String,
    discovered: HashMap<String, ServiceDiscovery>,
}

impl Daemon {
    pub async fn start(&mut self) -> Result<()> {
        env_logger::init();

        self.context = Arc::default();
        let context = self.context.clone();

        tokio::spawn(async { start_service(context).await });

        tokio::spawn(async {
            ClipboardServer::new(CLIPBOARD_HOST.to_string(), CLIPBOARD_PORT)
                .start()
                .await
        });

        DaemonServer::new(DAEMON_HOST.to_string(), DAEMON_PORT, self.context.clone())
            .start()
            .await
    }
}

async fn start_service(context: Arc<Mutex<DaemonContext>>) {
    let mut service = MdnsService::new(SERVICE_TYPE, CLIPBOARD_PORT);

    service.set_registered_callback(Box::new(on_service_registered));
    service.set_context(Box::new(context));

    let event_loop = service.register().unwrap();

    loop {
        event_loop.poll(Duration::from_secs(0)).unwrap();
    }
}

fn on_service_registered(
    result: zeroconf::Result<ServiceRegistration>,
    context: Option<Arc<dyn Any>>,
) {
    let service = match result {
        Ok(s) => s,
        Err(e) => {
            warn!("on_service_registered(): `{:?}`", e);
            return;
        }
    };

    debug!("Service registered: {:?}", service);

    let context = context
        .as_ref()
        .unwrap()
        .downcast_ref::<Arc<Mutex<DaemonContext>>>()
        .unwrap()
        .clone();

    context.lock().unwrap().service_name = service.name().clone();

    thread::spawn(|| start_browser(Box::new(context)));
}

fn start_browser(context: Box<dyn Any>) {
    let mut browser = MdnsBrowser::new(SERVICE_TYPE);

    browser.set_service_discovered_callback(Box::new(on_service_discovered));
    browser.set_context(context);

    let event_loop = browser.browse_services().unwrap();

    loop {
        event_loop.poll(Duration::from_secs(0)).unwrap();
    }
}

fn on_service_discovered(
    result: zeroconf::Result<ServiceDiscovery>,
    context: Option<Arc<dyn Any>>,
) {
    let service = match result {
        Ok(s) => s,
        Err(e) => {
            warn!("on_service_discovered(): `{:?}`", e);
            return;
        }
    };

    let context_mtx = context
        .unwrap()
        .downcast_ref::<Arc<Mutex<DaemonContext>>>()
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
