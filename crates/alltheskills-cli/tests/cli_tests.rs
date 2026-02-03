//! CLI integration tests

use assert_cmd::Command;
use predicates::prelude::*;

/// Test that the CLI binary runs and shows help
#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("alltheskills").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("alltheskills"))
        .stdout(predicate::str::contains("List all installed skills"))
        .stdout(predicate::str::contains("Install a skill"))
        .stdout(predicate::str::contains("Search for skills"));
}

/// Test that the list command runs (even if no skills found)
#[test]
fn test_cli_list() {
    let mut cmd = Command::cargo_bin("alltheskills").unwrap();
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No skills found").or(predicate::str::contains("skill(s)")));
}

/// Test that the config command runs
#[test]
fn test_cli_config() {
    let mut cmd = Command::cargo_bin("alltheskills").unwrap();
    cmd.arg("config");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Current configuration").or(predicate::str::contains("Version")));
}

/// Test that the config --path command works
#[test]
fn test_cli_config_path() {
    let mut cmd = Command::cargo_bin("alltheskills").unwrap();
    cmd.arg("config").arg("--path");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Config path"));
}

/// Test that search command handles empty results
#[test]
fn test_cli_search_no_results() {
    let mut cmd = Command::cargo_bin("alltheskills").unwrap();
    cmd.arg("search").arg("xyz_nonexistent_skill_12345");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No skills found"));
}

/// Test that info command handles missing skill
#[test]
fn test_cli_info_missing_skill() {
    let mut cmd = Command::cargo_bin("alltheskills").unwrap();
    cmd.arg("info").arg("xyz_nonexistent_skill_12345");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("not found"));
}

/// Test that validate command with no args works
#[test]
fn test_cli_validate_all() {
    let mut cmd = Command::cargo_bin("alltheskills").unwrap();
    cmd.arg("validate");
    cmd.assert()
        .success();
}

/// Test that update command runs
#[test]
fn test_cli_update() {
    let mut cmd = Command::cargo_bin("alltheskills").unwrap();
    cmd.arg("update");
    cmd.assert()
        .success();
}
