use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub secret_api_key: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

fn default_base_url() -> String {
    "https://api.porkbun.com/api/json/v3".to_string()
}

impl Config {
    pub fn load(api_key_override: Option<&str>, secret_api_key_override: Option<&str>) -> Result<Self> {
        let base_url = std::env::var("PORKBUN_BASE_URL").unwrap_or_else(|_| default_base_url());

        // Resolve each key independently: flag > env > (defer to config file)
        let api_key = if let Some(key) = api_key_override {
            Some(key.to_string())
        } else if let Ok(key) = std::env::var("PORKBUN_API_KEY") {
            Some(key)
        } else {
            None
        };

        let secret_api_key = if let Some(key) = secret_api_key_override {
            Some(key.to_string())
        } else if let Ok(key) = std::env::var("PORKBUN_SECRET_API_KEY") {
            Some(key)
        } else {
            None
        };

        // If both resolved, return early
        if let (Some(ak), Some(sk)) = (api_key.clone(), secret_api_key.clone()) {
            return Ok(Config { api_key: ak, secret_api_key: sk, base_url });
        }

        // Try config file for any missing keys
        let path = Self::config_path()?;
        let contents = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "No API key found. Create a config file at {} with:\n\n  api_key = \"pk1_...\"\n  secret_api_key = \"sk1_...\"\n\nOr set PORKBUN_API_KEY and PORKBUN_SECRET_API_KEY environment variables.",
                path.display()
            )
        })?;

        let file_config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file at {}", path.display()))?;

        Ok(Config {
            api_key: api_key.unwrap_or(file_config.api_key),
            secret_api_key: secret_api_key.unwrap_or(file_config.secret_api_key),
            base_url,
        })
    }

    fn config_path() -> Result<PathBuf> {
        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        Ok(PathBuf::from(home).join(".config/porkbun/config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_cli_flags_take_priority() {
        let config = Config::load(Some("flag-api"), Some("flag-secret")).unwrap();
        assert_eq!(config.api_key, "flag-api");
        assert_eq!(config.secret_api_key, "flag-secret");
    }

    #[test]
    #[serial]
    fn test_env_var_override() {
        std::env::set_var("PORKBUN_API_KEY", "env-api");
        std::env::set_var("PORKBUN_SECRET_API_KEY", "env-secret");
        let config = Config::load(None, None).unwrap();
        assert_eq!(config.api_key, "env-api");
        assert_eq!(config.secret_api_key, "env-secret");
        std::env::remove_var("PORKBUN_API_KEY");
        std::env::remove_var("PORKBUN_SECRET_API_KEY");
    }

    #[test]
    #[serial]
    fn test_base_url_env_override() {
        std::env::set_var("PORKBUN_BASE_URL", "http://localhost:9999");
        let config = Config::load(Some("k"), Some("s")).unwrap();
        assert_eq!(config.base_url, "http://localhost:9999");
        std::env::remove_var("PORKBUN_BASE_URL");
    }

    #[test]
    #[serial]
    fn test_default_base_url() {
        std::env::remove_var("PORKBUN_BASE_URL");
        let config = Config::load(Some("k"), Some("s")).unwrap();
        assert_eq!(config.base_url, "https://api.porkbun.com/api/json/v3");
    }

    #[test]
    #[serial]
    fn test_missing_api_key_errors() {
        std::env::remove_var("PORKBUN_API_KEY");
        std::env::remove_var("PORKBUN_SECRET_API_KEY");
        let result = Config::load(None, None);
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_partial_flag_override_with_env() {
        std::env::set_var("PORKBUN_API_KEY", "env-api");
        std::env::set_var("PORKBUN_SECRET_API_KEY", "env-secret");
        let config = Config::load(Some("flag-api"), None).unwrap();
        assert_eq!(config.api_key, "flag-api");
        assert_eq!(config.secret_api_key, "env-secret");
        std::env::remove_var("PORKBUN_API_KEY");
        std::env::remove_var("PORKBUN_SECRET_API_KEY");
    }
}
