# Porkbun CLI Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust CLI and library for the Porkbun domain registrar API v3 with full test coverage.

**Architecture:** Two-crate Cargo workspace (`porkbun-lib` for the API client library, `porkbun` for the CLI binary). All Porkbun API requests are POST with auth credentials in the JSON body. The client uses `post_raw` returning `serde_json::Value`, and each command handler extracts its specific response key.

**Tech Stack:** Rust (blocking reqwest, clap derive, serde, anyhow, tabled), mockito for integration tests, assert_cmd/predicates for CLI testing.

**Spec:** `docs/superpowers/specs/2026-03-25-porkbun-cli-design.md`

**Reference project:** `/home/alindsay/projects/orangerabbit-io/updown` — follow its patterns exactly for file structure, code style, test style, and infrastructure.

---

## Chunk 1: Project Scaffold and Infrastructure

### Task 1: Workspace and Cargo configuration

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `porkbun-lib/Cargo.toml`
- Create: `porkbun/Cargo.toml`

- [ ] **Step 1: Create workspace root Cargo.toml**

```toml
[workspace]
members = ["porkbun-lib", "porkbun"]
resolver = "2"
```

- [ ] **Step 2: Create porkbun-lib/Cargo.toml**

```toml
[package]
name = "porkbun-lib"
version = "0.1.0"
edition = "2021"
description = "Rust client library for the Porkbun domain registrar API"
license = "MIT OR Apache-2.0"
repository = "https://github.com/orangerabbit-io/porkbun"
homepage = "https://github.com/orangerabbit-io/porkbun"
readme = "../README.md"
keywords = ["porkbun", "dns", "domain", "api", "client"]
categories = ["api-bindings"]

[dependencies]
anyhow = "1"
reqwest = { version = "0.12", features = ["blocking", "json", "gzip"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tabled = "0.17"
toml = "0.8"

[dev-dependencies]
serial_test = "3"
```

- [ ] **Step 3: Create porkbun/Cargo.toml**

```toml
[package]
name = "porkbun"
version = "0.1.0"
edition = "2021"
description = "Command-line interface for the Porkbun domain registrar API"
license = "MIT OR Apache-2.0"
repository = "https://github.com/orangerabbit-io/porkbun"
homepage = "https://github.com/orangerabbit-io/porkbun"
readme = "../README.md"
keywords = ["porkbun", "dns", "domain", "cli", "registrar"]
categories = ["command-line-utilities"]

[dependencies]
porkbun-lib = { path = "../porkbun-lib" }
anyhow = "1"
clap = { version = "4", features = ["derive"] }
serde_json = "1"
tabled = "0.17"

[dev-dependencies]
mockito = "1"
assert_cmd = "2"
predicates = "3"
serial_test = "3"
```

- [ ] **Step 4: Create minimal lib.rs and main.rs so workspace compiles**

Create `porkbun-lib/src/lib.rs`:
```rust
pub mod client;
pub mod config;
pub mod models;
```

Create `porkbun-lib/src/client.rs`:
```rust
// Placeholder — implemented in Task 3
```

Create `porkbun-lib/src/config.rs`:
```rust
// Placeholder — implemented in Task 2
```

Create `porkbun-lib/src/models/mod.rs`:
```rust
// Placeholder — models added in Tasks 4-10
```

Create `porkbun/src/main.rs`:
```rust
fn main() {
    println!("porkbun CLI");
}
```

- [ ] **Step 5: Verify workspace compiles**

Run: `cargo check --workspace`
Expected: compiles with no errors

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml porkbun-lib/ porkbun/
git commit -m "chore: scaffold cargo workspace with porkbun-lib and porkbun crates"
```

### Task 1b: Infrastructure files

**Files:**
- Create: `.gitignore`
- Create: `flake.nix`
- Create: `.releaserc.json`
- Create: `package.json`
- Create: `.github/workflows/release.yml`
- Create: `LICENSE-MIT`
- Create: `LICENSE-APACHE`

- [ ] **Step 1: Create .gitignore**

```
/target
porkbun-lib/target
porkbun/target
.direnv
.env
*.pem
*.key
node_modules
```

- [ ] **Step 2: Create flake.nix**

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "porkbun";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
            pkg-config
            openssl
          ];
        };
      });
}
```

- [ ] **Step 3: Create .releaserc.json**

```json
{
  "branches": ["main"],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    ["@semantic-release/changelog", {
      "changelogFile": "CHANGELOG.md"
    }],
    ["@semantic-release/exec", {
      "prepareCmd": "sed -i 's/^version = .*/version = \"${nextRelease.version}\"/' porkbun/Cargo.toml porkbun-lib/Cargo.toml && sed -i 's/version = \"[0-9]*\\.[0-9]*\\.[0-9]*\";/version = \"${nextRelease.version}\";/' flake.nix && cargo generate-lockfile"
    }],
    ["@semantic-release/git", {
      "assets": ["CHANGELOG.md", "Cargo.lock", "flake.nix", "porkbun/Cargo.toml", "porkbun-lib/Cargo.toml"],
      "message": "chore(release): ${nextRelease.version}\n\n${nextRelease.notes}"
    }],
    "@semantic-release/github"
  ]
}
```

- [ ] **Step 4: Create package.json**

```json
{
  "private": true,
  "devDependencies": {
    "semantic-release": "^24",
    "@semantic-release/changelog": "^6",
    "@semantic-release/exec": "^7",
    "@semantic-release/git": "^10",
    "@semantic-release/github": "^11"
  }
}
```

- [ ] **Step 5: Create .github/workflows/release.yml**

