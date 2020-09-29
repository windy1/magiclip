#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate derive_new;

#[derive(Serialize, Deserialize, Debug)]
pub enum DaemonPayload {
    ListDiscoveredServices,
    SetClipboard(String),
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Getters, Serialize, Deserialize)]
pub struct UniqueService {
    name: String,
    host_name: String,
}

pub mod net {
    use std::str::{self, Utf8Error};

    pub fn decode_buffer(buffer: &[u8]) -> Result<&str, Utf8Error> {
        Ok(str::from_utf8(&buffer)?.trim_matches(char::from(0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemon_payload_serialize() {
        assert_eq!(
            serde_json::to_string(&DaemonPayload::SetClipboard("foobar".to_string())).unwrap(),
            "{\"SetClipboard\":\"foobar\"}"
        );
    }
}
