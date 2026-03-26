use crate::output::{self, OutputMode};
use anyhow::Result;
use clap::Subcommand;
use porkbun_lib::client::Client;
use porkbun_lib::models::ssl::SslBundle;

#[derive(Subcommand)]
pub enum SslAction {
    /// Retrieve SSL certificate bundle
    Retrieve { domain: String },
}

pub fn run(action: SslAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        SslAction::Retrieve { domain } => {
            let json =
                client.post_raw(&format!("ssl/retrieve/{}", domain), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let ssl: SslBundle = serde_json::from_value(json)?;
                    output::print_kv(&[
                        ("Certificate Chain", ssl.certificatechain),
                        ("Private Key", ssl.privatekey),
                        ("Public Key", ssl.publickey),
                    ]);
                }
            }
            Ok(())
        }
    }
}
