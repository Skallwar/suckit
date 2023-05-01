//! Tests for using --exclude --include flags for suckit

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
fn visit_filter_is_download_filter() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            &url,
            "-o",
            output_dir,
            "-v",
            "-e",
            "no_download_no_visit.html",
            "--visit-filter-is-download-filter",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let result = cmd.wait_with_output().unwrap();
    let stdout_str = unsafe { String::from_utf8_unchecked(result.stdout) };
    assert!(result.status.success());

    let paths = read_dir(&files_dir).unwrap();

    assert!(!stdout_str.contains("should_not_get_visited.html"));
}

// Test to use include flag for visiting pages only matching the given pattern.
#[test]
fn visit_include_filter() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[&url, "-o", output_dir, "--include-visit", "mp[3-4]"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let paths = read_dir(&files_dir).unwrap();

    assert_eq!(
        paths.count() - 1, // minus one because of index.html which is downloaded unconditionally
        get_file_count_with_pattern(".mp3", &files_dir).unwrap()
    );
}

// Test demonstrating usage of multiple include patterns for visiting pages only matching the given pattern.
#[test]
fn visit_include_multiple_filters() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[&url, "-o", output_dir, "--include-visit", "(mp[3-4])|(txt)"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let status = cmd.wait().unwrap();
    assert!(status.success());
    let paths = read_dir(&files_dir).unwrap();
    let mp3_count = get_file_count_with_pattern(".mp3", &files_dir).unwrap();
    let txt_count = get_file_count_with_pattern(".txt", &files_dir).unwrap();
    assert_eq!(
        paths.count() - 1, // minus one because of index.html which is downloaded unconditionally
        mp3_count + txt_count
    );
}

// Test to use exclude flag for excluding pages matching the given pattern.
#[test]
fn visit_exclude_filter() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[&url, "-o", output_dir, "--exclude-visit", "jpe?g"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());

    let jpeg_count = get_file_count_with_pattern(".jpe?g", &files_dir).unwrap();
    assert_eq!(jpeg_count, 0);
}

// Test to use include flag for downloading pages only matching the given pattern.
#[test]
fn download_include_filter() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[&url, "-o", output_dir, "-i", "mp[3-4]"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let paths = read_dir(&files_dir).unwrap();

    assert_eq!(
        paths.count(),
        get_file_count_with_pattern(".mp3", &files_dir).unwrap()
    );

    std::fs::remove_dir_all(output_dir).unwrap();
}

// Test demonstrating usage of multiple include patterns for downloading pages only matching the given pattern.
#[test]
fn download_include_multiple_filters() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[&url, "-o", output_dir, "-i", "(mp[3-4])|(txt)"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let status = cmd.wait().unwrap();
    assert!(status.success());

    let paths = read_dir(&files_dir).unwrap();
    let mp3_count = get_file_count_with_pattern(".mp3", &files_dir).unwrap();
    let txt_count = get_file_count_with_pattern(".txt", &files_dir).unwrap();
    assert_eq!(paths.count(), mp3_count + txt_count);
}

// Test to use exclude flag for excluding pages matching the given pattern.
#[test]
fn download_exclude_filter() {
    let ip = fixtures::spawn_local_http_server(PAGE, false, None);
    let url = format!("http://{}", ip);

    let tempdir = mktemp::Temp::new_dir().unwrap();
    let output_dir = tempdir.to_str().unwrap();

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[&url, "-o", output_dir, "-e", "jpe?g"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let paths = read_dir(&files_dir).unwrap();
    let jpeg_count = get_file_count_with_pattern(".jpe?g", &files_dir).unwrap();
    assert_eq!(jpeg_count, 0);
}
