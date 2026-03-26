mod common;

use mockito::Server;
use predicates::prelude::*;

#[test]
fn test_pricing_get_table() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pricing/get")
        .with_body(common::fixture("pricing_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["pricing", "get"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("com"))
        .stdout(predicate::str::contains("9.73"));

    mock.assert();
}

#[test]
fn test_pricing_get_json() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pricing/get")
        .with_body(common::fixture("pricing_get.json"))
        .with_header("content-type", "application/json")
        .create();

    let mut cmd = common::binary();
    cmd.args(["--json", "pricing", "get"])
        .env("PORKBUN_BASE_URL", server.url())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"pricing\""));

    mock.assert();
}

#[test]
fn test_pricing_get_no_auth_required() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/pricing/get")
        .with_body(common::fixture("pricing_get.json"))
        .with_header("content-type", "application/json")
        .create();

    // No --api-key or --secret-api-key flags — should still work
    let mut cmd = common::binary();
    cmd.args(["pricing", "get"])
        .env("PORKBUN_BASE_URL", server.url())
        .env_remove("PORKBUN_API_KEY")
        .env_remove("PORKBUN_SECRET_API_KEY")
        .assert()
        .success();

    mock.assert();
}
