# Porkbun CLI Design Spec

## Overview

A Rust CLI and library for the Porkbun domain registrar API v3. Follows the same architecture as the orangerabbit-io/forwardemail and orangerabbit-io/updown projects: two-crate Cargo workspace, blocking HTTP client, clap-derive CLI, tabled output, mockito tests, and live read-only tests.

Covers 27 of 29 Porkbun API endpoints (the 2 OAuth endpoints are excluded — this is a direct-key CLI).

## Project Structure

Two-crate Cargo workspace:

```
porkbun/
├── porkbun-lib/                 # Library crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs               # Exports client, config, models
│       ├── client.rs            # HTTP client (POST-only, dual-key auth)
│       ├── config.rs            # Config loading (flag > env > file)
│       └── models/
│           ├── mod.rs
│           ├── dns.rs           # DnsRecord, DnsRecordRow
│           ├── dnssec.rs        # DnssecRecord, DnssecRecordRow
│           ├── domain.rs        # Domain, DomainRow, DomainAvailability, DomainCreate
│           ├── glue.rs          # GlueRecord, GlueRecordRow
│           ├── pricing.rs       # TldPricing, TldPricingRow
│           ├── ssl.rs           # SslBundle
│           └── url_forward.rs   # UrlForward, UrlForwardRow
├── porkbun/                     # CLI crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs              # Clap CLI, Commands enum
│       ├── output.rs            # Table/JSON output helpers
│       └── cmd/
│           ├── mod.rs
│           ├── ping.rs
│           ├── pricing.rs
│           ├── domains.rs
│           ├── dns.rs
│           ├── dnssec.rs
│           ├── url_forward.rs
│           ├── glue.rs
│           └── ssl.rs
├── Cargo.toml                   # Workspace manifest
├── flake.nix
├── .releaserc.json
├── package.json
├── LICENSE-MIT
├── LICENSE-APACHE
├── README.md
└── .github/workflows/release.yml
```

## Authentication & Client

### Configuration Priority

1. `--api-key` + `--secret-api-key` CLI flags (highest)
2. `PORKBUN_API_KEY` + `PORKBUN_SECRET_API_KEY` environment variables
3. `~/.config/porkbun/config.toml` file (lowest)

Config file format:

```toml
api_key = "pk1_..."
secret_api_key = "sk1_..."
```

Base URL defaults to `https://api.porkbun.com/api/json/v3` with `PORKBUN_BASE_URL` env var override for testing.

`Config::load` signature: `pub fn load(api_key: Option<&str>, secret_api_key: Option<&str>) -> Result<Config>`. Takes both optional flag overrides. Returns a `Config` struct with `api_key`, `secret_api_key`, and `base_url` fields.

### Client Design

Porkbun is all-POST with auth credentials in the JSON body (not headers).

```rust
pub struct Client {
    http: reqwest::blocking::Client,
    base_url: String,
    api_key: String,
    secret_api_key: String,
}
```

Key methods:

- `post<T: DeserializeOwned>(&self, path: &str, body: Value) -> Result<T>` — Authenticated POST, merges auth keys into body, checks response status field.
- `post_raw(&self, path: &str, body: Value) -> Result<Value>` — Authenticated POST returning raw JSON Value.
- `post_unauthenticated<T: DeserializeOwned>(&self, path: &str, body: Value) -> Result<T>` — For pricing/get (no auth required).

Error handling:

- Porkbun returns HTTP 200 with `{"status": "ERROR", "message": "..."}` for application errors. The client checks the `status` field and bails with the `message`.
- HTTP 403 indicates two-factor auth required.
- Non-200 HTTP status codes are handled as transport errors.

## CLI Commands

Global flags: `--json` (JSON output), `--api-key <KEY>`, `--secret-api-key <KEY>`.

### ping

| Command | Endpoint |
|---|---|
| `porkbun ping` | `/ping` |

### pricing

| Command | Endpoint | Auth |
|---|---|---|
| `porkbun pricing get` | `/pricing/get` | no |

### domains

| Command | Endpoint |
|---|---|
| `porkbun domains list [--start <N>] [--include-labels]` | `/domain/listAll` |
| `porkbun domains check <DOMAIN>` | `/domain/checkDomain/{DOMAIN}` |
| `porkbun domains create <DOMAIN> --cost <PENNIES> [--agree-to-terms]` | `/domain/create/{DOMAIN}` |
| `porkbun domains get-ns <DOMAIN>` | `/domain/getNs/{DOMAIN}` |
| `porkbun domains update-ns <DOMAIN> --ns <NS>...` | `/domain/updateNs/{DOMAIN}` |
| `porkbun domains update-auto-renew <DOMAIN> --status <on\|off>` | `/domain/updateAutoRenew/{DOMAIN}` |

### dns

