use anyhow::{bail, Context, Result};
use reqwest::blocking::Client as HttpClient;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};
use serde_json::Value;

pub struct Client {
    http: HttpClient,
    base_url: String,
    api_key: String,
    secret_api_key: String,
}

impl Client {
    pub fn new(api_key: String, secret_api_key: String, base_url: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));

        let http = HttpClient::builder()
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Client {
            http,
            base_url,
            api_key,
            secret_api_key,
        })
    }

    pub fn unauthenticated(base_url: String) -> Result<Self> {
        Self::new(String::new(), String::new(), base_url)
    }

    pub fn post_raw(&self, path: &str, body: Value) -> Result<Value> {
        let url = format!("{}/{}", self.base_url, path);
        let mut body = match body {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(serde_json::Map::new()),
        };
        if let Value::Object(ref mut map) = body {
            map.insert("apikey".to_string(), Value::String(self.api_key.clone()));
            map.insert(
                "secretapikey".to_string(),
                Value::String(self.secret_api_key.clone()),
            );
        }

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;

        let status = resp.status();
        if status.as_u16() == 403 {
            bail!("Authentication failed (HTTP 403): two-factor authentication may be required");
        }
        if !status.is_success() {
            let body = resp.text().unwrap_or_default();
            bail!("API error (HTTP {}) for {}: {}", status, url, body);
        }

        let json: Value = resp
            .json()
            .with_context(|| format!("Failed to parse JSON response from {}", url))?;

        if json.get("status").and_then(|s| s.as_str()) == Some("ERROR") {
            let message = json
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");
            bail!("API error: {}", message);
        }

        Ok(json)
    }

    pub fn post_unauthenticated(&self, path: &str) -> Result<Value> {
        let url = format!("{}/{}", self.base_url, path);
        let body = Value::Object(serde_json::Map::new());

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().unwrap_or_default();
            bail!("API error (HTTP {}) for {}: {}", status, url, body);
        }

        let json: Value = resp
            .json()
            .with_context(|| format!("Failed to parse JSON response from {}", url))?;

        if json.get("status").and_then(|s| s.as_str()) == Some("ERROR") {
            let message = json
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");
            bail!("API error: {}", message);
        }

        Ok(json)
    }
}