```yaml
name: Release

on:
  push:
    branches: [main]

permissions:
  contents: write
  issues: write
  pull-requests: write

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: actions/setup-node@v4
        with:
          node-version: 22
      - uses: dtolnay/rust-toolchain@stable
      - run: npm install
      - run: npx semantic-release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

- [ ] **Step 6: Create LICENSE-MIT**

Copy from `/home/alindsay/projects/orangerabbit-io/updown/LICENSE-MIT` — same text, same copyright holder (Orange Rabbit).

- [ ] **Step 7: Create LICENSE-APACHE**

Copy from `/home/alindsay/projects/orangerabbit-io/updown/LICENSE-APACHE` — standard Apache 2.0 text.

- [ ] **Step 8: Commit**

```bash
git add .gitignore flake.nix .releaserc.json package.json .github/ LICENSE-MIT LICENSE-APACHE
git commit -m "chore: add infrastructure files (nix, semantic-release, CI, licenses)"
```

---

## Chunk 2: Library Core (Config + Client)

### Task 2: Configuration module

**Files:**
- Create: `porkbun-lib/src/config.rs`
- Test: inline unit tests

The config module loads API credentials from three sources in priority order: CLI flags > env vars > config file. It handles TWO keys (api_key + secret_api_key) unlike the single-key updown project.

- [ ] **Step 1: Write failing tests for config priority**

Write in `porkbun-lib/src/config.rs`:

```rust
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
    pub fn load(api_key: Option<&str>, secret_api_key: Option<&str>) -> Result<Self> {
        todo!()
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
        // Only api_key from flag, secret from env
        let config = Config::load(Some("flag-api"), None).unwrap();
        assert_eq!(config.api_key, "flag-api");
        assert_eq!(config.secret_api_key, "env-secret");
        std::env::remove_var("PORKBUN_API_KEY");
        std::env::remove_var("PORKBUN_SECRET_API_KEY");
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p porkbun-lib config::tests`
Expected: FAIL — `todo!()` panics

- [ ] **Step 3: Implement Config::load**

Replace the `todo!()` in `Config::load`:

```rust
pub fn load(api_key_override: Option<&str>, secret_api_key_override: Option<&str>) -> Result<Self> {
    let base_url = std::env::var("PORKBUN_BASE_URL").unwrap_or_else(|_| default_base_url());

    // Resolve each key independently: flag > env > config file
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
        return Ok(Config {
            api_key: ak,
            secret_api_key: sk,
            base_url,
        });
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
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p porkbun-lib config::tests`
Expected: all 6 tests PASS

- [ ] **Step 5: Commit**

```bash
git add porkbun-lib/src/config.rs
git commit -m "feat: add configuration module with three-source priority chain"
```

### Task 3: HTTP Client

**Files:**
- Create: `porkbun-lib/src/client.rs`
- Test: inline unit tests

The Porkbun client is POST-only with auth credentials merged into the JSON body. This differs from updown which uses header-based auth and GET/POST/PUT/DELETE.

- [ ] **Step 1: Write the client with tests**

```rust
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

    /// Unauthenticated client for endpoints that don't need API keys (e.g. pricing/get).
    pub fn unauthenticated(base_url: String) -> Result<Self> {
        Self::new(String::new(), String::new(), base_url)
    }

    /// Authenticated POST returning raw JSON Value.
    /// Merges apikey and secretapikey into the body, checks status field.
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

    /// Unauthenticated POST returning raw JSON Value (for pricing/get).
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
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo check -p porkbun-lib`
Expected: compiles with no errors

- [ ] **Step 3: Commit**

```bash
git add porkbun-lib/src/client.rs
git commit -m "feat: add HTTP client with POST-only auth-in-body pattern"
```

---

## Chunk 3: Models

### Task 4: DNS Record model

**Files:**
- Create: `porkbun-lib/src/models/dns.rs`
- Modify: `porkbun-lib/src/models/mod.rs`

- [ ] **Step 1: Write model with unit test**

`porkbun-lib/src/models/dns.rs`:

```rust
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecord {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub content: String,
    pub ttl: String,
    #[serde(default)]
    pub prio: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct DnsRecordRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "TYPE")]
    pub record_type: String,
    #[tabled(rename = "CONTENT")]
    pub content: String,
    #[tabled(rename = "TTL")]
    pub ttl: String,
    #[tabled(rename = "PRIO")]
    pub prio: String,
}

impl From<&DnsRecord> for DnsRecordRow {
    fn from(r: &DnsRecord) -> Self {
        DnsRecordRow {
            id: r.id.clone(),
            name: r.name.clone(),
            record_type: r.record_type.clone(),
            content: r.content.clone(),
            ttl: r.ttl.clone(),
            prio: r.prio.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_dns_record() {
        let json = r#"{
            "id": "326953021",
            "name": "www.example.com",
            "type": "A",
            "content": "1.2.3.4",
            "ttl": "600",
            "prio": "0",
            "notes": "test record"
        }"#;
        let record: DnsRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "326953021");
        assert_eq!(record.record_type, "A");
        assert_eq!(record.content, "1.2.3.4");
        assert_eq!(record.notes, Some("test record".to_string()));
    }

    #[test]
    fn test_dns_record_row() {
        let record = DnsRecord {
            id: "1".to_string(),
            name: "test.example.com".to_string(),
            record_type: "CNAME".to_string(),
            content: "other.example.com".to_string(),
            ttl: "3600".to_string(),
            prio: None,
            notes: None,
        };
        let row = DnsRecordRow::from(&record);
        assert_eq!(row.prio, "-");
    }
}
```

Update `porkbun-lib/src/models/mod.rs`:
```rust
pub mod dns;
```

- [ ] **Step 2: Run tests**

Run: `cargo test -p porkbun-lib models::dns::tests`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add porkbun-lib/src/models/
git commit -m "feat: add DnsRecord model with row display"
```

### Task 5: Domain model

**Files:**
- Create: `porkbun-lib/src/models/domain.rs`
- Modify: `porkbun-lib/src/models/mod.rs`

- [ ] **Step 1: Write model with tests**

`porkbun-lib/src/models/domain.rs`:

```rust
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub domain: String,
    pub status: String,
    pub tld: String,
    pub create_date: String,
    pub expire_date: String,
    pub security_lock: String,
    pub whois_privacy: String,
    pub auto_renew: String,
    pub not_local: i64,
    #[serde(default)]
    pub labels: Option<Vec<Label>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    pub id: String,
    pub title: String,
    pub color: String,
}

#[derive(Debug, Tabled)]
pub struct DomainRow {
    #[tabled(rename = "DOMAIN")]
    pub domain: String,
    #[tabled(rename = "STATUS")]
    pub status: String,
    #[tabled(rename = "TLD")]
    pub tld: String,
    #[tabled(rename = "EXPIRES")]
    pub expire_date: String,
    #[tabled(rename = "AUTO_RENEW")]
    pub auto_renew: String,
}

impl From<&Domain> for DomainRow {
    fn from(d: &Domain) -> Self {
        DomainRow {
            domain: d.domain.clone(),
            status: d.status.clone(),
            tld: d.tld.clone(),
            expire_date: d.expire_date.clone(),
            auto_renew: d.auto_renew.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainAvailability {
    pub avail: bool,
    #[serde(rename = "type")]
    pub avail_type: String,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub first_year_promo: Option<String>,
    #[serde(default)]
    pub regular_price: Option<String>,
    #[serde(default)]
    pub premium: Option<bool>,
    #[serde(default)]
    pub additional: Option<String>,
    #[serde(default)]
    pub min_duration: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainCreate {
    pub domain: String,
    pub cost: String,
    pub order_id: i64,
    pub balance: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_domain() {
        let json = r#"{
            "domain": "example.com",
            "status": "ACTIVE",
            "tld": "com",
            "createDate": "2024-01-01 00:00:00",
            "expireDate": "2025-01-01 00:00:00",
            "securityLock": "1",
            "whoisPrivacy": "1",
            "autoRenew": "1",
            "notLocal": 0
        }"#;
        let domain: Domain = serde_json::from_str(json).unwrap();
        assert_eq!(domain.domain, "example.com");
        assert_eq!(domain.status, "ACTIVE");
        assert!(domain.labels.is_none());
    }

    #[test]
    fn test_deserialize_domain_with_labels() {
        let json = r#"{
            "domain": "example.com",
            "status": "ACTIVE",
            "tld": "com",
            "createDate": "2024-01-01 00:00:00",
            "expireDate": "2025-01-01 00:00:00",
            "securityLock": "1",
            "whoisPrivacy": "1",
            "autoRenew": "1",
            "notLocal": 0,
            "labels": [{"id": "27240", "title": "cool", "color": "#ff9e9e"}]
        }"#;
        let domain: Domain = serde_json::from_str(json).unwrap();
        let labels = domain.labels.unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].title, "cool");
    }

    #[test]
    fn test_deserialize_domain_availability() {
        let json = r#"{
            "avail": true,
            "type": "available",
            "price": "9.73",
            "regularPrice": "10.73",
            "premium": false,
            "minDuration": 1
        }"#;
        let da: DomainAvailability = serde_json::from_str(json).unwrap();
        assert!(da.avail);
        assert_eq!(da.avail_type, "available");
        assert_eq!(da.regular_price, Some("10.73".to_string()));
    }
}
```

