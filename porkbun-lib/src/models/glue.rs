use serde::Serialize;
use tabled::Tabled;

#[derive(Debug, Serialize)]
pub struct GlueRecord {
    pub hostname: String,
    pub v4: Vec<String>,
    pub v6: Vec<String>,
}

#[derive(Debug, Tabled)]
pub struct GlueRecordRow {
    #[tabled(rename = "HOSTNAME")]
    pub hostname: String,
    #[tabled(rename = "IPV4")]
    pub v4: String,
    #[tabled(rename = "IPV6")]
    pub v6: String,
}

impl From<&GlueRecord> for GlueRecordRow {
    fn from(g: &GlueRecord) -> Self {
        GlueRecordRow {
            hostname: g.hostname.clone(),
            v4: g.v4.join(", "),
            v6: g.v6.join(", "),
        }
    }
}

/// Porkbun returns glue records as: [["hostname", {"v4": [...], "v6": [...]}], ...]
/// This function deserializes that tuple array into Vec<GlueRecord>.
pub fn deserialize_glue_hosts(value: &serde_json::Value) -> Result<Vec<GlueRecord>, String> {
    let arr = value
        .as_array()
        .ok_or("Expected array for hosts")?;

    let mut records = Vec::new();
    for entry in arr {
        let tuple = entry
            .as_array()
            .ok_or("Expected [hostname, {v4, v6}] tuple")?;
        if tuple.len() != 2 {
            return Err("Glue record tuple must have exactly 2 elements".to_string());
        }
        let hostname = tuple[0]
            .as_str()
            .ok_or("Expected string hostname")?
            .to_string();
        let ips = &tuple[1];
        let v4 = ips
            .get("v4")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let v6 = ips
            .get("v6")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        records.push(GlueRecord { hostname, v4, v6 });
    }
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_glue_hosts() {
        let json = serde_json::json!([
            ["ns1.example.com", {"v4": ["1.2.3.4"], "v6": ["::1"]}],
            ["ns2.example.com", {"v4": ["5.6.7.8"], "v6": []}]
        ]);
        let records = deserialize_glue_hosts(&json).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].hostname, "ns1.example.com");
        assert_eq!(records[0].v4, vec!["1.2.3.4"]);
        assert_eq!(records[0].v6, vec!["::1"]);
        assert_eq!(records[1].v6.len(), 0);
    }

    #[test]
    fn test_glue_record_row() {
        let record = GlueRecord {
            hostname: "ns1.example.com".to_string(),
            v4: vec!["1.2.3.4".to_string(), "5.6.7.8".to_string()],
            v6: vec![],
        };
        let row = GlueRecordRow::from(&record);
        assert_eq!(row.v4, "1.2.3.4, 5.6.7.8");
        assert_eq!(row.v6, "");
    }
}
