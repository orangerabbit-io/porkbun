use crate::output::{self, OutputMode};
use anyhow::Result;
use clap::Subcommand;
use porkbun_lib::client::Client;
use porkbun_lib::models::dnssec::{DnssecRecord, DnssecRecordRow};
use std::collections::HashMap;

#[derive(Subcommand)]
pub enum DnssecAction {
    /// Create a DNSSEC record
    Create {
        domain: String,
        #[arg(long)]
        key_tag: String,
        #[arg(long)]
        alg: String,
        #[arg(long)]
        digest_type: String,
        #[arg(long)]
        digest: String,
        #[arg(long)]
        max_sig_life: Option<String>,
        #[arg(long)]
        key_data_flags: Option<String>,
        #[arg(long)]
        key_data_protocol: Option<String>,
        #[arg(long)]
        key_data_algo: Option<String>,
        #[arg(long)]
        key_data_pub_key: Option<String>,
    },
    /// List DNSSEC records
    List { domain: String },
    /// Delete a DNSSEC record
    Delete { domain: String, key_tag: String },
}

pub fn run(action: DnssecAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DnssecAction::Create {
            domain,
            key_tag,
            alg,
            digest_type,
            digest,
            max_sig_life,
            key_data_flags,
            key_data_protocol,
            key_data_algo,
            key_data_pub_key,
        } => {
            let mut body = serde_json::json!({
                "keyTag": key_tag,
                "alg": alg,
                "digestType": digest_type,
                "digest": digest,
            });
            if let Some(v) = max_sig_life {
                body["maxSigLife"] = serde_json::json!(v);
            }
            if let Some(v) = key_data_flags {
                body["keyDataFlags"] = serde_json::json!(v);
            }
            if let Some(v) = key_data_protocol {
                body["keyDataProtocol"] = serde_json::json!(v);
            }
            if let Some(v) = key_data_algo {
                body["keyDataAlgo"] = serde_json::json!(v);
            }
            if let Some(v) = key_data_pub_key {
                body["keyDataPubKey"] = serde_json::json!(v);
            }
            let json = client.post_raw(&format!("dns/createDnssecRecord/{}", domain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNSSEC record created."),
            }
            Ok(())
        }
        DnssecAction::List { domain } => {
            let json = client.post_raw(
                &format!("dns/getDnssecRecords/{}", domain),
                serde_json::json!({}),
            )?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let records: HashMap<String, DnssecRecord> = if json["records"].is_null() {
                        HashMap::new()
                    } else {
                        serde_json::from_value(json["records"].clone())?
                    };
                    let rows: Vec<DnssecRecordRow> =
                        records.values().map(DnssecRecordRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
        DnssecAction::Delete { domain, key_tag } => {
            let json = client.post_raw(
                &format!("dns/deleteDnssecRecord/{}/{}", domain, key_tag),
                serde_json::json!({}),
            )?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNSSEC record deleted."),
            }
            Ok(())
        }
    }
}
