use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DnssecRecord {
    pub key_tag: String,
    pub alg: String,
    pub digest_type: String,
    pub digest: String,
}

#[derive(Debug, Tabled)]
pub struct DnssecRecordRow {
    #[tabled(rename = "KEY_TAG")]
    pub key_tag: String,
    #[tabled(rename = "ALG")]
    pub alg: String,
    #[tabled(rename = "DIGEST_TYPE")]
    pub digest_type: String,
    #[tabled(rename = "DIGEST")]
    pub digest: String,
}

impl From<&DnssecRecord> for DnssecRecordRow {
    fn from(r: &DnssecRecord) -> Self {
        DnssecRecordRow {
            key_tag: r.key_tag.clone(),
            alg: r.alg.clone(),
            digest_type: r.digest_type.clone(),
            digest: r.digest.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_deserialize_dnssec_map() {
        let json = r#"{
            "12345": {"keyTag": "12345", "alg": "13", "digestType": "2", "digest": "abc123"}
        }"#;
        let records: HashMap<String, DnssecRecord> = serde_json::from_str(json).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records["12345"].alg, "13");
    }
}