Update `porkbun-lib/src/models/mod.rs` — add `pub mod domain;`

- [ ] **Step 2: Run tests**

Run: `cargo test -p porkbun-lib models::domain::tests`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add porkbun-lib/src/models/
git commit -m "feat: add Domain, DomainAvailability, DomainCreate models"
```

### Task 6: Remaining models (pricing, ssl, url_forward, dnssec, glue)

**Files:**
- Create: `porkbun-lib/src/models/pricing.rs`
- Create: `porkbun-lib/src/models/ssl.rs`
- Create: `porkbun-lib/src/models/url_forward.rs`
- Create: `porkbun-lib/src/models/dnssec.rs`
- Create: `porkbun-lib/src/models/glue.rs`
- Modify: `porkbun-lib/src/models/mod.rs`

- [ ] **Step 1: Write pricing model**

`porkbun-lib/src/models/pricing.rs`:

```rust
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TldPricing {
    pub registration: String,
    pub renewal: String,
    pub transfer: String,
}

#[derive(Debug, Tabled)]
pub struct TldPricingRow {
    #[tabled(rename = "TLD")]
    pub tld: String,
    #[tabled(rename = "REGISTRATION")]
    pub registration: String,
    #[tabled(rename = "RENEWAL")]
    pub renewal: String,
    #[tabled(rename = "TRANSFER")]
    pub transfer: String,
}

impl TldPricingRow {
    pub fn from_entry(tld: &str, pricing: &TldPricing) -> Self {
        TldPricingRow {
            tld: tld.to_string(),
            registration: pricing.registration.clone(),
            renewal: pricing.renewal.clone(),
            transfer: pricing.transfer.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_deserialize_pricing_map() {
        let json = r#"{
            "com": {"registration": "9.73", "renewal": "10.73", "transfer": "9.73"},
            "net": {"registration": "10.73", "renewal": "11.73", "transfer": "10.73"}
        }"#;
        let pricing: HashMap<String, TldPricing> = serde_json::from_str(json).unwrap();
        assert_eq!(pricing.len(), 2);
        assert_eq!(pricing["com"].registration, "9.73");
    }
}
```

- [ ] **Step 2: Write ssl model**

`porkbun-lib/src/models/ssl.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SslBundle {
    pub certificatechain: String,
    pub privatekey: String,
    pub publickey: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_ssl_bundle() {
        let json = r#"{
            "certificatechain": "-----BEGIN CERTIFICATE-----\nMIIE...",
            "privatekey": "-----BEGIN PRIVATE KEY-----\nMIIE...",
            "publickey": "-----BEGIN PUBLIC KEY-----\nMIIB..."
        }"#;
        let ssl: SslBundle = serde_json::from_str(json).unwrap();
        assert!(ssl.certificatechain.starts_with("-----BEGIN"));
    }
}
```

- [ ] **Step 3: Write url_forward model**

`porkbun-lib/src/models/url_forward.rs`:

```rust
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlForward {
    pub id: String,
    pub subdomain: String,
    pub location: String,
    #[serde(rename = "type")]
    pub forward_type: String,
    pub include_path: String,
    pub wildcard: String,
}

#[derive(Debug, Tabled)]
pub struct UrlForwardRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "SUBDOMAIN")]
    pub subdomain: String,
    #[tabled(rename = "LOCATION")]
    pub location: String,
    #[tabled(rename = "TYPE")]
    pub forward_type: String,
    #[tabled(rename = "INCLUDE_PATH")]
    pub include_path: String,
}

impl From<&UrlForward> for UrlForwardRow {
    fn from(f: &UrlForward) -> Self {
        UrlForwardRow {
            id: f.id.clone(),
            subdomain: f.subdomain.clone(),
            location: f.location.clone(),
            forward_type: f.forward_type.clone(),
            include_path: f.include_path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_url_forward() {
        let json = r#"{
            "id": "12345",
            "subdomain": "",
            "location": "https://example.com",
            "type": "temporary",
            "includePath": "yes",
            "wildcard": "no"
        }"#;
        let fwd: UrlForward = serde_json::from_str(json).unwrap();
        assert_eq!(fwd.id, "12345");
        assert_eq!(fwd.forward_type, "temporary");
        assert_eq!(fwd.include_path, "yes");
    }
}
```

- [ ] **Step 4: Write dnssec model**

`porkbun-lib/src/models/dnssec.rs`:

```rust
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DnssecRecord {
    pub key_tag: String,
    pub alg: String,
    pub digest_type: String,
    pub digest: String,
}

#[derive(Debug, Tabled)]
pub struct DnssecRecordRow {
    #[tabled(rename = "KEY_TAG")]
    pub key_tag: String,
    #[tabled(rename = "ALG")]
    pub alg: String,
    #[tabled(rename = "DIGEST_TYPE")]
    pub digest_type: String,
    #[tabled(rename = "DIGEST")]
    pub digest: String,
}

impl From<&DnssecRecord> for DnssecRecordRow {
    fn from(r: &DnssecRecord) -> Self {
        DnssecRecordRow {
            key_tag: r.key_tag.clone(),
            alg: r.alg.clone(),
            digest_type: r.digest_type.clone(),
            digest: r.digest.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_deserialize_dnssec_map() {
        let json = r#"{
            "12345": {"keyTag": "12345", "alg": "13", "digestType": "2", "digest": "abc123"}
        }"#;
        let records: HashMap<String, DnssecRecord> = serde_json::from_str(json).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records["12345"].alg, "13");
    }
}
```

- [ ] **Step 5: Write glue model with custom deserialization**

`porkbun-lib/src/models/glue.rs`:

```rust
use serde::Serialize;
use tabled::Tabled;

#[derive(Debug, Serialize)]
pub struct GlueRecord {
    pub hostname: String,
    pub v4: Vec<String>,
    pub v6: Vec<String>,
}

#[derive(Debug, Tabled)]
pub struct GlueRecordRow {
    #[tabled(rename = "HOSTNAME")]
    pub hostname: String,
    #[tabled(rename = "IPV4")]
    pub v4: String,
    #[tabled(rename = "IPV6")]
    pub v6: String,
}

impl From<&GlueRecord> for GlueRecordRow {
    fn from(g: &GlueRecord) -> Self {
        GlueRecordRow {
            hostname: g.hostname.clone(),
            v4: g.v4.join(", "),
            v6: g.v6.join(", "),
        }
    }
}

/// Porkbun returns glue records as: [["hostname", {"v4": [...], "v6": [...]}], ...]
/// This function deserializes that tuple array into Vec<GlueRecord>.
pub fn deserialize_glue_hosts(value: &serde_json::Value) -> Result<Vec<GlueRecord>, String> {
    let arr = value
        .as_array()
        .ok_or("Expected array for hosts")?;

    let mut records = Vec::new();
    for entry in arr {
        let tuple = entry
            .as_array()
            .ok_or("Expected [hostname, {v4, v6}] tuple")?;
        if tuple.len() != 2 {
            return Err("Glue record tuple must have exactly 2 elements".to_string());
        }
        let hostname = tuple[0]
            .as_str()
            .ok_or("Expected string hostname")?
            .to_string();
        let ips = &tuple[1];
        let v4 = ips
            .get("v4")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let v6 = ips
            .get("v6")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        records.push(GlueRecord { hostname, v4, v6 });
    }
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_glue_hosts() {
        let json = serde_json::json!([
            ["ns1.example.com", {"v4": ["1.2.3.4"], "v6": ["::1"]}],
            ["ns2.example.com", {"v4": ["5.6.7.8"], "v6": []}]
        ]);
        let records = deserialize_glue_hosts(&json).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].hostname, "ns1.example.com");
        assert_eq!(records[0].v4, vec!["1.2.3.4"]);
        assert_eq!(records[0].v6, vec!["::1"]);
        assert_eq!(records[1].v6.len(), 0);
    }

    #[test]
    fn test_glue_record_row() {
        let record = GlueRecord {
            hostname: "ns1.example.com".to_string(),
            v4: vec!["1.2.3.4".to_string(), "5.6.7.8".to_string()],
            v6: vec![],
        };
        let row = GlueRecordRow::from(&record);
        assert_eq!(row.v4, "1.2.3.4, 5.6.7.8");
        assert_eq!(row.v6, "");
    }
}
```

- [ ] **Step 6: Update models/mod.rs**

```rust
pub mod dns;
pub mod dnssec;
pub mod domain;
pub mod glue;
pub mod pricing;
pub mod ssl;
pub mod url_forward;
```

- [ ] **Step 7: Run all model tests**

Run: `cargo test -p porkbun-lib models`
Expected: all tests PASS

- [ ] **Step 8: Commit**

```bash
git add porkbun-lib/src/models/
git commit -m "feat: add pricing, ssl, url_forward, dnssec, and glue models"
```

---

## Chunk 4: CLI Scaffold and Output

### Task 7: Output module

**Files:**
- Create: `porkbun/src/output.rs`

- [ ] **Step 1: Write output module**

Copy the output module pattern from updown exactly:

```rust
use tabled::{Table, Tabled};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputMode {
    Table,
    Json,
}

