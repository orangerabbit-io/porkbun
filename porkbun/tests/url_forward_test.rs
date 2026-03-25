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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "url-forward",
        "list",
        "example.com",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("98765"))
    .stdout(predicate::str::contains("target.com"));

    mock.assert();
}

#[test]
fn test_url_forward_list_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/getUrlForwarding/example.com")
        .with_body(common::fixture("url_forward_list.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "--json",
        "url-forward",
        "list",
        "example.com",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"forwards\""));

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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "url-forward",
        "add",
        "example.com",
        "--location",
        "https://target.com",
        "--type",
        "permanent",
        "--include-path",
        "yes",
        "--wildcard",
        "no",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "url-forward",
        "delete",
        "example.com",
        "98765",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("deleted"));

    mock.assert();
}
