use crate::output::{self, OutputMode};
use anyhow::Result;
use porkbun_lib::client::Client;

pub fn run(client: &Client, mode: OutputMode) -> Result<()> {
    let json = client.post_raw("ping", serde_json::json!({}))?;
    match mode {
        OutputMode::Json => output::print_json(&json),
        OutputMode::Table => {
            let ip = json.get("yourIp").and_then(|v| v.as_str()).unwrap_or("-");
            output::print_kv(&[("Your IP", ip.to_string())]);
        }
    }
    Ok(())
}