impl OutputMode {
    pub fn from_json_flag(json: bool) -> Self {
        if json {
            OutputMode::Json
        } else {
            OutputMode::Table
        }
    }
}

pub fn print_json(value: &serde_json::Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
    );
}

pub fn print_table<T: Tabled>(items: &[T]) {
    if items.is_empty() {
        println!("No results.");
        return;
    }
    let table = Table::new(items).to_string();
    println!("{}", table);
}

pub fn print_kv(pairs: &[(&str, String)]) {
    let max_key_len = pairs.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    for (key, value) in pairs {
        println!("{:>width$}:  {}", key, value, width = max_key_len);
    }
}

pub fn print_confirm(message: &str) {
    println!("{}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_mode_from_flag() {
        assert_eq!(OutputMode::from_json_flag(true), OutputMode::Json);
        assert_eq!(OutputMode::from_json_flag(false), OutputMode::Table);
    }
}
```

- [ ] **Step 2: Run test**

Run: `cargo test -p porkbun output::tests`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add porkbun/src/output.rs
git commit -m "feat: add output module with table, json, kv, and confirm helpers"
```

### Task 8: CLI main.rs with all commands wired up

**Files:**
- Create: `porkbun/src/main.rs`
- Create: `porkbun/src/cmd/mod.rs`
- Create: `porkbun/src/cmd/ping.rs`
- Create: `porkbun/src/cmd/pricing.rs`
- Create: `porkbun/src/cmd/domains.rs`
- Create: `porkbun/src/cmd/dns.rs`
- Create: `porkbun/src/cmd/dnssec.rs`
- Create: `porkbun/src/cmd/url_forward.rs`
- Create: `porkbun/src/cmd/glue.rs`
- Create: `porkbun/src/cmd/ssl.rs`

- [ ] **Step 1: Write cmd/mod.rs**

```rust
pub mod dns;
pub mod dnssec;
pub mod domains;
pub mod glue;
pub mod ping;
pub mod pricing;
pub mod ssl;
pub mod url_forward;
```

- [ ] **Step 2: Write main.rs with all subcommands**

```rust
mod cmd;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process;
use porkbun_lib::client::Client;
use porkbun_lib::config::Config;

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
        Commands::Pricing { ref action } => unreachable!(),
        Commands::Domains { action } => cmd::domains::run(action, &client, mode),
        Commands::Dns { action } => cmd::dns::run(action, &client, mode),
        Commands::Dnssec { action } => cmd::dnssec::run(action, &client, mode),
        Commands::UrlForward { action } => cmd::url_forward::run(action, &client, mode),
        Commands::Glue { action } => cmd::glue::run(action, &client, mode),
        Commands::Ssl { action } => cmd::ssl::run(action, &client, mode),
    }
}
```

- [ ] **Step 3: Write all command handler stubs**

Each cmd file follows this pattern — define the action enum and a `run()` function. Here are all 8 command modules. Each has the full action enum with correct clap args and a dispatching `run()`:

**cmd/ping.rs:**
```rust
use anyhow::Result;
use crate::output::{self, OutputMode};
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
```

**cmd/pricing.rs:**
```rust
use anyhow::Result;
use clap::Subcommand;
use std::collections::HashMap;
use crate::output::{self, OutputMode};
use porkbun_lib::client::Client;
use porkbun_lib::models::pricing::{TldPricing, TldPricingRow};

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
```

**cmd/domains.rs:**
```rust
use anyhow::Result;
use clap::Subcommand;
use crate::output::{self, OutputMode};
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
        #[arg(long)]
        status: String,
    },
}

pub fn run(action: DomainsAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DomainsAction::List { start, include_labels } => {
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
            let json = client.post_raw(&format!("domain/checkDomain/{}", domain), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let da: DomainAvailability = serde_json::from_value(json["response"].clone())?;
                    output::print_kv(&[
                        ("Available", da.avail.to_string()),
                        ("Type", da.avail_type),
                        ("Price", da.price.unwrap_or("-".to_string())),
                        ("First Year Promo", da.first_year_promo.unwrap_or("-".to_string())),
                        ("Regular Price", da.regular_price.unwrap_or("-".to_string())),
                        ("Premium", da.premium.map(|b| b.to_string()).unwrap_or("-".to_string())),
                        ("Min Duration", da.min_duration.map(|d| d.to_string()).unwrap_or("-".to_string())),
                    ]);
                }
            }
            Ok(())
        }
        DomainsAction::Create { domain, cost, agree_to_terms } => {
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
            let json = client.post_raw(&format!("domain/getNs/{}", domain), serde_json::json!({}))?;
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
                            let status = result.get("status").and_then(|v| v.as_str()).unwrap_or("-");
                            let message = result.get("message").and_then(|v| v.as_str()).unwrap_or("");
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
```

