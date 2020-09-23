use std::any::Any;
use std::sync::{Arc, Mutex};
use zeroconf::{MdnsService, ServiceRegistration};

#[derive(Default, Debug)]
pub struct Context {
    service_name: String,
}

fn main() {
    let mut service = MdnsService::new("_http._tcp", 8080);
    let context: Arc<Mutex<Context>> = Arc::default();

    service.set_registered_callback(Box::new(on_service_registered));
    service.set_context(Box::new(context));

    // blocks current thread, must keep-alive to keep service active
    service.start().unwrap();
}

fn on_service_registered(service: ServiceRegistration, context: Option<Arc<dyn Any>>) {
    println!("Service registered: {:?}", service);

    let context = context
        .as_ref()
        .unwrap()
        .downcast_ref::<Arc<Mutex<Context>>>()
        .unwrap()
        .clone();

    context.lock().unwrap().service_name = service.name().clone();

    println!("Context: {:?}", context);

    // ...
}
