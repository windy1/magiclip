use std::any::Any;
use std::sync::Arc;

pub type ServiceRegisteredCallback = dyn Fn(ServiceRegistration, Option<Arc<dyn Any>>);

#[derive(Builder, BuilderDelegate, Debug, Getters)]
pub struct ServiceRegistration {
    name: String,
    kind: String,
    domain: String,
}
