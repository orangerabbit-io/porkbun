use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub domain: String,
    pub status: String,
    pub tld: String,
    pub create_date: String,
    pub expire_date: String,
    pub security_lock: String,
    pub whois_privacy: String,
    pub auto_renew: String,
    pub not_local: i64,
    #[serde(default)]
    pub labels: Option<Vec<Label>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    pub id: String,
    pub title: String,
    pub color: String,
}

#[derive(Debug, Tabled)]
pub struct DomainRow {
    #[tabled(rename = "DOMAIN")]
    pub domain: String,
    #[tabled(rename = "STATUS")]
    pub status: String,
    #[tabled(rename = "TLD")]
    pub tld: String,
    #[tabled(rename = "EXPIRES")]
    pub expire_date: String,
    #[tabled(rename = "AUTO_RENEW")]
    pub auto_renew: String,
}

impl From<&Domain> for DomainRow {
    fn from(d: &Domain) -> Self {
        DomainRow {
            domain: d.domain.clone(),
            status: d.status.clone(),
            tld: d.tld.clone(),
            expire_date: d.expire_date.clone(),
            auto_renew: d.auto_renew.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainAvailability {
    pub avail: bool,
    #[serde(rename = "type")]
    pub avail_type: String,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub first_year_promo: Option<String>,
    #[serde(default)]
    pub regular_price: Option<String>,
    #[serde(default)]
    pub premium: Option<bool>,
    #[serde(default)]
    pub additional: Option<String>,
    #[serde(default)]
    pub min_duration: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainCreate {
    pub domain: String,
    pub cost: String,
    pub order_id: i64,
    pub balance: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_domain() {
        let json = r#"{
            "domain": "example.com",
            "status": "ACTIVE",
            "tld": "com",
            "createDate": "2024-01-01 00:00:00",
            "expireDate": "2025-01-01 00:00:00",
            "securityLock": "1",
            "whoisPrivacy": "1",
            "autoRenew": "1",
            "notLocal": 0
        }"#;
        let domain: Domain = serde_json::from_str(json).unwrap();
        assert_eq!(domain.domain, "example.com");
        assert_eq!(domain.status, "ACTIVE");
        assert!(domain.labels.is_none());
    }

    #[test]
    fn test_deserialize_domain_with_labels() {
        let json = r##"{
            "domain": "example.com",
            "status": "ACTIVE",
            "tld": "com",
            "createDate": "2024-01-01 00:00:00",
            "expireDate": "2025-01-01 00:00:00",
            "securityLock": "1",
            "whoisPrivacy": "1",
            "autoRenew": "1",
            "notLocal": 0,
            "labels": [{"id": "27240", "title": "cool", "color": "#ff9e9e"}]
        }"##;
        let domain: Domain = serde_json::from_str(json).unwrap();
        let labels = domain.labels.unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].title, "cool");
    }

    #[test]
    fn test_deserialize_domain_availability() {
        let json = r#"{
            "avail": true,
            "type": "available",
            "price": "9.73",
            "regularPrice": "10.73",
            "premium": false,
            "minDuration": 1
        }"#;
        let da: DomainAvailability = serde_json::from_str(json).unwrap();
        assert!(da.avail);
        assert_eq!(da.avail_type, "available");
        assert_eq!(da.regular_price, Some("10.73".to_string()));
    }
}
