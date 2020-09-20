use crate::util::BuilderDelegate;

pub type ServiceRegisteredCallback = dyn Fn(ServiceRegistration);

#[derive(Builder, Debug)]
pub struct ServiceRegistration {
    name: String,
    kind: String,
    domain: String,
}

impl BuilderDelegate<ServiceRegistrationBuilder> for ServiceRegistration {}
