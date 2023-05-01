//! Tests for using --auth flags for suckit

mod fixtures;

use std::fs::read_dir;
use std::process::Command;
use std::process::Stdio;

const PAGE: &'static str = "tests/fixtures/";
const IP: &'static str = "0.0.0.0";

// Shouldn't supply credentials to a non-matching host
#[test]
fn auth_different_host() {
    let ip = fixtures::spawn_local_http_server(PAGE, true, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            &url,
            "-o",
            output_dir,
            "-a",
            "username password example.com",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let paths = read_dir(format!("{}/{}", output_dir, IP)).unwrap();

    // Only the initial invalid response file should be present
    assert_eq!(paths.count(), 1);
}

// Should authenticate with credentials to host (defaulting to origin host)
#[test]
fn auth_valid() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[&url, "-o", output_dir, "-a", "username password"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());

    let paths = read_dir(format!("{}/{}", output_dir, IP)).unwrap();
    // Should load multiple paths, not just the invalid auth response
    assert!(paths.count() > 1);
}
