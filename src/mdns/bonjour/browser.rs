use crate::mdns::ServiceResolution;

pub struct MdnsBrowser {
    resolver_found_callback: Box<dyn Fn(ServiceResolution)>,
}

impl MdnsBrowser {
    pub fn new(
        kind: &str,
        resolver_found_callback: Box<dyn Fn(ServiceResolution)>,
    ) -> Option<Self> {
        Some(Self {
            resolver_found_callback,
        })
    }

    pub fn start(&mut self) {
        todo!()
    }
}
