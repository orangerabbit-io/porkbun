# porkbun

A command-line interface for the [Porkbun](https://porkbun.com) domain registrar API.

## Install

### From source

```sh
git clone https://github.com/orangerabbit-io/porkbun.git
cd porkbun
cargo build --release
cp target/release/porkbun ~/.local/bin/
```

### With Nix

```sh
nix run github:orangerabbit-io/porkbun
```

## Configuration

Create `~/.config/porkbun/config.toml`:

```toml
api_key = "pk1_..."
secret_api_key = "sk1_..."
```

Get your API keys from the [Porkbun API access page](https://porkbun.com/account/api).

Alternatively, set `PORKBUN_API_KEY` and `PORKBUN_SECRET_API_KEY` environment variables, or pass `--api-key` and `--secret-api-key` on every command.

Priority: `--api-key`/`--secret-api-key` flags > `PORKBUN_API_KEY`/`PORKBUN_SECRET_API_KEY` env vars > `~/.config/porkbun/config.toml`.

To override the API base URL (e.g., for testing against a local mock):

```sh
export PORKBUN_BASE_URL=http://localhost:9999
```

## Usage

### Ping

```sh
porkbun ping
```

### Pricing

```sh
porkbun pricing get
```

### Domains

```sh
porkbun domains list
porkbun domains list --start 10 --include-labels
porkbun domains check example.com
porkbun domains create example.com --cost 999 --agree-to-terms
porkbun domains get-ns example.com
porkbun domains update-ns example.com --ns ns1.example.com ns2.example.com
porkbun domains update-auto-renew example.com --status on
porkbun domains update-auto-renew example.com --status off
```

### DNS

```sh
porkbun dns create example.com --type A --content 1.2.3.4
porkbun dns create example.com --type A --content 1.2.3.4 --name www --ttl 300
porkbun dns create example.com --type MX --content mail.example.com --prio 10
porkbun dns edit example.com 123456789 --type A --content 5.6.7.8
porkbun dns edit-by-name-type example.com A www --content 5.6.7.8
porkbun dns delete example.com 123456789
porkbun dns delete-by-name-type example.com A www
porkbun dns retrieve example.com
porkbun dns retrieve example.com 123456789
porkbun dns retrieve-by-name-type example.com A
porkbun dns retrieve-by-name-type example.com A www
```

### DNSSEC

```sh
porkbun dnssec create example.com --key-tag 12345 --alg 13 --digest-type 2 --digest abc123...
porkbun dnssec list example.com
porkbun dnssec delete example.com 12345
```

### URL Forwarding

```sh
porkbun url-forward add example.com --location https://target.example.com --type temporary --include-path yes --wildcard yes
porkbun url-forward add example.com --subdomain www --location https://target.example.com --type permanent --include-path no --wildcard no
porkbun url-forward list example.com
porkbun url-forward delete example.com 123456789
```

### Glue Records

```sh
porkbun glue create example.com ns1 --ips 1.2.3.4
porkbun glue create example.com ns1 --ips 1.2.3.4 2001:db8::1
porkbun glue update example.com ns1 --ips 5.6.7.8
porkbun glue delete example.com ns1
porkbun glue list example.com
```

### SSL

```sh
porkbun ssl retrieve example.com
```

## Output

Table output by default. Add `--json` to any command for JSON:

```sh
porkbun domains list --json
porkbun domains list --json | jq '.[].domain'
porkbun dns retrieve example.com --json | jq '.records[] | select(.type == "A")'
porkbun pricing get --json | jq '.pricing.com'
```

## API Coverage

| Resource | Commands |
|----------|----------|
| Ping | ping |
| Pricing | get |
| Domains | list, check, create, get-ns, update-ns, update-auto-renew |
| DNS | create, edit, edit-by-name-type, delete, delete-by-name-type, retrieve, retrieve (by ID), retrieve-by-name-type |
| DNSSEC | create, list, delete |
| URL Forwarding | add, list, delete |
| Glue Records | create, update, delete, list |
| SSL | retrieve |

27 endpoints total.

## Development

```sh
cargo test --workspace     # all tests (unit + integration)
cargo clippy --workspace   # lint
cargo fmt --all            # format
```

## Live Testing

Live tests run against the real Porkbun API and require active credentials.

```sh
PORKBUN_LIVE_TEST=1 \
PORKBUN_API_KEY=pk1_... \
PORKBUN_SECRET_API_KEY=sk1_... \
PORKBUN_TEST_DOMAIN=yourdomain.com \
cargo test --test live_test -- --test-threads=1
```

`PORKBUN_TEST_DOMAIN` must be a domain registered in your Porkbun account. `PORKBUN_API_KEY` and `PORKBUN_SECRET_API_KEY` can be set via env or config file.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
