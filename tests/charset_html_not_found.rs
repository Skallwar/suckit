//! Test for charset detection/conversion

mod fixtures;

use std::fs;
use std::process::{Command, Stdio};

const PAGE: &'static str = "tests/fixtures/";
const PAGE_NO_META: &'static str = "tests/fixtures/charset_test_html_no_meta.html";
const IP: &'static str = "0.0.0.0";

#[test]
fn test_html_charset_not_found() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    // Spawn a single instance of a local http server usable by all tests in this module.
    let file_dir = format!("{}/{}", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[&url, "-o", output_dir])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let status = cmd.wait().unwrap();
    assert!(status.success());

    let file_path = fs::read_dir(file_dir)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path(); // There is only one file in the directory

    let data_source = fs::read(PAGE_NO_META).unwrap();
    let data_downloaded = fs::read(file_path).unwrap();

    assert!(!fixtures::do_vecs_match(&data_source, &data_downloaded));
}
