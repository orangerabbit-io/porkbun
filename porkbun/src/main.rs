mod cmd;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use porkbun_lib::client::Client;
use porkbun_lib::config::Config;
use std::process;

#[derive(Parser)]
#[command(name = "porkbun", about = "CLI for the Porkbun domain registrar API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Force JSON output
    #[arg(long, global = true)]
    pub json: bool,

    /// API key (overrides config file and env var)
    #[arg(long, global = true)]
    pub api_key: Option<String>,

    /// Secret API key (overrides config file and env var)
    #[arg(long, global = true)]
    pub secret_api_key: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Test API authentication
    Ping,
    /// TLD pricing information
    Pricing {
        #[command(subcommand)]
        action: cmd::pricing::PricingAction,
    },
    /// Domain management
    Domains {
        #[command(subcommand)]
        action: cmd::domains::DomainsAction,
    },
    /// DNS record management
    Dns {
        #[command(subcommand)]
        action: cmd::dns::DnsAction,
    },
    /// DNSSEC record management
    Dnssec {
        #[command(subcommand)]
        action: cmd::dnssec::DnssecAction,
    },
    /// URL forwarding management
    #[command(name = "url-forward")]
    UrlForward {
        #[command(subcommand)]
        action: cmd::url_forward::UrlForwardAction,
    },
    /// Glue record management
    Glue {
        #[command(subcommand)]
        action: cmd::glue::GlueAction,
    },
    /// SSL certificate retrieval
    Ssl {
        #[command(subcommand)]
        action: cmd::ssl::SslAction,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {:#}", e);

        let exit_code = if format!("{:#}", e).contains("No API key found")
            || format!("{:#}", e).contains("Failed to parse config")
            || format!("{:#}", e).contains("HOME environment variable")
        {
            2
        } else {
            1
        };
        process::exit(exit_code);
    }
}

fn run(cli: Cli) -> Result<()> {
    let mode = output::OutputMode::from_json_flag(cli.json);

    // pricing get doesn't need auth
    if let Commands::Pricing { ref action } = cli.command {
        let base_url = std::env::var("PORKBUN_BASE_URL")
            .unwrap_or_else(|_| "https://api.porkbun.com/api/json/v3".to_string());
        let client = Client::unauthenticated(base_url)?;
        return cmd::pricing::run(action, &client, mode);
    }

    let config = Config::load(cli.api_key.as_deref(), cli.secret_api_key.as_deref())?;
    let client = Client::new(config.api_key, config.secret_api_key, config.base_url)?;

    match cli.command {
        Commands::Ping => cmd::ping::run(&client, mode),
        Commands::Pricing { .. } => unreachable!(),
        Commands::Domains { action } => cmd::domains::run(action, &client, mode),
        Commands::Dns { action } => cmd::dns::run(action, &client, mode),
        Commands::Dnssec { action } => cmd::dnssec::run(action, &client, mode),
        Commands::UrlForward { action } => cmd::url_forward::run(action, &client, mode),
        Commands::Glue { action } => cmd::glue::run(action, &client, mode),
        Commands::Ssl { action } => cmd::ssl::run(action, &client, mode),
    }
}