**cmd/dns.rs:**
```rust
use anyhow::Result;
use clap::Subcommand;
use crate::output::{self, OutputMode};
use porkbun_lib::client::Client;
use porkbun_lib::models::dns::{DnsRecord, DnsRecordRow};

#[derive(Subcommand)]
pub enum DnsAction {
    /// Create a DNS record
    Create {
        domain: String,
        #[arg(long)]
        r#type: String,
        #[arg(long)]
        content: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        ttl: Option<String>,
        #[arg(long)]
        prio: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Edit a DNS record by ID
    Edit {
        domain: String,
        id: String,
        #[arg(long)]
        r#type: String,
        #[arg(long)]
        content: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        ttl: Option<String>,
        #[arg(long)]
        prio: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Edit DNS records by name and type
    #[command(name = "edit-by-name-type")]
    EditByNameType {
        domain: String,
        r#type: String,
        subdomain: Option<String>,
        #[arg(long)]
        content: String,
        #[arg(long)]
        ttl: Option<String>,
        #[arg(long)]
        prio: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Delete a DNS record by ID
    Delete {
        domain: String,
        id: String,
    },
    /// Delete DNS records by name and type
    #[command(name = "delete-by-name-type")]
    DeleteByNameType {
        domain: String,
        r#type: String,
        subdomain: Option<String>,
    },
    /// Retrieve DNS records
    Retrieve {
        domain: String,
        id: Option<String>,
    },
    /// Retrieve DNS records by name and type
    #[command(name = "retrieve-by-name-type")]
    RetrieveByNameType {
        domain: String,
        r#type: String,
        subdomain: Option<String>,
    },
}

pub fn run(action: DnsAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DnsAction::Create { domain, r#type, content, name, ttl, prio, notes } => {
            let mut body = serde_json::json!({ "type": r#type, "content": content });
            if let Some(n) = name { body["name"] = serde_json::json!(n); }
            if let Some(t) = ttl { body["ttl"] = serde_json::json!(t); }
            if let Some(p) = prio { body["prio"] = serde_json::json!(p); }
            if let Some(n) = notes { body["notes"] = serde_json::json!(n); }
            let json = client.post_raw(&format!("dns/create/{}", domain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let id = json.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
                    output::print_kv(&[("Record ID", id.to_string())]);
                }
            }
            Ok(())
        }
        DnsAction::Edit { domain, id, r#type, content, name, ttl, prio, notes } => {
            let mut body = serde_json::json!({ "type": r#type, "content": content });
            if let Some(n) = name { body["name"] = serde_json::json!(n); }
            if let Some(t) = ttl { body["ttl"] = serde_json::json!(t); }
            if let Some(p) = prio { body["prio"] = serde_json::json!(p); }
            if let Some(n) = notes { body["notes"] = serde_json::json!(n); }
            let json = client.post_raw(&format!("dns/edit/{}/{}", domain, id), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNS record updated."),
            }
            Ok(())
        }
        DnsAction::EditByNameType { domain, r#type, subdomain, content, ttl, prio, notes } => {
            let sub = subdomain.as_deref().unwrap_or("");
            let mut body = serde_json::json!({ "content": content });
            if let Some(t) = ttl { body["ttl"] = serde_json::json!(t); }
            if let Some(p) = prio { body["prio"] = serde_json::json!(p); }
            if let Some(n) = notes { body["notes"] = serde_json::json!(n); }
            let json = client.post_raw(&format!("dns/editByNameType/{}/{}/{}", domain, r#type, sub), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNS record(s) updated."),
            }
            Ok(())
        }
        DnsAction::Delete { domain, id } => {
            let json = client.post_raw(&format!("dns/delete/{}/{}", domain, id), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNS record deleted."),
            }
            Ok(())
        }
        DnsAction::DeleteByNameType { domain, r#type, subdomain } => {
            let sub = subdomain.as_deref().unwrap_or("");
            let json = client.post_raw(&format!("dns/deleteByNameType/{}/{}/{}", domain, r#type, sub), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNS record(s) deleted."),
            }
            Ok(())
        }
        DnsAction::Retrieve { domain, id } => {
            let path = match id {
                Some(ref i) => format!("dns/retrieve/{}/{}", domain, i),
                None => format!("dns/retrieve/{}", domain),
            };
            let json = client.post_raw(&path, serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let records: Vec<DnsRecord> = serde_json::from_value(json["records"].clone())?;
                    let rows: Vec<DnsRecordRow> = records.iter().map(DnsRecordRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
        DnsAction::RetrieveByNameType { domain, r#type, subdomain } => {
            let sub = subdomain.as_deref().unwrap_or("");
            let json = client.post_raw(&format!("dns/retrieveByNameType/{}/{}/{}", domain, r#type, sub), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let records: Vec<DnsRecord> = serde_json::from_value(json["records"].clone())?;
                    let rows: Vec<DnsRecordRow> = records.iter().map(DnsRecordRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
    }
}
```

**cmd/dnssec.rs:**
```rust
use anyhow::Result;
use clap::Subcommand;
use std::collections::HashMap;
use crate::output::{self, OutputMode};
use porkbun_lib::client::Client;
use porkbun_lib::models::dnssec::{DnssecRecord, DnssecRecordRow};

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
    List {
        domain: String,
    },
    /// Delete a DNSSEC record
    Delete {
        domain: String,
        key_tag: String,
    },
}

pub fn run(action: DnssecAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        DnssecAction::Create { domain, key_tag, alg, digest_type, digest, max_sig_life, key_data_flags, key_data_protocol, key_data_algo, key_data_pub_key } => {
            let mut body = serde_json::json!({
                "keyTag": key_tag,
                "alg": alg,
                "digestType": digest_type,
                "digest": digest,
            });
            if let Some(v) = max_sig_life { body["maxSigLife"] = serde_json::json!(v); }
            if let Some(v) = key_data_flags { body["keyDataFlags"] = serde_json::json!(v); }
            if let Some(v) = key_data_protocol { body["keyDataProtocol"] = serde_json::json!(v); }
            if let Some(v) = key_data_algo { body["keyDataAlgo"] = serde_json::json!(v); }
            if let Some(v) = key_data_pub_key { body["keyDataPubKey"] = serde_json::json!(v); }
            let json = client.post_raw(&format!("dns/createDnssecRecord/{}", domain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNSSEC record created."),
            }
            Ok(())
        }
        DnssecAction::List { domain } => {
            let json = client.post_raw(&format!("dns/getDnssecRecords/{}", domain), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let records: HashMap<String, DnssecRecord> =
                        serde_json::from_value(json["records"].clone())?;
                    let rows: Vec<DnssecRecordRow> = records.values().map(DnssecRecordRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
        DnssecAction::Delete { domain, key_tag } => {
            let json = client.post_raw(&format!("dns/deleteDnssecRecord/{}/{}", domain, key_tag), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("DNSSEC record deleted."),
            }
            Ok(())
        }
    }
}
```

**cmd/url_forward.rs:**
```rust
use anyhow::Result;
use clap::Subcommand;
use crate::output::{self, OutputMode};
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
    List {
        domain: String,
    },
    /// Delete a URL forward
    Delete {
        domain: String,
        record_id: String,
    },
}

pub fn run(action: UrlForwardAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        UrlForwardAction::Add { domain, location, r#type, include_path, wildcard, subdomain } => {
            let mut body = serde_json::json!({
                "location": location,
                "type": r#type,
                "includePath": include_path,
                "wildcard": wildcard,
            });
            if let Some(s) = subdomain { body["subdomain"] = serde_json::json!(s); }
            let json = client.post_raw(&format!("domain/addUrlForward/{}", domain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("URL forward added."),
            }
            Ok(())
        }
        UrlForwardAction::List { domain } => {
            let json = client.post_raw(&format!("domain/getUrlForwarding/{}", domain), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let forwards: Vec<UrlForward> = serde_json::from_value(json["forwards"].clone())?;
                    let rows: Vec<UrlForwardRow> = forwards.iter().map(UrlForwardRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
        UrlForwardAction::Delete { domain, record_id } => {
            let json = client.post_raw(&format!("domain/deleteUrlForward/{}/{}", domain, record_id), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("URL forward deleted."),
            }
            Ok(())
        }
    }
}
```

