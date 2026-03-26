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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "domains",
        "list",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "--json",
        "domains",
        "list",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "domains",
        "check",
        "newdomain.com",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "domains",
        "get-ns",
        "example.com",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "domains",
        "update-ns",
        "example.com",
        "--ns",
        "ns1.custom.com",
        "ns2.custom.com",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "domains",
        "update-auto-renew",
        "example.com",
        "--status",
        "on",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("example.com"));

    mock.assert();
}

#[test]
fn test_domains_create() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/create/newdomain.com")
        .with_body(common::fixture("domain_create.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "domains",
        "create",
        "newdomain.com",
        "--cost",
        "973",
        "--agree-to-terms",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "--json",
        "domains",
        "check",
        "newdomain.com",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "--json",
        "domains",
        "get-ns",
        "example.com",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"ns\""));

    mock.assert();
}
