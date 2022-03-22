//! Tests for using --exclude --include flags for suckit

mod fixtures;

use fixtures::get_file_count_with_pattern;
use std::fs::read_dir;
use std::process::Command;
use std::process::Stdio;
use std::sync::Once;

const PAGE: &'static str = "tests/fixtures/index.html";
const IP: &'static str = "0.0.0.0";
static START: Once = Once::new();

#[test]
fn test_include_exclude() {
    // Spawn a single instance of a local http server usable by all tests in this module.
    START.call_once(|| {
        fixtures::spawn_local_http_server(PAGE, false, None);
    });

    // Tests below are grouped together as they depend on the local_http_server above.
    download_include_filter();
    download_include_multiple_filters();
    download_exclude_filter();

    visit_include_filter();
    visit_include_multiple_filters();
    visit_exclude_filter();
}

// Test to use include flag for visiting pages only matching the given pattern.
fn visit_include_filter() {
    let output_dir = "w2";
    let _ = std::fs::remove_dir_all(output_dir);

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            fixtures::HTTP_ADDR,
            "-o",
            output_dir,
            "--include-visit",
            "mp[3-4]",
            "-j",
            "16",
        ])
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

    std::fs::remove_dir_all(output_dir).unwrap();
}

// Test demonstrating usage of multiple include patterns for visiting pages only matching the given pattern.
fn visit_include_multiple_filters() {
    let output_dir = "w1";
    let _ = std::fs::remove_dir_all(output_dir);

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            fixtures::HTTP_ADDR,
            "-o",
            output_dir,
            "--include-visit",
            "(mp[3-4])|(txt)",
            "-j",
            "16",
        ])
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

    std::fs::remove_dir_all(output_dir).unwrap();
}

// Test to use exclude flag for excluding pages matching the given pattern.
fn visit_exclude_filter() {
    let output_dir = "w3";
    let _ = std::fs::remove_dir_all(output_dir);

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            fixtures::HTTP_ADDR,
            "-o",
            output_dir,
            "--exclude-visit",
            "jpe?g",
            "-j",
            "16",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let paths = read_dir(&files_dir).unwrap();
    let mp3_count = get_file_count_with_pattern(".mp3", &files_dir).unwrap();
    let txt_count = get_file_count_with_pattern(".txt", &files_dir).unwrap();
    let index_file = 1;
    assert_eq!(paths.count(), mp3_count + txt_count + index_file);

    std::fs::remove_dir_all(output_dir).unwrap();
}

// Test to use include flag for downloading pages only matching the given pattern.
fn download_include_filter() {
    let output_dir = "w2";
    let _ = std::fs::remove_dir_all(output_dir);

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            fixtures::HTTP_ADDR,
            "-o",
            output_dir,
            "-i",
            "mp[3-4]",
            "-j",
            "16",
        ])
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
fn download_include_multiple_filters() {
    let output_dir = "w1";
    let _ = std::fs::remove_dir_all(output_dir);

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            fixtures::HTTP_ADDR,
            "-o",
            output_dir,
            "-i",
            "(mp[3-4])|(txt)",
            "-j",
            "16",
        ])
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

    std::fs::remove_dir_all(output_dir).unwrap();
}

// Test to use exclude flag for excluding pages matching the given pattern.
fn download_exclude_filter() {
    let output_dir = "w3";
    let _ = std::fs::remove_dir_all(output_dir);

    let files_dir = format!("{}/{}/", output_dir, IP);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_suckit"))
        .args(&[
            fixtures::HTTP_ADDR,
            "-o",
            output_dir,
            "-e",
            "jpe?g",
            "-j",
            "16",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    assert!(status.success());
    let paths = read_dir(&files_dir).unwrap();
    let mp3_count = get_file_count_with_pattern(".mp3", &files_dir).unwrap();
    let txt_count = get_file_count_with_pattern(".txt", &files_dir).unwrap();
    let index_file = 1;
    assert_eq!(paths.count(), mp3_count + txt_count + index_file);

    std::fs::remove_dir_all(output_dir).unwrap();
}
