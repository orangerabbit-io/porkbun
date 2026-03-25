use crate::output::{self, OutputMode};
use anyhow::Result;
use clap::Subcommand;
use porkbun_lib::client::Client;
use porkbun_lib::models::pricing::{TldPricing, TldPricingRow};
use std::collections::HashMap;

#[derive(Subcommand)]
pub enum PricingAction {
    /// Get pricing for all TLDs
    Get,
}

pub fn run(action: &PricingAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        PricingAction::Get => {
            let json = client.post_unauthenticated("pricing/get")?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let pricing: HashMap<String, TldPricing> =
                        serde_json::from_value(json["pricing"].clone())?;
                    let mut rows: Vec<TldPricingRow> = pricing
                        .iter()
                        .map(|(tld, p)| TldPricingRow::from_entry(tld, p))
                        .collect();
                    rows.sort_by(|a, b| a.tld.cmp(&b.tld));
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
    }
}
