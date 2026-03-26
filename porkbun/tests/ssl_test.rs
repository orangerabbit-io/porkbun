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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "ssl",
        "retrieve",
        "example.com",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "--json",
        "ssl",
        "retrieve",
        "example.com",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"certificatechain\""));

    mock.assert();
}
