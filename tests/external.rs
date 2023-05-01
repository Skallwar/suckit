//! Tests for using --ext-depth

mod fixtures;

use fixtures::get_file_count_with_pattern;
use std::fs::read_dir;
use std::process::Command;
use std::process::Stdio;
use std::sync::Once;

const PAGE: &'static str = "tests/fixtures/";
const IP: &'static str = "0.0.0.0";

// Test to use include flag for downloading pages only matching the given pattern.
#[test]
fn with_external() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let local = format!("{}/{}/", output_dir, IP);
    let external = format!("{}/{}/", output_dir, "google.com");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[&url, "-o", output_dir, "-d", "0", "--ext-depth", "1"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let path_local = read_dir(&local).unwrap();
    let path_external = read_dir(&external).unwrap();

    assert_eq!(path_local.count() + path_external.count(), 2);
}

#[test]
fn without_external() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let external = format!("{}/{}/", output_dir, "google.com");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[&url, "-o", output_dir, "-d", "0", "--ext-depth", "0"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let path_external = read_dir(&external);

    assert!(path_external.is_err());
}
