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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "dns",
        "retrieve",
        "example.com",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "--json",
        "dns",
        "retrieve",
        "example.com",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "dns",
        "create",
        "example.com",
        "--type",
        "A",
        "--content",
        "1.2.3.4",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "dns",
        "edit",
        "example.com",
        "123",
        "--type",
        "A",
        "--content",
        "5.6.7.8",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "dns",
        "delete",
        "example.com",
        "123",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("deleted"));

    mock.assert();
}

#[test]
fn test_dns_retrieve_by_name_type() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/dns/retrieveByNameType/example.com/A/www")
        .with_body(common::fixture("dns_retrieve.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "dns",
        "retrieve-by-name-type",
        "example.com",
        "A",
        "www",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "dns",
        "edit-by-name-type",
        "example.com",
        "A",
        "www",
        "--content",
        "9.8.7.6",
    ])
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
    cmd.args([
        "--api-key",
        "pk1_test",
        "--secret-api-key",
        "sk1_test",
        "dns",
        "delete-by-name-type",
        "example.com",
        "A",
        "www",
    ])
    .env("PORKBUN_BASE_URL", server.url())
    .assert()
    .success()
    .stdout(predicate::str::contains("deleted"));

    mock.assert();
}
