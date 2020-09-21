pub type ResolverFoundCallback = dyn Fn(ServiceResolution);

#[derive(Debug, Builder, BuilderDelegate, Getters)]
pub struct ServiceResolution {
    name: String,
    kind: String,
    domain: String,
    host_name: String,
    address: String,
    port: u16,
}
