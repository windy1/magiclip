#[derive(Debug, Builder)]
pub struct ServiceResolution {
    name: String,
    kind: String,
    domain: String,
    host_name: String,
    address: String,
    port: u16,
}

impl ServiceResolution {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn host_name(&self) -> &str {
        &self.host_name
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn is_local(&self) -> bool {
        self.domain == "local"
    }

    pub fn builder() -> ServiceResolutionBuilder {
        ServiceResolutionBuilder::default()
    }
}
