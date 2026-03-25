use crate::output::{self, OutputMode};
use anyhow::Result;
use clap::Subcommand;
use porkbun_lib::client::Client;
use porkbun_lib::models::url_forward::{UrlForward, UrlForwardRow};

#[derive(Subcommand)]
pub enum UrlForwardAction {
    /// Add a URL forward
    Add {
        domain: String,
        #[arg(long)]
        location: String,
        #[arg(long)]
        r#type: String,
        #[arg(long)]
        include_path: String,
        #[arg(long)]
        wildcard: String,
        #[arg(long)]
        subdomain: Option<String>,
    },
    /// List URL forwards
    List { domain: String },
    /// Delete a URL forward
    Delete { domain: String, record_id: String },
}

pub fn run(action: UrlForwardAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        UrlForwardAction::Add {
            domain,
            location,
            r#type,
            include_path,
            wildcard,
            subdomain,
        } => {
            let mut body = serde_json::json!({
                "location": location,
                "type": r#type,
                "includePath": include_path,
                "wildcard": wildcard,
            });
            if let Some(s) = subdomain {
                body["subdomain"] = serde_json::json!(s);
            }
            let json = client.post_raw(&format!("domain/addUrlForward/{}", domain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("URL forward added."),
            }
            Ok(())
        }
        UrlForwardAction::List { domain } => {
            let json = client.post_raw(
                &format!("domain/getUrlForwarding/{}", domain),
                serde_json::json!({}),
            )?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let forwards: Vec<UrlForward> =
                        serde_json::from_value(json["forwards"].clone())?;
                    let rows: Vec<UrlForwardRow> =
                        forwards.iter().map(UrlForwardRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
        UrlForwardAction::Delete { domain, record_id } => {
            let json = client.post_raw(
                &format!("domain/deleteUrlForward/{}/{}", domain, record_id),
                serde_json::json!({}),
            )?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("URL forward deleted."),
            }
            Ok(())
        }
    }
}
