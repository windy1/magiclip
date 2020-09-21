pub type ServiceRegisteredCallback = dyn Fn(ServiceRegistration);

#[derive(Builder, Debug, Getters, BuilderDelegate)]
pub struct ServiceRegistration {
    name: String,
    kind: String,
    domain: String,
}
