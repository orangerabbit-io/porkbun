use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SslBundle {
    pub certificatechain: String,
    pub privatekey: String,
    pub publickey: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_ssl_bundle() {
        let json = r#"{
            "certificatechain": "-----BEGIN CERTIFICATE-----\nMIIE...",
            "privatekey": "-----BEGIN PRIVATE KEY-----\nMIIE...",
            "publickey": "-----BEGIN PUBLIC KEY-----\nMIIB..."
        }"#;
        let ssl: SslBundle = serde_json::from_str(json).unwrap();
        assert!(ssl.certificatechain.starts_with("-----BEGIN"));
    }
}
