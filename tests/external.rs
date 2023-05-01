//! Tests for using --ext-depth

mod fixtures;

use fixtures::get_file_count_with_pattern;
use std::fs::read_dir;
use std::process::Command;
use std::process::Stdio;
use std::sync::Once;

const PAGE: &'static str = "tests/fixtures/";
const IP: &'static str = "0.0.0.0";
static START: Once = Once::new();

#[test]
fn test_external_download() {
    // Spawn a single instance of a local http server usable by all tests in this module.
    START.call_once(|| {
        fixtures::spawn_local_http_server(PAGE, false, None);
    });

    // Tests below are grouped together as they depend on the local_http_server above.
    with_external();
    without_external();
}

// Test to use include flag for downloading pages only matching the given pattern.
fn with_external() {
    let output_dir = "w1";
    let local = format!("{}/{}/", output_dir, IP);
    let external = format!("{}/{}/", output_dir, "google.com");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            fixtures::HTTP_ADDR,
            "-o",
            output_dir,
            "-d",
            "0",
            "--ext-depth",
            "1",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let path_local = read_dir(&local).unwrap();
    let path_external = read_dir(&external).unwrap();

    assert_eq!(path_local.count() + path_external.count(), 2);

    std::fs::remove_dir_all(output_dir).unwrap();
}

fn without_external() {
    let output_dir = "w2";
    let external = format!("{}/{}/", output_dir, "google.com");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            fixtures::HTTP_ADDR,
            "-o",
            output_dir,
            "-d",
            "0",
            "--ext-depth",
            "0",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let path_external = read_dir(&external);

    assert!(path_external.is_err());

    std::fs::remove_dir_all(output_dir).unwrap();
}
