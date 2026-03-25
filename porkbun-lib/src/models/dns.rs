use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecord {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub content: String,
    pub ttl: String,
    #[serde(default)]
    pub prio: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct DnsRecordRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "TYPE")]
    pub record_type: String,
    #[tabled(rename = "CONTENT")]
    pub content: String,
    #[tabled(rename = "TTL")]
    pub ttl: String,
    #[tabled(rename = "PRIO")]
    pub prio: String,
}

impl From<&DnsRecord> for DnsRecordRow {
    fn from(r: &DnsRecord) -> Self {
        DnsRecordRow {
            id: r.id.clone(),
            name: r.name.clone(),
            record_type: r.record_type.clone(),
            content: r.content.clone(),
            ttl: r.ttl.clone(),
            prio: r.prio.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_dns_record() {
        let json = r#"{
            "id": "326953021",
            "name": "www.example.com",
            "type": "A",
            "content": "1.2.3.4",
            "ttl": "600",
            "prio": "0",
            "notes": "test record"
        }"#;
        let record: DnsRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "326953021");
        assert_eq!(record.record_type, "A");
        assert_eq!(record.content, "1.2.3.4");
        assert_eq!(record.notes, Some("test record".to_string()));
    }

    #[test]
    fn test_dns_record_row() {
        let record = DnsRecord {
            id: "1".to_string(),
            name: "test.example.com".to_string(),
            record_type: "CNAME".to_string(),
            content: "other.example.com".to_string(),
            ttl: "3600".to_string(),
            prio: None,
            notes: None,
        };
        let row = DnsRecordRow::from(&record);
        assert_eq!(row.prio, "-");
    }
}
