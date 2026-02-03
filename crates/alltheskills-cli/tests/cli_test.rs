use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_cli_list_help() {
    let mut cmd = cargo_bin_cmd!("alltheskills");
    cmd.arg("list").arg("--help").assert().success();
}

#[test]
fn test_cli_install_help() {
    let mut cmd = cargo_bin_cmd!("alltheskills");
    cmd.arg("install").arg("--help").assert().success();
}

#[test]
fn test_cli_search_help() {
    let mut cmd = cargo_bin_cmd!("alltheskills");
    cmd.arg("search").arg("--help").assert().success();
}

#[test]
fn test_cli_info_help() {
    let mut cmd = cargo_bin_cmd!("alltheskills");
    cmd.arg("info").arg("--help").assert().success();
}

#[test]
fn test_cli_add_source_help() {
    let mut cmd = cargo_bin_cmd!("alltheskills");
    cmd.arg("add-source").arg("--help").assert().success();
}

#[test]
fn test_cli_export_as_skill_help() {
    let mut cmd = cargo_bin_cmd!("alltheskills");
    cmd.arg("export-as-skill").arg("--help").assert().success();
}

#[test]
fn test_cli_config_help() {
    let mut cmd = cargo_bin_cmd!("alltheskills");
    cmd.arg("config").arg("--help").assert().success();
}

#[test]
fn test_cli_version() {
    let mut cmd = cargo_bin_cmd!("alltheskills");
    cmd.arg("--version").assert().success();
}