| Command | Endpoint |
|---|---|
| `porkbun dns create <DOMAIN> --type <TYPE> --content <CONTENT> [--name <SUB>] [--ttl <TTL>] [--prio <PRIO>] [--notes <NOTES>]` | `/dns/create/{DOMAIN}` |
| `porkbun dns edit <DOMAIN> <ID> --type <TYPE> --content <CONTENT> [--name <SUB>] [--ttl <TTL>] [--prio <PRIO>] [--notes <NOTES>]` | `/dns/edit/{DOMAIN}/{ID}` |
| `porkbun dns edit-by-name-type <DOMAIN> <TYPE> [SUBDOMAIN] --content <CONTENT> [--ttl <TTL>] [--prio <PRIO>] [--notes <NOTES>]` | `/dns/editByNameType/{DOMAIN}/{TYPE}/{SUBDOMAIN}` |
| `porkbun dns delete <DOMAIN> <ID>` | `/dns/delete/{DOMAIN}/{ID}` |
| `porkbun dns delete-by-name-type <DOMAIN> <TYPE> [SUBDOMAIN]` | `/dns/deleteByNameType/{DOMAIN}/{TYPE}/{SUBDOMAIN}` |
| `porkbun dns retrieve <DOMAIN> [ID]` | `/dns/retrieve/{DOMAIN}/{ID}` |
| `porkbun dns retrieve-by-name-type <DOMAIN> <TYPE> [SUBDOMAIN]` | `/dns/retrieveByNameType/{DOMAIN}/{TYPE}/{SUBDOMAIN}` |

### dnssec

| Command | Endpoint |
|---|---|
| `porkbun dnssec create <DOMAIN> --key-tag <TAG> --alg <ALG> --digest-type <TYPE> --digest <DIGEST> [--max-sig-life <N>] [--key-data-flags <F>] [--key-data-protocol <P>] [--key-data-algo <A>] [--key-data-pub-key <K>]` | `/dns/createDnssecRecord/{DOMAIN}` |

DNSSEC create request body uses flat keys (not nested): `{"secretapikey": "...", "apikey": "...", "keyTag": "...", "alg": "...", "digestType": "...", "digest": "...", "maxSigLife": "...", "keyDataFlags": "...", "keyDataProtocol": "...", "keyDataAlgo": "...", "keyDataPubKey": "..."}`. All values are strings. The CLI flags map directly to camelCase body keys.
| `porkbun dnssec list <DOMAIN>` | `/dns/getDnssecRecords/{DOMAIN}` |
| `porkbun dnssec delete <DOMAIN> <KEY_TAG>` | `/dns/deleteDnssecRecord/{DOMAIN}/{KEYTAG}` |

### url-forward

| Command | Endpoint |
|---|---|
| `porkbun url-forward add <DOMAIN> --location <URL> --type <temporary\|permanent> --include-path <yes\|no> --wildcard <yes\|no> [--subdomain <SUB>]` | `/domain/addUrlForward/{DOMAIN}` |
| `porkbun url-forward list <DOMAIN>` | `/domain/getUrlForwarding/{DOMAIN}` |
| `porkbun url-forward delete <DOMAIN> <RECORD_ID>` | `/domain/deleteUrlForward/{DOMAIN}/{ID}` |

### glue

| Command | Endpoint |
|---|---|
| `porkbun glue create <DOMAIN> <SUBDOMAIN> --ips <IP>...` | `/domain/createGlue/{DOMAIN}/{SUB}` |
| `porkbun glue update <DOMAIN> <SUBDOMAIN> --ips <IP>...` | `/domain/updateGlue/{DOMAIN}/{SUB}` |
| `porkbun glue delete <DOMAIN> <SUBDOMAIN>` | `/domain/deleteGlue/{DOMAIN}/{SUB}` |
| `porkbun glue list <DOMAIN>` | `/domain/getGlue/{DOMAIN}` |

### ssl

| Command | Endpoint |
|---|---|
| `porkbun ssl retrieve <DOMAIN>` | `/ssl/retrieve/{DOMAIN}` |

## Models

Each model has an API struct (serde Deserialize) and a Row struct (tabled::Tabled) with `From<&ApiStruct> for Row`.

### DnsRecord

Fields: id (String), name (String), record_type (String, `#[serde(rename = "type")]`), content (String), ttl (String), prio (Option\<String\>), notes (Option\<String\>). Row shows: ID, NAME, TYPE, CONTENT, TTL, PRIO. Response key: `"records"` → `Vec<DnsRecord>`.

### DnssecRecord

Fields: key_tag (String), alg (String), digest_type (String), digest (String). All strings. Row shows: KEY_TAG, ALG, DIGEST_TYPE, DIGEST. Response key: `"records"` → `HashMap<String, DnssecRecord>` keyed by key tag.

### Domain

