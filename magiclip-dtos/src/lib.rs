#[macro_use]
extern crate serde;

#[derive(Serialize, Deserialize, Debug)]
pub enum DaemonPayload {
    ListDiscoveredServices,
    SetClipboard(String),
}

pub mod net {
    use std::str::{self, Utf8Error};

    pub fn decode_buffer<'a>(buffer: &'a [u8]) -> Result<&'a str, Utf8Error> {
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
