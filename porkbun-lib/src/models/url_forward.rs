use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlForward {
    pub id: String,
    pub subdomain: String,
    pub location: String,
    #[serde(rename = "type")]
    pub forward_type: String,
    pub include_path: String,
    pub wildcard: String,
}

#[derive(Debug, Tabled)]
pub struct UrlForwardRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "SUBDOMAIN")]
    pub subdomain: String,
    #[tabled(rename = "LOCATION")]
    pub location: String,
    #[tabled(rename = "TYPE")]
    pub forward_type: String,
    #[tabled(rename = "INCLUDE_PATH")]
    pub include_path: String,
}

impl From<&UrlForward> for UrlForwardRow {
    fn from(f: &UrlForward) -> Self {
        UrlForwardRow {
            id: f.id.clone(),
            subdomain: f.subdomain.clone(),
            location: f.location.clone(),
            forward_type: f.forward_type.clone(),
            include_path: f.include_path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_url_forward() {
        let json = r#"{
            "id": "12345",
            "subdomain": "",
            "location": "https://example.com",
            "type": "temporary",
            "includePath": "yes",
            "wildcard": "no"
        }"#;
        let fwd: UrlForward = serde_json::from_str(json).unwrap();
        assert_eq!(fwd.id, "12345");
        assert_eq!(fwd.forward_type, "temporary");
        assert_eq!(fwd.include_path, "yes");
    }
}
