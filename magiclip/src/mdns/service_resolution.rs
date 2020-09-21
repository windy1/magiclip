use std::any::Any;
use std::sync::Arc;

pub type ResolverFoundCallback = dyn Fn(ServiceResolution, Option<Arc<dyn Any>>);

#[derive(Debug, Builder, BuilderDelegate, Getters)]
pub struct ServiceResolution {
    name: String,
    kind: String,
    domain: String,
    host_name: String,
    address: String,
    port: u16,
}
