//! Test for charset detection/conversion

mod fixtures;

use std::fs;
use std::process::{Command, Stdio};
use std::sync::Once;

use lazy_static::lazy_static;

const PAGE_NO_META: &'static str = "tests/fixtures/charset_test_html_no_meta.html";
static START: Once = Once::new();

lazy_static! {
    static ref CHARSET_HEADER: Vec<(&'static str, &'static str)> =
        vec![("Content-Type", "charset=windows-1252")];
}

#[test]
fn test_http_charset_found() {
    // Spawn a single instance of a local http server usable by all tests in this module.
    START.call_once(|| {
        fixtures::spawn_local_http_server(PAGE_NO_META, false, Some(&CHARSET_HEADER));
    });

    let output_dir = "charset_html_found";
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[fixtures::HTTP_ADDR, "-o", output_dir])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let status = cmd.wait().unwrap();
    assert!(status.success());
    let file_path = fs::read_dir(output_dir)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path(); // There is only one file in the directory

    let data_source = fs::read(PAGE_NO_META).unwrap();
    let data_downloaded = fs::read(file_path).unwrap();

    assert!(fixtures::do_vecs_match(&data_source, &data_downloaded));

    fs::remove_dir_all(output_dir).unwrap();
}
