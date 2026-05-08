mod test_env;

use assert_fs::{assert::PathAssert, fixture::PathChild};
use serde_json::json;
use test_env::TestEnv;

#[test]
fn test_cargo_version_bump() {
    let mut env = TestEnv::new(json!({
        "set_version": true,
        "publish": false,
        "act_on_no_update": false
    }));
    env.commit("fix: test\n\nBREAKING CHANGE: test");
    env.run(1, 0, 0, "true");
    env.repo()
        .child("Cargo.toml")
        .assert(predicates::str::contains("\nversion = \"1.0.0\"\n"));
}

#[test]
fn test_cargo_publish() {
    let mut env = TestEnv::new(json!({
        "set_version": true,
        "publish": true,
        "act_on_no_update": false
    }));
    env.commit("fix: test\n\nBREAKING CHANGE: test");
    env.run(1, 0, 0, "true");
    env.repo()
        .child("Cargo.toml")
        .assert(predicates::str::contains("\nversion = \"1.0.0\"\n"));
}

#[test]
fn test_cargo_act_on_no_update() {
    let mut env = TestEnv::new(json!({
        "set_version": true,
        "publish": true,
        "act_on_no_update": true
    }));
    env.run(1, 0, 0, "false");
    env.repo()
        .child("Cargo.toml")
        .assert(predicates::str::contains("\nversion = \"1.0.0\"\n"));
}

#[test]
fn test_cargo_not_act_on_no_update() {
    let mut env = TestEnv::new(json!({
        "set_version": true,
        "publish": true,
        "act_on_no_update": false
    }));
    env.run(1, 0, 0, "false");
    env.repo()
        .child("Cargo.toml")
        .assert(predicates::str::contains("\nversion = \"0.1.0\"\n"));
}