Fields: domain (String), status (String), tld (String), create_date (String), expire_date (String), security_lock (String), whois_privacy (String), auto_renew (String), not_local (i64), labels (Option\<Vec\<Label\>\>). Row shows: DOMAIN, STATUS, TLD, EXPIRES, AUTO_RENEW. Response key: `"domains"` → `Vec<Domain>`.

### Label

Fields: id (String), title (String), color (String). Used within `Domain.labels`.

### DomainAvailability

Fields: avail (bool), avail_type (String, `#[serde(rename = "type")]` — "available"/"unavailable"), price (Option\<String\>), first_year_promo (Option\<String\>), regular_price (Option\<String\>), premium (Option\<bool\>), additional (Option\<String\>), min_duration (Option\<i64\>). Displayed as key-value pairs. Response key: nested under `"response"` in raw Value.

### DomainCreate

Fields: domain (String), cost (String), order_id (i64), balance (String). Displayed as key-value pairs. Response keys at top level alongside `"status"`.

### GlueRecord

Fields: hostname (String), v4 (Vec\<String\>), v6 (Vec\<String\>). Row shows: HOSTNAME, IPV4, IPV6.

Custom deserialization required: Porkbun returns `[["hostname", {"v4": [...], "v6": [...]}], ...]` tuple array format. Response key: `"hosts"` → custom deser into `Vec<GlueRecord>`.

### TldPricing

Fields: registration (String), renewal (String), transfer (String). Row shows: TLD, REGISTRATION, RENEWAL, TRANSFER. TLD comes from the map key in the API response, not the struct. Response key: `"pricing"` → `HashMap<String, TldPricing>`.

### SslBundle

Fields: certificatechain (String), privatekey (String), publickey (String). Displayed as key-value pairs (PEM values). This struct does NOT use `rename_all = "camelCase"` — field names match API verbatim (all lowercase, no separators). Response keys at top level alongside `"status"`.

### UrlForward

Fields: id (String), subdomain (String), location (String), forward_type (String, `#[serde(rename = "type")]`), include_path (String), wildcard (String). Row shows: ID, SUBDOMAIN, LOCATION, TYPE, INCLUDE_PATH. Response key: `"forwards"` → `Vec<UrlForward>`.

### DnsCreateResponse

Fields: id (i64). Returned from `dns create`. Displayed as key-value: "Record ID: {id}".

### Serde Conventions

All Porkbun API fields are camelCase. All model structs use `#[serde(rename_all = "camelCase")]`. Fields like `create_date`, `expire_date`, `security_lock`, `whois_privacy`, `auto_renew`, `not_local`, `first_year_promo`, `regular_price`, `min_duration`, `key_tag`, `digest_type`, `include_path`, `order_id` are automatically handled by this rename.

### Response Deserialization Strategy

Porkbun API responses vary in structure. Rather than a single generic `ApiResponse<T>` wrapper, the client handles responses in two ways:

1. **`post_raw` returning `Value`** — The client parses the response as `serde_json::Value`, checks `status == "SUCCESS"`, and returns the full Value. Used by command handlers that need to extract specific nested keys (`response`, `records`, `domains`, `hosts`, `forwards`, `ns`, `pricing`, `limits`, etc.).

2. **Direct deserialization** — For simple responses where the interesting data is at a known key, the command handler extracts the relevant field from the raw Value and deserializes it (e.g., `value["records"]` into `Vec<DnsRecord>`, `value["domains"]` into `Vec<Domain>`, `value["pricing"]` into `HashMap<String, TldPricing>`).

This avoids the `#[serde(flatten)]` problem where Porkbun nests data under varying keys (`response`, `records`, `domains`, `hosts`, `forwards`, `ns`, `pricing`). Each command handler knows its own response shape.

### Output Behavior

**`--json` flag**: When `--json` is passed, ALL commands print the raw API JSON response via `print_json`. This applies to read commands (list, retrieve, get) and mutation commands alike. When `--json` is NOT passed:

- **Read commands** (list, retrieve, get): print tabled output or key-value pairs as specified per model.
- **Mutation commands** that return only `{"status": "SUCCESS"}` (dns edit, dns delete, update-ns, glue create/update/delete, url-forward add/delete, dnssec create/delete): print a confirmation message via `print_confirm` (e.g., "DNS record deleted.").
- **`domains update-auto-renew`**: prints the `results` map as key-value pairs per domain. Response shape: `{"status": "SUCCESS", "results": {"example.com": {"status": "SUCCESS", "message": "..."}}}`. Extract `"results"` as `HashMap<String, Value>`, print each domain with its status/message.
- **`dns create`**: prints "Record ID: {id}".
- **`domains create`**: prints key-value pairs from `DomainCreate`.
- **`ping`**: prints key-value pair "Your IP: {yourIp}". Response key: `"yourIp"` at top level.
- **`domains get-ns`**: prints nameservers one per line. Response key: `"ns"` → `Vec<String>`.
- **`domains check`**: extracts `value["response"]` → deserializes into `DomainAvailability` → prints key-value pairs.

