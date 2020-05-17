use std::fs::File;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use tiny_http::{Response, Server};

const PAGE: &'static str = "tests/fixtures/index.html";

pub fn spawn_local_http_server() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    println!("Spawning http server");
    thread::spawn(move || {
        for request in server.incoming_requests() {
            let response = Response::from_file(File::open(PAGE).unwrap());
            request.respond(response).unwrap();
        }
    });
}

pub fn get_file_count_with_pattern(pattern: &str, dir: &str) -> Result<usize, ()> {
    // Command being run: `ls | grep .mp3 | wc -w`
    let mut du_output_child = Command::new("ls")
        .args(&[dir])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(du_output) = du_output_child.stdout.take() {
        let mut sort_output_child = Command::new("egrep")
            .arg(pattern)
            .stdin(du_output)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        du_output_child.wait().unwrap();

        if let Some(sort_output) = sort_output_child.stdout.take() {
            let head_output_child = Command::new("wc")
                .args(&["-w"])
                .stdin(sort_output)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let head_stdout = head_output_child.wait_with_output().unwrap();
            sort_output_child.wait().unwrap();
            return Ok(String::from_utf8(head_stdout.stdout)
                .unwrap()
                .trim()
                .parse()
                .unwrap());
        }
    }
    Err(())
}
