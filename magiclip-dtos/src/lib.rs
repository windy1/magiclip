#[macro_use]
extern crate serde;

#[derive(Serialize, Deserialize, Debug)]
pub enum DaemonPayload {
    ListDiscoveredServices,
    SetClipboard(String),
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
