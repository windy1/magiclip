use std::any::Any;
use std::sync::Arc;

pub type ServiceDiscoveredCallback = dyn Fn(ServiceDiscovery, Option<Arc<dyn Any>>);

#[derive(Debug, Builder, BuilderDelegate, Getters, Serialize, Deserialize)]
pub struct ServiceDiscovery {
    name: String,
    kind: String,
    domain: String,
    host_name: String,
    address: String,
    port: u16,
}
