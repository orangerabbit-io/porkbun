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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "ping",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "--json",
        "ping",
    ])
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
