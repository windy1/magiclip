use std::any::Any;

pub type ServiceRegisteredCallback = dyn Fn(ServiceRegistration, Option<Box<dyn Any>>);

#[derive(Builder, BuilderDelegate, Debug, Getters)]
pub struct ServiceRegistration {
    name: String,
    kind: String,
    domain: String,
}
