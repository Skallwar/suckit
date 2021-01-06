use std::fs::File;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use tiny_http::{Header, Response, Server};

pub const HTTP_ADDR: &'static str = "http://0.0.0.0:8000";
const ADDR: &'static str = "0.0.0.0:8000";
const AUTH_HEADER: &str = "Authorization";
const AUTH_CREDENTIALS: &str = "Basic dXNlcm5hbWU6cGFzc3dvcmQ="; // base64-encoded "username:password"

pub fn spawn_local_http_server(
    page: &'static str,
    requires_auth: bool,
    headers: Option<&'static Vec<(&'static str, &'static str)>>,
) {
    let server = Server::http(ADDR).unwrap();
    println!("Spawning http server");
    thread::spawn(move || {
        for request in server.incoming_requests() {
            // Authenticate request from headers if provided
            let auth_header = request
                .headers()
                .iter()
                .find(|h| h.field.equiv(AUTH_HEADER));
            let valid_auth = check_auth_credentials(auth_header);

            let mut response = if requires_auth && !valid_auth {
                let mut response = Response::from_string("Invalid auth").with_status_code(401);
                let h = Header::from_bytes("WWW-Authenticate", r#"Basic realm="Test""#).unwrap();
                response.add_header(h);
                response.boxed()
            } else {
                Response::from_file(File::open(page).unwrap()).boxed()
            };

            match headers {
                Some(vec) => {
                    let mut key_vec: Vec<u8> = vec![];
                    let mut value_vec: Vec<u8> = vec![];
                    for (key, value) in vec {
                        key_vec.extend_from_slice(key.as_bytes());
                        value_vec.extend_from_slice(value.as_bytes());
                    }

                    let h = Header::from_bytes(key_vec, value_vec).unwrap();
                    response.add_header(h);
                }
                _ => (),
            }

            request.respond(response).unwrap();
        }
    });
}

fn check_auth_credentials(auth_header: Option<&Header>) -> bool {
    match auth_header {
        None => false,
        Some(header) => header.value.as_str() == AUTH_CREDENTIALS,
    }
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

pub fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
