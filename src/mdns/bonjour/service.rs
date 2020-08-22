pub struct MdnsService {
    name: String,
    kind: String,
    port: u16,
}

impl MdnsService {
    pub fn new(name: &str, kind: &str, port: u16) -> Option<Self> {
        Some(Self {
            name: name.to_string(),
            kind: kind.to_string(),
            port,
        })
    }

    pub fn start(&self) {
        todo!()
    }
}
