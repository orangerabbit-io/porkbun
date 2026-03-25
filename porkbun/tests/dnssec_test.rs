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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "dnssec",
        "list",
        "example.com",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("12345"));

    mock.assert();
}

#[test]
fn test_dnssec_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/getDnssecRecords/example.com")
        .with_body(common::fixture("dnssec_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "--json",
        "dnssec",
        "list",
        "example.com",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"records\""));

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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "dnssec",
        "create",
        "example.com",
        "--key-tag",
        "12345",
        "--alg",
        "13",
        "--digest-type",
        "2",
        "--digest",
        "abc123",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "dnssec",
        "delete",
        "example.com",
        "12345",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("deleted"));

    mock.assert();
}
