pub type ServiceRegisteredCallback = dyn Fn(ServiceRegistration);

#[derive(Builder, BuilderDelegate, Debug, Getters)]
pub struct ServiceRegistration {
    name: String,
    kind: String,
    domain: String,
}
