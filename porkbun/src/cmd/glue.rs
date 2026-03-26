use crate::output::{self, OutputMode};
use anyhow::Result;
use clap::Subcommand;
use porkbun_lib::client::Client;
use porkbun_lib::models::glue::{deserialize_glue_hosts, GlueRecordRow};

#[derive(Subcommand)]
pub enum GlueAction {
    /// Create a glue record
    Create {
        domain: String,
        subdomain: String,
        #[arg(long, num_args = 1..)]
        ips: Vec<String>,
    },
    /// Update a glue record
    Update {
        domain: String,
        subdomain: String,
        #[arg(long, num_args = 1..)]
        ips: Vec<String>,
    },
    /// Delete a glue record
    Delete { domain: String, subdomain: String },
    /// List glue records
    List { domain: String },
}

pub fn run(action: GlueAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        GlueAction::Create {
            domain,
            subdomain,
            ips,
        } => {
            let body = serde_json::json!({ "ips": ips });
            let json =
                client.post_raw(&format!("domain/createGlue/{}/{}", domain, subdomain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("Glue record created."),
            }
            Ok(())
        }
        GlueAction::Update {
            domain,
            subdomain,
            ips,
        } => {
            let body = serde_json::json!({ "ips": ips });
            let json =
                client.post_raw(&format!("domain/updateGlue/{}/{}", domain, subdomain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("Glue record updated."),
            }
            Ok(())
        }
        GlueAction::Delete { domain, subdomain } => {
            let json = client.post_raw(
                &format!("domain/deleteGlue/{}/{}", domain, subdomain),
                serde_json::json!({}),
            )?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("Glue record deleted."),
            }
            Ok(())
        }
        GlueAction::List { domain } => {
            let json =
                client.post_raw(&format!("domain/getGlue/{}", domain), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let empty = serde_json::json!([]);
                    let hosts = json.get("hosts").unwrap_or(&empty);
                    let records = deserialize_glue_hosts(hosts)
                        .map_err(|e| anyhow::anyhow!("Failed to parse glue records: {}", e))?;
                    let rows: Vec<GlueRecordRow> =
                        records.iter().map(GlueRecordRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
    }
}