**cmd/glue.rs:**
```rust
use anyhow::Result;
use clap::Subcommand;
use crate::output::{self, OutputMode};
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
    Delete {
        domain: String,
        subdomain: String,
    },
    /// List glue records
    List {
        domain: String,
    },
}

pub fn run(action: GlueAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        GlueAction::Create { domain, subdomain, ips } => {
            let body = serde_json::json!({ "ips": ips });
            let json = client.post_raw(&format!("domain/createGlue/{}/{}", domain, subdomain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("Glue record created."),
            }
            Ok(())
        }
        GlueAction::Update { domain, subdomain, ips } => {
            let body = serde_json::json!({ "ips": ips });
            let json = client.post_raw(&format!("domain/updateGlue/{}/{}", domain, subdomain), body)?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("Glue record updated."),
            }
            Ok(())
        }
        GlueAction::Delete { domain, subdomain } => {
            let json = client.post_raw(&format!("domain/deleteGlue/{}/{}", domain, subdomain), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => output::print_confirm("Glue record deleted."),
            }
            Ok(())
        }
        GlueAction::List { domain } => {
            let json = client.post_raw(&format!("domain/getGlue/{}", domain), serde_json::json!({}))?;
            match mode {
                OutputMode::Json => output::print_json(&json),
                OutputMode::Table => {
                    let hosts = json.get("hosts").unwrap_or(&serde_json::json!([]));
                    let records = deserialize_glue_hosts(hosts)
                        .map_err(|e| anyhow::anyhow!("Failed to parse glue records: {}", e))?;
                    let rows: Vec<GlueRecordRow> = records.iter().map(GlueRecordRow::from).collect();
                    output::print_table(&rows);
                }
            }
            Ok(())
        }
    }
}
```

**cmd/ssl.rs:**
```rust
use anyhow::Result;
use clap::Subcommand;
use crate::output::{self, OutputMode};
use porkbun_lib::client::Client;
use porkbun_lib::models::ssl::SslBundle;

#[derive(Subcommand)]
pub enum SslAction {
    /// Retrieve SSL certificate bundle
    Retrieve {
        domain: String,
    },
}

pub fn run(action: SslAction, client: &Client, mode: OutputMode) -> Result<()> {
    match action {
        SslAction::Retrieve { domain } => {
            let json = client.post_raw(&format!("ssl/retrieve/{}", domain), serde_json::json!({}))?;
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
```

- [ ] **Step 4: Verify workspace compiles**

Run: `cargo check --workspace`
Expected: compiles with no errors

- [ ] **Step 5: Run clippy**

Run: `cargo clippy --workspace`
Expected: no warnings

- [ ] **Step 6: Commit**

```bash
git add porkbun/src/
git commit -m "feat: add CLI with all 27 command handlers"
```

---

## Chunk 5: Integration Tests

### Task 9: Test infrastructure and fixtures

**Files:**
- Create: `porkbun/tests/common/mod.rs`
- Create: all fixture JSON files under `porkbun/tests/fixtures/`

- [ ] **Step 1: Create common test helpers**

`porkbun/tests/common/mod.rs`:
```rust
use std::path::PathBuf;

#[allow(dead_code)]
pub fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Missing fixture: {}", path.display()))
}

pub fn binary() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("porkbun").unwrap()
}
```

- [ ] **Step 2: Create fixture files**

All fixtures go under `porkbun/tests/fixtures/`. Create each one:

**ping.json:**
```json
{"status": "SUCCESS", "yourIp": "192.0.2.1"}
```

**pricing_get.json:**
```json
{"status": "SUCCESS", "pricing": {"com": {"registration": "9.73", "renewal": "10.73", "transfer": "9.73"}, "net": {"registration": "10.73", "renewal": "11.73", "transfer": "10.73"}}}
```

**domains_list.json:**
```json
{"status": "SUCCESS", "domains": [{"domain": "example.com", "status": "ACTIVE", "tld": "com", "createDate": "2024-01-01 00:00:00", "expireDate": "2025-01-01 00:00:00", "securityLock": "1", "whoisPrivacy": "1", "autoRenew": "1", "notLocal": 0}]}
```

**domain_check.json:**
```json
{"status": "SUCCESS", "response": {"avail": true, "type": "available", "price": "9.73", "regularPrice": "10.73", "premium": false, "minDuration": 1}}
```

**domain_create.json:**
```json
{"status": "SUCCESS", "domain": "newdomain.com", "cost": "973", "orderId": 12345, "balance": "50.00"}
```

**domain_get_ns.json:**
```json
{"status": "SUCCESS", "ns": ["ns1.porkbun.com", "ns2.porkbun.com"]}
```

**dns_retrieve.json:**
```json
{"status": "SUCCESS", "records": [{"id": "326953021", "name": "example.com", "type": "A", "content": "1.2.3.4", "ttl": "600", "prio": "0", "notes": ""}]}
```

**dns_create.json:**
```json
{"status": "SUCCESS", "id": 326953099}
```

**dnssec_list.json:**
```json
{"status": "SUCCESS", "records": {"12345": {"keyTag": "12345", "alg": "13", "digestType": "2", "digest": "abc123def456"}}}
```

**glue_list.json:**
```json
{"status": "SUCCESS", "hosts": [["ns1.example.com", {"v4": ["1.2.3.4"], "v6": ["2001:db8::1"]}]]}
```

**url_forward_list.json:**
```json
{"status": "SUCCESS", "forwards": [{"id": "98765", "subdomain": "", "location": "https://target.com", "type": "permanent", "includePath": "yes", "wildcard": "no"}]}
```

**ssl_retrieve.json:**
```json
{"status": "SUCCESS", "certificatechain": "-----BEGIN CERTIFICATE-----\nMIIE...\n-----END CERTIFICATE-----", "privatekey": "-----BEGIN PRIVATE KEY-----\nMIIE...\n-----END PRIVATE KEY-----", "publickey": "-----BEGIN PUBLIC KEY-----\nMIIB...\n-----END PUBLIC KEY-----"}
```

**domain_update_auto_renew.json:**
```json
{"status": "SUCCESS", "results": {"example.com": {"status": "SUCCESS", "message": "Auto-renew has been enabled."}}}
```

**success.json:**
```json
{"status": "SUCCESS"}
```

- [ ] **Step 3: Commit**

```bash
git add porkbun/tests/
git commit -m "feat: add test helpers and fixture files"
```

### Task 10: Integration tests for all commands

**Files:**
- Create: `porkbun/tests/ping_test.rs`
- Create: `porkbun/tests/pricing_test.rs`
- Create: `porkbun/tests/domains_test.rs`
- Create: `porkbun/tests/dns_test.rs`
- Create: `porkbun/tests/dnssec_test.rs`
- Create: `porkbun/tests/url_forward_test.rs`
- Create: `porkbun/tests/glue_test.rs`
- Create: `porkbun/tests/ssl_test.rs`

All integration tests follow this pattern: start mockito server, mock POST endpoint, run CLI binary with `PORKBUN_BASE_URL` pointing to mock, assert output. Porkbun uses POST for everything, so mocks use `.mock("POST", ...)` and match JSON body for auth keys.

- [ ] **Step 1: Write ping tests**

`porkbun/tests/ping_test.rs`:
```rust
mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_ping_table() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/ping")
        .with_body(common::fixture("ping.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "ping"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("192.0.2.1"));

    mock.assert();
}

#[test]
fn test_ping_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/ping")
        .with_body(common::fixture("ping.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "--json", "ping"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"yourIp\""));

    mock.assert();
}

#[test]
fn test_ping_auth_error() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/ping")
        .with_body(r#"{"status": "ERROR", "message": "Invalid API key"}"#)
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "bad", "--secret-api-key", "bad", "ping"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid API key"));

    mock.assert();
}
```

- [ ] **Step 2: Write pricing tests**

