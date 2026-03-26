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
        args,
        stdout,
        stderr
    );
    serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("Invalid JSON from {:?}: {}\nstdout: {}", args, e, stdout))
}

#[allow(dead_code)]
fn run_ok(args: &[&str]) -> String {
    let mut cmd = common::binary();
    let output = cmd.args(args).output().expect("failed to execute binary");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "Command failed: {:?}\nstdout: {}\nstderr: {}",
        args,
        stdout,
        stderr
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