### Domain List Details

`--start <N>` maps to the `"start"` key in the JSON request body (integer, pagination offset). `--include-labels` maps to `"includeLabels": "yes"` in the request body. The `DomainRow` table always shows DOMAIN, STATUS, TLD, EXPIRES, AUTO_RENEW — labels are visible only in `--json` output. This keeps the table clean while making label data accessible.

### Pricing Command Auth

The `pricing get` command does not require authentication. The CLI handles this by constructing the client with `base_url` only (no auth keys) when the command is `pricing get`. Config loading is skipped for this command — only `PORKBUN_BASE_URL` is read. The `post_unauthenticated` method sends an empty JSON body.

## Testing Strategy

### Unit Tests (inline in porkbun-lib)

- Model deserialization from JSON fixtures, especially:
  - Glue record tuple format
  - Pricing map keyed by TLD
  - ApiResponse SUCCESS/ERROR parsing
- Config priority logic with `#[serial]` for env var mutation tests

### Integration Tests (mockito, in porkbun/tests/)

One test file per command module. Each test:

1. Starts a mockito server
2. Mocks the POST endpoint, matching expected JSON body (including auth keys)
3. Runs the CLI binary via `assert_cmd` with `PORKBUN_BASE_URL` pointing at the mock
4. Asserts stdout for expected table or JSON output
5. Verifies the mock was hit

Test files:

```
porkbun/tests/
├── common/mod.rs
├── fixtures/
│   ├── ping.json
│   ├── pricing_get.json
│   ├── domains_list.json
│   ├── domain_check.json
│   ├── domain_create.json
│   ├── domain_get_ns.json
│   ├── dns_retrieve.json
│   ├── dns_create.json
│   ├── dnssec_list.json
│   ├── glue_list.json
│   ├── url_forward_list.json
│   ├── ssl_retrieve.json
│   ├── domain_update_auto_renew.json
│   └── success.json
├── ping_test.rs
├── pricing_test.rs
├── domains_test.rs
├── dns_test.rs
├── dnssec_test.rs
├── url_forward_test.rs
├── glue_test.rs
├── ssl_test.rs
└── live_test.rs
```

Both `--json` and table output tested for list/retrieve commands. Mutation commands verify request body parameters.

### Live Tests (gated by `PORKBUN_LIVE_TEST=1`)

Read-only operations only, single-threaded (`--test-threads=1`). Requires `PORKBUN_TEST_DOMAIN` env var for domain-specific endpoints.

Endpoints tested live:

- `ping`
- `pricing get`
- `domains list`
- `domains get-ns` (uses PORKBUN_TEST_DOMAIN)
- `dns retrieve` (uses PORKBUN_TEST_DOMAIN)
- `dns retrieve-by-name-type` (uses PORKBUN_TEST_DOMAIN)
- `ssl retrieve` (uses PORKBUN_TEST_DOMAIN)
- `url-forward list` (uses PORKBUN_TEST_DOMAIN)
- `glue list` (uses PORKBUN_TEST_DOMAIN)
- `dnssec list` (uses PORKBUN_TEST_DOMAIN)

## Infrastructure

### Dependencies

**porkbun-lib:**

- anyhow 1
- reqwest 0.12 (blocking, json, gzip)
- serde 1 (derive)
- serde_json 1
- tabled 0.17
- toml 0.8
- serial_test 3 (dev)

**porkbun (CLI):**

- porkbun-lib (path)
- anyhow 1
- clap 4 (derive)
- serde_json 1
- tabled 0.17
- mockito 1 (dev)
- assert_cmd 2 (dev)
- predicates 3 (dev)
- serial_test 3 (dev)

### Nix

flake.nix with `rustPlatform.buildRustPackage`, openssl/pkg-config build inputs, devShell with rustc/cargo/clippy/rustfmt/pkg-config/openssl.

### Semantic Release

.releaserc.json: commit-analyzer, release-notes-generator, changelog, exec (update version in both Cargo.toml + flake.nix + regenerate Cargo.lock), git (commit assets), github (create release).

package.json: private, devDependencies for semantic-release plugins.

### CI/CD

.github/workflows/release.yml: push to main, checkout with full history, Node 22, stable Rust, npm install, npx semantic-release.

### Licensing

Dual MIT OR Apache-2.0. LICENSE-MIT and LICENSE-APACHE files, Orange Rabbit 2025.

### README

Sections: Install (source + nix), Configuration (priority chain), Usage (all commands with examples), Output formats, API Coverage table, Development (test/clippy/fmt), Live Testing, License.