`porkbun/tests/pricing_test.rs`:
```rust
mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_pricing_get_table() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pricing/get")
        .with_body(common::fixture("pricing_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["pricing", "get"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("com"))
        .stdout(predicate::str::contains("9.73"));

    mock.assert();
}

#[test]
fn test_pricing_get_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pricing/get")
        .with_body(common::fixture("pricing_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--json", "pricing", "get"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"pricing\""));

    mock.assert();
}

#[test]
fn test_pricing_get_no_auth_required() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pricing/get")
        .with_body(common::fixture("pricing_get.json"))
        .with_header("content-type", "application/json")
        .create();

    // No --api-key or --secret-api-key flags — should still work
    let mut cmd = common::binary();
    cmd.args(["pricing", "get"])
        .env("PORKBUN_BASE_URL", server.url())
        .env_remove("PORKBUN_API_KEY")
        .env_remove("PORKBUN_SECRET_API_KEY")
        .assert()
        .success();

    mock.assert();
}
```

- [ ] **Step 3: Write domains tests**

`porkbun/tests/domains_test.rs`:
```rust
mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_domains_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/listAll")
        .with_body(common::fixture("domains_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "domains", "list"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("example.com"))
        .stdout(predicate::str::contains("ACTIVE"));

    mock.assert();
}

#[test]
fn test_domains_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/listAll")
        .with_body(common::fixture("domains_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "--json", "domains", "list"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"domains\""));

    mock.assert();
}

#[test]
fn test_domains_check() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/checkDomain/newdomain.com")
        .with_body(common::fixture("domain_check.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "domains", "check", "newdomain.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("true"))
        .stdout(predicate::str::contains("9.73"));

    mock.assert();
}

#[test]
fn test_domains_get_ns() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/getNs/example.com")
        .with_body(common::fixture("domain_get_ns.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "domains", "get-ns", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("ns1.porkbun.com"))
        .stdout(predicate::str::contains("ns2.porkbun.com"));

    mock.assert();
}

#[test]
fn test_domains_update_ns() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/updateNs/example.com")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "domains", "update-ns", "example.com", "--ns", "ns1.custom.com", "ns2.custom.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Name servers updated"));

    mock.assert();
}

#[test]
fn test_domains_update_auto_renew() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/updateAutoRenew/example.com")
        .with_body(common::fixture("domain_update_auto_renew.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "domains", "update-auto-renew", "example.com", "--status", "on"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("example.com"));

    mock.assert();
}
```

- [ ] **Step 4: Write dns tests**

`porkbun/tests/dns_test.rs`:
```rust
mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_dns_retrieve_table() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/retrieve/example.com")
        .with_body(common::fixture("dns_retrieve.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "dns", "retrieve", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("326953021"))
        .stdout(predicate::str::contains("1.2.3.4"));

    mock.assert();
}

#[test]
fn test_dns_retrieve_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/retrieve/example.com")
        .with_body(common::fixture("dns_retrieve.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "--json", "dns", "retrieve", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"records\""));

    mock.assert();
}

#[test]
fn test_dns_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/create/example.com")
        .with_body(common::fixture("dns_create.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "dns", "create", "example.com", "--type", "A", "--content", "1.2.3.4"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("326953099"));

    mock.assert();
}

#[test]
fn test_dns_edit() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/edit/example.com/123")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "dns", "edit", "example.com", "123", "--type", "A", "--content", "5.6.7.8"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("updated"));

    mock.assert();
}

#[test]
fn test_dns_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/delete/example.com/123")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "dns", "delete", "example.com", "123"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    mock.assert();
}
```

Additional dns tests for full coverage — append to `porkbun/tests/dns_test.rs`:

```rust
#[test]
fn test_dns_retrieve_by_name_type() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/retrieveByNameType/example.com/A/www")
        .with_body(common::fixture("dns_retrieve.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "dns", "retrieve-by-name-type", "example.com", "A", "www"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("1.2.3.4"));

    mock.assert();
}

#[test]
fn test_dns_edit_by_name_type() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/editByNameType/example.com/A/www")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "dns", "edit-by-name-type", "example.com", "A", "www", "--content", "9.8.7.6"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("updated"));

    mock.assert();
}

#[test]
fn test_dns_delete_by_name_type() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/deleteByNameType/example.com/A/www")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "dns", "delete-by-name-type", "example.com", "A", "www"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    mock.assert();
}
```

Additional domains test — append to `porkbun/tests/domains_test.rs`:

```rust
#[test]
fn test_domains_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/create/newdomain.com")
        .with_body(common::fixture("domain_create.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "domains", "create", "newdomain.com", "--cost", "973", "--agree-to-terms"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("newdomain.com"))
        .stdout(predicate::str::contains("12345"));

    mock.assert();
}

#[test]
fn test_domains_check_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/checkDomain/newdomain.com")
        .with_body(common::fixture("domain_check.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "--json", "domains", "check", "newdomain.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"response\""));

    mock.assert();
}

#[test]
fn test_domains_get_ns_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/getNs/example.com")
        .with_body(common::fixture("domain_get_ns.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "--json", "domains", "get-ns", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ns\""));

    mock.assert();
}
```

Additional glue test — append to `porkbun/tests/glue_test.rs`:

```rust
#[test]
fn test_glue_update() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/updateGlue/example.com/ns1")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "glue", "update", "example.com", "ns1", "--ips", "9.8.7.6"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("updated"));

    mock.assert();
}

#[test]
fn test_glue_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/getGlue/example.com")
        .with_body(common::fixture("glue_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "--json", "glue", "list", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"hosts\""));

    mock.assert();
}
```

Additional JSON output tests — append to `porkbun/tests/dnssec_test.rs`:

```rust
#[test]
fn test_dnssec_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/getDnssecRecords/example.com")
        .with_body(common::fixture("dnssec_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "--json", "dnssec", "list", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"records\""));

    mock.assert();
}
```

Additional JSON output test — append to `porkbun/tests/url_forward_test.rs`:

```rust
#[test]
fn test_url_forward_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/getUrlForwarding/example.com")
        .with_body(common::fixture("url_forward_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "--json", "url-forward", "list", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"forwards\""));

    mock.assert();
}
```

- [ ] **Step 5: Write remaining test files (dnssec, url_forward, glue, ssl)**

`porkbun/tests/dnssec_test.rs`:
```rust
mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_dnssec_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/getDnssecRecords/example.com")
        .with_body(common::fixture("dnssec_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "dnssec", "list", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("12345"));

    mock.assert();
}

#[test]
fn test_dnssec_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/createDnssecRecord/example.com")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "dnssec", "create", "example.com", "--key-tag", "12345", "--alg", "13", "--digest-type", "2", "--digest", "abc123"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("created"));

    mock.assert();
}

#[test]
fn test_dnssec_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/deleteDnssecRecord/example.com/12345")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "dnssec", "delete", "example.com", "12345"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    mock.assert();
}
```

`porkbun/tests/url_forward_test.rs`:
```rust
mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_url_forward_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/getUrlForwarding/example.com")
        .with_body(common::fixture("url_forward_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "url-forward", "list", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("98765"))
        .stdout(predicate::str::contains("target.com"));

    mock.assert();
}

#[test]
fn test_url_forward_add() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/addUrlForward/example.com")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "url-forward", "add", "example.com", "--location", "https://target.com", "--type", "permanent", "--include-path", "yes", "--wildcard", "no"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("added"));

    mock.assert();
}

#[test]
fn test_url_forward_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/deleteUrlForward/example.com/98765")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "url-forward", "delete", "example.com", "98765"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    mock.assert();
}
```

