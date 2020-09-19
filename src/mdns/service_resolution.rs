pub type ResolverFoundCallback = dyn Fn(ServiceResolution);

#[derive(Debug, Builder, Getters)]
pub struct ServiceResolution {
    name: String,
    kind: String,
    domain: String,
    host_name: String,
    address: String,
    port: u16,
}

impl ServiceResolution {
    pub fn is_local(&self) -> bool {
        self.domain == "local"
    }

    pub fn builder() -> ServiceResolutionBuilder {
        ServiceResolutionBuilder::default()
    }
}
