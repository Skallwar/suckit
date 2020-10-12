//! Tests for using --auth flags for suckit

mod fixtures;

use fixtures::get_file_count_with_pattern;
use std::fs::read_dir;
use std::process::Command;
use std::process::Stdio;
use std::sync::Once;

const ADDR: &'static str = "http://0.0.0.0:8000";
static START: Once = Once::new();

#[test]
fn test_auth() {
    // Spawn a single instance of a local http server usable by all tests in this module.
    START.call_once(|| {
        fixtures::spawn_local_http_server(true);
    });

    // Tests below are grouped together as they depend on the local_http_server above.
    auth_different_host();
    auth_valid();
}

// Shouldn't supply credentials to a non-matching host
fn auth_different_host() {
    let output_dir = "w4";
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            ADDR,
            "-o",
            "w4",
            "-a",
            "username password example.com",
            "-j",
            "16",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let paths = read_dir(output_dir).unwrap();
    // Only the initial invalid response file should be present
    assert_eq!(paths.count(), 1);

    std::fs::remove_dir_all(output_dir).unwrap();
}

// Should authenticate with credentials to host (defaulting to origin host)
fn auth_valid() {
    let output_dir = "w5";
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[ADDR, "-o", "w5", "-a", "username password", "-j", "16"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let paths = read_dir(output_dir).unwrap();
    // Should load multiple paths, not just the invalid auth response
    assert!(paths.count() > 1);

    std::fs::remove_dir_all(output_dir).unwrap();
}
