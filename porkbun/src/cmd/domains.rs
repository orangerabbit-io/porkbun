use crate::output::{self, OutputMode};
use anyhow::Result;
use clap::Subcommand;
use porkbun_lib::client::Client;
use porkbun_lib::models::domain::{Domain, DomainAvailability, DomainCreate, DomainRow};

#[derive(Subcommand)]
pub enum DomainsAction {
    /// List all domains
    List {
        /// Pagination offset (default 0)
        #[arg(long)]
        start: Option<i64>,
        /// Include label data
        #[arg(long)]
        include_labels: bool,
    },
    /// Check domain availability
    Check {
        /// Domain name to check
        domain: String,
    },
    /// Register a new domain
    Create {
        /// Domain name to register
        domain: String,
        /// Cost in pennies for minimum duration
        #[arg(long)]
        cost: i64,
        /// Agree to terms of service
        #[arg(long)]
        agree_to_terms: bool,
    },
    /// Get authoritative name servers
    #[command(name = "get-ns")]
    GetNs {
        /// Domain name
        domain: String,
    },
    /// Update name servers
    #[command(name = "update-ns")]
    UpdateNs {
        /// Domain name
        domain: String,
        /// Name server hostnames
        #[arg(long, num_args = 1..)]
        ns: Vec<String>,
    },
    /// Toggle auto-renew
    #[command(name = "update-auto-renew")]
    UpdateAutoRenew {
        /// Domain name
        domain: String,
        /// on or off
        #[arg(long, value_parser = ["on", "off"])]
        status: String,
    },
}

pub fn run(action: DomainsAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DomainsAction::List {
            start,
            include_labels,
        } => {
            let mut body = serde_json::json!({});
            if let Some(s) = start {
                body["start"] = serde_json::json!(s);
            }
            if include_labels {
                body["includeLabels"] = serde_json::json!("yes");
            }
            let json = client.post_raw("domain/listAll", body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let domains: Vec<Domain> = serde_json::from_value(json["domains"].clone())?;
                    let rows: Vec<DomainRow> = domains.iter().map(DomainRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
        DomainsAction::Check { domain } => {
            let json = client.post_raw(
                &format!("domain/checkDomain/{}", domain),
                serde_json::json!({}),
            )?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let da: DomainAvailability = serde_json::from_value(json["response"].clone())?;
                    output::print_kv(&[
                        ("Available", da.avail.to_string()),
                        ("Type", da.avail_type),
                        ("Price", da.price.unwrap_or("-".to_string())),
                        (
                            "First Year Promo",
                            da.first_year_promo.unwrap_or("-".to_string()),
                        ),
                        ("Regular Price", da.regular_price.unwrap_or("-".to_string())),
                        (
                            "Premium",
                            da.premium.map(|b| b.to_string()).unwrap_or("-".to_string()),
                        ),
                        (
                            "Min Duration",
                            da.min_duration
                                .map(|d| d.to_string())
                                .unwrap_or("-".to_string()),
                        ),
                    ]);
                }
            }
            Ok(())
        }
        DomainsAction::Create {
            domain,
            cost,
            agree_to_terms,
        } => {
            let body = serde_json::json!({
                "cost": cost,
                "agreeToTerms": if agree_to_terms { "yes" } else { "no" },
            });
            let json = client.post_raw(&format!("domain/create/{}", domain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let dc: DomainCreate = serde_json::from_value(json.clone())?;
                    output::print_kv(&[
                        ("Domain", dc.domain),
                        ("Cost", dc.cost),
                        ("Order ID", dc.order_id.to_string()),
                        ("Balance", dc.balance),
                    ]);
                }
            }
            Ok(())
        }
        DomainsAction::GetNs { domain } => {
            let json =
                client.post_raw(&format!("domain/getNs/{}", domain), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    if let Some(ns) = json.get("ns").and_then(|v| v.as_array()) {
                        for n in ns {
                            if let Some(s) = n.as_str() {
                                println!("{}", s);
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        DomainsAction::UpdateNs { domain, ns } => {
            let body = serde_json::json!({ "ns": ns });
            let json = client.post_raw(&format!("domain/updateNs/{}", domain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("Name servers updated."),
            }
            Ok(())
        }
        DomainsAction::UpdateAutoRenew { domain, status } => {
            let body = serde_json::json!({ "status": status });
            let json = client.post_raw(&format!("domain/updateAutoRenew/{}", domain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    if let Some(results) = json.get("results").and_then(|v| v.as_object()) {
                        for (domain, result) in results {
                            let status =
                                result.get("status").and_then(|v| v.as_str()).unwrap_or("-");
                            let message =
                                result.get("message").and_then(|v| v.as_str()).unwrap_or("");
                            output::print_kv(&[
                                ("Domain", domain.clone()),
                                ("Status", status.to_string()),
                                ("Message", message.to_string()),
                            ]);
                        }
                    } else {
                        output::print_confirm("Auto-renew updated.");
                    }
                }
            }
            Ok(())
        }
    }
}
