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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "glue",
        "list",
        "example.com",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("ns1.example.com"))
    .stdout(predicate::str::contains("1.2.3.4"));

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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "--json",
        "glue",
        "list",
        "example.com",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("\"hosts\""));

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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "glue",
        "create",
        "example.com",
        "ns1",
        "--ips",
        "1.2.3.4",
        "5.6.7.8",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("created"));

    mock.assert();
}

#[test]
fn test_glue_update() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/domain/updateGlue/example.com/ns1")
        .with_body(common::fixture("success.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "glue",
        "update",
        "example.com",
        "ns1",
        "--ips",
        "9.8.7.6",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("updated"));

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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "glue",
        "delete",
        "example.com",
        "ns1",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("deleted"));

    mock.assert();
}
