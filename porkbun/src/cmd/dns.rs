use crate::output::{self, OutputMode};
use anyhow::Result;
use clap::Subcommand;
use porkbun_lib::client::Client;
use porkbun_lib::models::dns::{DnsRecord, DnsRecordRow};

#[derive(Subcommand)]
pub enum DnsAction {
    /// Create a DNS record
    Create {
        domain: String,
        #[arg(long)]
        r#type: String,
        #[arg(long)]
        content: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        ttl: Option<String>,
        #[arg(long)]
        prio: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Edit a DNS record by ID
    Edit {
        domain: String,
        id: String,
        #[arg(long)]
        r#type: String,
        #[arg(long)]
        content: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        ttl: Option<String>,
        #[arg(long)]
        prio: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Edit DNS records by name and type
    #[command(name = "edit-by-name-type")]
    EditByNameType {
        domain: String,
        r#type: String,
        subdomain: Option<String>,
        #[arg(long)]
        content: String,
        #[arg(long)]
        ttl: Option<String>,
        #[arg(long)]
        prio: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Delete a DNS record by ID
    Delete { domain: String, id: String },
    /// Delete DNS records by name and type
    #[command(name = "delete-by-name-type")]
    DeleteByNameType {
        domain: String,
        r#type: String,
        subdomain: Option<String>,
    },
    /// Retrieve DNS records
    Retrieve { domain: String, id: Option<String> },
    /// Retrieve DNS records by name and type
    #[command(name = "retrieve-by-name-type")]
    RetrieveByNameType {
        domain: String,
        r#type: String,
        subdomain: Option<String>,
    },
}

pub fn run(action: DnsAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DnsAction::Create {
            domain,
            r#type,
            content,
            name,
            ttl,
            prio,
            notes,
        } => {
            let mut body = serde_json::json!({ "type": r#type, "content": content });
            if let Some(n) = name {
                body["name"] = serde_json::json!(n);
            }
            if let Some(t) = ttl {
                body["ttl"] = serde_json::json!(t);
            }
            if let Some(p) = prio {
                body["prio"] = serde_json::json!(p);
            }
            if let Some(n) = notes {
                body["notes"] = serde_json::json!(n);
            }
            let json = client.post_raw(&format!("dns/create/{}", domain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let id = json.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
                    output::print_kv(&[("Record ID", id.to_string())]);
                }
            }
            Ok(())
        }
        DnsAction::Edit {
            domain,
            id,
            r#type,
            content,
            name,
            ttl,
            prio,
            notes,
        } => {
            let mut body = serde_json::json!({ "type": r#type, "content": content });
            if let Some(n) = name {
                body["name"] = serde_json::json!(n);
            }
            if let Some(t) = ttl {
                body["ttl"] = serde_json::json!(t);
            }
            if let Some(p) = prio {
                body["prio"] = serde_json::json!(p);
            }
            if let Some(n) = notes {
                body["notes"] = serde_json::json!(n);
            }
            let json = client.post_raw(&format!("dns/edit/{}/{}", domain, id), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNS record updated."),
            }
            Ok(())
        }
        DnsAction::EditByNameType {
            domain,
            r#type,
            subdomain,
            content,
            ttl,
            prio,
            notes,
        } => {
            let sub = subdomain.as_deref().unwrap_or("");
            let mut body = serde_json::json!({ "content": content });
            if let Some(t) = ttl {
                body["ttl"] = serde_json::json!(t);
            }
            if let Some(p) = prio {
                body["prio"] = serde_json::json!(p);
            }
            if let Some(n) = notes {
                body["notes"] = serde_json::json!(n);
            }
            let json = client.post_raw(
                &format!("dns/editByNameType/{}/{}/{}", domain, r#type, sub),
                body,
            )?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNS record(s) updated."),
            }
            Ok(())
        }
        DnsAction::Delete { domain, id } => {
            let json = client.post_raw(
                &format!("dns/delete/{}/{}", domain, id),
                serde_json::json!({}),
            )?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNS record deleted."),
            }
            Ok(())
        }
        DnsAction::DeleteByNameType {
            domain,
            r#type,
            subdomain,
        } => {
            let sub = subdomain.as_deref().unwrap_or("");
            let json = client.post_raw(
                &format!("dns/deleteByNameType/{}/{}/{}", domain, r#type, sub),
                serde_json::json!({}),
            )?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNS record(s) deleted."),
            }
            Ok(())
        }
        DnsAction::Retrieve { domain, id } => {
            let path = match id {
                Some(ref i) => format!("dns/retrieve/{}/{}", domain, i),
                None => format!("dns/retrieve/{}", domain),
            };
            let json = client.post_raw(&path, serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let records: Vec<DnsRecord> = if json["records"].is_null() {
                        Vec::new()
                    } else {
                        serde_json::from_value(json["records"].clone())?
                    };
                    let rows: Vec<DnsRecordRow> = records.iter().map(DnsRecordRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
        DnsAction::RetrieveByNameType {
            domain,
            r#type,
            subdomain,
        } => {
            let sub = subdomain.as_deref().unwrap_or("");
            let json = client.post_raw(
                &format!("dns/retrieveByNameType/{}/{}/{}", domain, r#type, sub),
                serde_json::json!({}),
            )?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let records: Vec<DnsRecord> = if json["records"].is_null() {
                        Vec::new()
                    } else {
                        serde_json::from_value(json["records"].clone())?
                    };
                    let rows: Vec<DnsRecordRow> = records.iter().map(DnsRecordRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
    }
}
