use assert_cmd::Command;

#[test]
fn test_cli_list_help() {
    let mut cmd = Command::cargo_bin("allskills-cli").unwrap();
    cmd.arg("list").arg("--help").assert().success();
}

#[test]
fn test_cli_install_help() {
    let mut cmd = Command::cargo_bin("allskills-cli").unwrap();
    cmd.arg("install").arg("--help").assert().success();
}

#[test]
fn test_cli_search_help() {
    let mut cmd = Command::cargo_bin("allskills-cli").unwrap();
    cmd.arg("search").arg("--help").assert().success();
}

#[test]
fn test_cli_info_help() {
    let mut cmd = Command::cargo_bin("allskills-cli").unwrap();
    cmd.arg("info").arg("--help").assert().success();
}

#[test]
fn test_cli_add_source_help() {
    let mut cmd = Command::cargo_bin("allskills-cli").unwrap();
    cmd.arg("add-source").arg("--help").assert().success();
}

#[test]
fn test_cli_export_as_skill_help() {
    let mut cmd = Command::cargo_bin("allskills-cli").unwrap();
    cmd.arg("export-as-skill").arg("--help").assert().success();
}

#[test]
fn test_cli_config_help() {
    let mut cmd = Command::cargo_bin("allskills-cli").unwrap();
    cmd.arg("config").arg("--help").assert().success();
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("allskills-cli").unwrap();
    cmd.arg("--version").assert().success();
}