`porkbun/tests/glue_test.rs`:
```rust
mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_glue_list_table() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/getGlue/example.com")
        .with_body(common::fixture("glue_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "glue", "list", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("ns1.example.com"))
        .stdout(predicate::str::contains("1.2.3.4"));

    mock.assert();
}

#[test]
fn test_glue_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/createGlue/example.com/ns1")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "glue", "create", "example.com", "ns1", "--ips", "1.2.3.4", "5.6.7.8"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("created"));

    mock.assert();
}

#[test]
fn test_glue_delete() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/deleteGlue/example.com/ns1")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "glue", "delete", "example.com", "ns1"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    mock.assert();
}
```

`porkbun/tests/ssl_test.rs`:
```rust
mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_ssl_retrieve_table() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/ssl/retrieve/example.com")
        .with_body(common::fixture("ssl_retrieve.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "ssl", "retrieve", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("BEGIN CERTIFICATE"));

    mock.assert();
}

#[test]
fn test_ssl_retrieve_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/ssl/retrieve/example.com")
        .with_body(common::fixture("ssl_retrieve.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--api-key", "pk1_test", "--secret-api-key", "sk1_test", "--json", "ssl", "retrieve", "example.com"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"certificatechain\""));

    mock.assert();
}
```

- [ ] **Step 6: Run all integration tests**

Run: `cargo test --workspace`
Expected: all tests PASS

- [ ] **Step 7: Run clippy**

Run: `cargo clippy --workspace`
Expected: no warnings (fix any that appear)

- [ ] **Step 8: Run fmt**

Run: `cargo fmt --all`

- [ ] **Step 9: Commit**

```bash
git add porkbun/tests/
git commit -m "test: add integration tests for all commands with mockito"
```

---

## Chunk 6: Live Tests and README

### Task 11: Live tests

**Files:**
- Create: `porkbun/tests/live_test.rs`

- [ ] **Step 1: Write live tests**

`porkbun/tests/live_test.rs`:
```rust
//! Live integration tests against the Porkbun API.
//!
//! Gated behind PORKBUN_LIVE_TEST=1. Requires PORKBUN_API_KEY and
//! PORKBUN_SECRET_API_KEY set. Domain-specific tests require PORKBUN_TEST_DOMAIN.
//!
//! Run: PORKBUN_LIVE_TEST=1 PORKBUN_TEST_DOMAIN=yourdomain.com cargo test --test live_test -- --test-threads=1

mod common;

use serial_test::serial;

fn require_live() {
    if std::env::var("PORKBUN_LIVE_TEST").unwrap_or_default() != "1" {
        eprintln!("Skipping live test (set PORKBUN_LIVE_TEST=1 to enable)");
        std::process::exit(0);
    }
}

fn test_domain() -> String {
    std::env::var("PORKBUN_TEST_DOMAIN")
        .expect("PORKBUN_TEST_DOMAIN must be set for domain-specific live tests")
}

fn run_json(args: &[&str]) -> serde_json::Value {
    let mut cmd = common::binary();
    let output = cmd
        .arg("--json")
        .args(args)
        .output()
        .expect("failed to execute binary");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "Command failed: {:?}\nstdout: {}\nstderr: {}",
        args, stdout, stderr
    );
    serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("Invalid JSON from {:?}: {}\nstdout: {}", args, e, stdout))
}

fn run_ok(args: &[&str]) -> String {
    let mut cmd = common::binary();
    let output = cmd.args(args).output().expect("failed to execute binary");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "Command failed: {:?}\nstdout: {}\nstderr: {}",
        args, stdout, stderr
    );
    stdout
}

#[test]
#[serial]
fn live_ping() {
    require_live();
    let json = run_json(&["ping"]);
    assert_eq!(json.get("status").and_then(|v| v.as_str()), Some("SUCCESS"));
    assert!(json.get("yourIp").is_some());
}

#[test]
#[serial]
fn live_pricing_get() {
    require_live();
    let json = run_json(&["pricing", "get"]);
    assert!(json.get("pricing").is_some());
    let pricing = json["pricing"].as_object().unwrap();
    assert!(pricing.contains_key("com"));
}

#[test]
#[serial]
fn live_domains_list() {
    require_live();
    let json = run_json(&["domains", "list"]);
    assert!(json.get("domains").and_then(|v| v.as_array()).is_some());
}

#[test]
#[serial]
fn live_domains_get_ns() {
    require_live();
    let domain = test_domain();
    let json = run_json(&["domains", "get-ns", &domain]);
    assert!(json.get("ns").and_then(|v| v.as_array()).is_some());
}

#[test]
#[serial]
fn live_dns_retrieve() {
    require_live();
    let domain = test_domain();
    let json = run_json(&["dns", "retrieve", &domain]);
    assert!(json.get("records").and_then(|v| v.as_array()).is_some());
}

#[test]
#[serial]
fn live_dns_retrieve_by_name_type() {
    require_live();
    let domain = test_domain();
    // Retrieve A records — most domains have at least one
    let json = run_json(&["dns", "retrieve-by-name-type", &domain, "A"]);
    assert!(json.get("records").is_some());
}

#[test]
#[serial]
fn live_ssl_retrieve() {
    require_live();
    let domain = test_domain();
    let json = run_json(&["ssl", "retrieve", &domain]);
    assert!(json.get("certificatechain").is_some());
}

#[test]
#[serial]
fn live_url_forward_list() {
    require_live();
    let domain = test_domain();
    // This may return an error if URL forwarding is not configured — that's OK
    let _ = run_json(&["url-forward", "list", &domain]);
}

#[test]
#[serial]
fn live_glue_list() {
    require_live();
    let domain = test_domain();
    let _ = run_json(&["glue", "list", &domain]);
}

#[test]
#[serial]
fn live_dnssec_list() {
    require_live();
    let domain = test_domain();
    let _ = run_json(&["dnssec", "list", &domain]);
}
```

- [ ] **Step 2: Verify live tests compile (don't run them)**

Run: `cargo test --test live_test --no-run`
Expected: compiles

- [ ] **Step 3: Commit**

```bash
git add porkbun/tests/live_test.rs
git commit -m "test: add live integration tests (read-only, gated by PORKBUN_LIVE_TEST)"
```

### Task 12: README

**Files:**
- Create: `README.md`

- [ ] **Step 1: Write README**

Follow the exact section structure from updown's README, adapted for porkbun. Sections:

1. Brief description
2. Install (from source, with Nix)
3. Configuration (three-source priority chain, both keys)
4. Usage (all commands with examples grouped by resource)
5. Output formats (table default, --json)
6. API Coverage table
7. Development (cargo test, clippy, fmt)
8. Live Testing (env vars required)
9. License (dual MIT/Apache-2.0)

- [ ] **Step 2: Commit**

```bash
git add README.md
git commit -m "docs: add README with install, configuration, usage, and development docs"
```

### Task 13: Final verification

- [ ] **Step 1: Full test suite**

Run: `cargo test --workspace`
Expected: all tests PASS

- [ ] **Step 2: Clippy clean**

Run: `cargo clippy --workspace`
Expected: no warnings

- [ ] **Step 3: Fmt check**

Run: `cargo fmt --all -- --check`
Expected: no formatting issues

- [ ] **Step 4: Build release binary**

Run: `cargo build --release`
Expected: builds successfully, binary at `target/release/porkbun`
