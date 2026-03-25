use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TldPricing {
    pub registration: String,
    pub renewal: String,
    pub transfer: String,
}

#[derive(Debug, Tabled)]
pub struct TldPricingRow {
    #[tabled(rename = "TLD")]
    pub tld: String,
    #[tabled(rename = "REGISTRATION")]
    pub registration: String,
    #[tabled(rename = "RENEWAL")]
    pub renewal: String,
    #[tabled(rename = "TRANSFER")]
    pub transfer: String,
}

impl TldPricingRow {
    pub fn from_entry(tld: &str, pricing: &TldPricing) -> Self {
        TldPricingRow {
            tld: tld.to_string(),
            registration: pricing.registration.clone(),
            renewal: pricing.renewal.clone(),
            transfer: pricing.transfer.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_deserialize_pricing_map() {
        let json = r#"{
            "com": {"registration": "9.73", "renewal": "10.73", "transfer": "9.73"},
            "net": {"registration": "10.73", "renewal": "11.73", "transfer": "10.73"}
        }"#;
        let pricing: HashMap<String, TldPricing> = serde_json::from_str(json).unwrap();
        assert_eq!(pricing.len(), 2);
        assert_eq!(pricing["com"].registration, "9.73");
    }
}
