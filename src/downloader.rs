use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

use crate::warn;

use super::response::{Response, ResponseData};

const AUTH_CHUNK_SIZE: usize = 3;

///A Downloader to download web content
pub struct Downloader {
    client: reqwest::blocking::Client,
    tries: usize,
    auth_map: HashMap<String, (String, Option<String>)>,
}

/// Parse HTTP authentication credentials from string iterable
fn parse_auth(auth: &[String], origin: &Url) -> Result<(String, Option<String>, String), String> {
    // Convert any empty strings to None
    let auth: Vec<Option<String>> = auth
        .iter()
        .map(|s| match s.as_ref() {
            "" => None,
            s => Some(s.to_string()),
        })
        .collect();

    // Match on auth values and origin host, defaulting to the origin host if host not provided
    match (auth.as_slice(), origin.host_str()) {
        ([Some(username)], Some(origin_host)) => {
            Ok((username.to_string(), None, origin_host.to_string()))
        }
        ([Some(username), password], Some(origin_host)) => Ok((
            username.to_string(),
            password.clone(),
            origin_host.to_string(),
        )),
        ([Some(username), password, None, ..], Some(origin_host)) => Ok((
            username.to_string(),
            password.clone(),
            origin_host.to_string(),
        )),
        ([Some(username), password, Some(host), ..], _) => {
            Ok((username.to_string(), password.clone(), host.to_string()))
        }
        _ => Err("Invalid arguments supplied to auth".to_string()),
    }
}

impl Downloader {
    /// Create a new Downloader
    pub fn new(tries: usize, user_agent: &str, auth: &[String], origin: &Url) -> Downloader {
        // Create a mapping of hosts to username, password tuples for authentication
        let mut auth_map = HashMap::new();
        // Iterate over the auth string in chunks of 3 items each for (username, password, host)
        for auth_chunk in auth.chunks(AUTH_CHUNK_SIZE) {
            // Throwing the error with panic! for now if parsing fails
            let (username, password, host) = parse_auth(auth_chunk, origin).unwrap();
            auth_map.insert(host, (username, password));
        }

        Downloader {
            client: reqwest::blocking::ClientBuilder::new()
                .cookie_store(true)
                .user_agent(user_agent)
                .build()
                .unwrap(),
            tries,
            auth_map,
        }
    }

    ///Check if the type in the 'content-type' head field is html
    fn is_html(content_type: &str) -> bool {
        content_type.contains("text/html")
    }

    ///Return the filename based on the HTML header of the response
    fn get_filename(header_map: &reqwest::header::HeaderMap) -> Option<String> {
        if let Some(content_disposition) = header_map.get("content-disposition") {
            let content_disposition = content_disposition.to_str().unwrap();
            let index = content_disposition.find('=').unwrap() + 1;

            Some(content_disposition[index..].to_string())
        } else {
            None
        }
    }

    /// Load HTTP auth credentials in a username, password tuple based on the host string
    fn get_auth(&self, url: &Url) -> Option<&(String, Option<String>)> {
        if let Some(host) = url.host_str() {
            self.auth_map.get(&host.to_string())
        } else {
            None
        }
    }

    ///Download the content at this url
    fn make_request(&self, url: &Url) -> Result<Response, reqwest::Error> {
        let req = self.client.get(url.clone());
        let req = match self.get_auth(url) {
            Some((username, password)) => req.basic_auth(username, password.clone()),
            None => req,
        };
        match req.send() {
            Ok(mut data) => {
                lazy_static! {
                    static ref DATA_TYPE_REGEX: Regex =
                        Regex::new("^.*(\\b[a-z]+/[a-z-+\\.]+).*$").unwrap();
                    static ref CHARSET_REGEX: Regex =
                        Regex::new("^.*charset\\s*=\\s*\"?([^\"\\s;]+).*$").unwrap();
                }

                let (data_type, charset): (String, Option<String>) =
                    match data.headers().get("content-type") {
                        Some(content_type_header) => {
                            let content_type = content_type_header.to_str().unwrap();
                            let data_type_captures =
                                DATA_TYPE_REGEX.captures_iter(&content_type).nth(0);
                            let data_type = data_type_captures
                                .map_or(String::from("text/html"), |first| {
                                    String::from(first.get(1).unwrap().as_str().to_lowercase())
                                });
                            let charset_captures =
                                CHARSET_REGEX.captures_iter(&content_type).nth(0);
                            let charset = charset_captures.map(|first| {
                                String::from(first.get(1).unwrap().as_str().to_lowercase())
                            });
                            (data_type, charset)
                        }
                        None => (String::from("text/html"), None),
                    };

                let filename = if !Downloader::is_html(&data_type) {
                    Downloader::get_filename(data.headers())
                } else {
                    None
                };

                let mut raw_data: Vec<u8> = Vec::new();
                data.copy_to(&mut raw_data).unwrap();
                let response_data = if Downloader::is_html(&data_type) {
                    ResponseData::Html(raw_data)
                } else {
                    ResponseData::Other(raw_data)
                };

                Ok(Response::new(response_data, filename, charset))
            }

            Err(e) => {
                warn!("Downloader.get() has encountered an error: {}", e);
                Err(e)
            }
        }
    }

    ///Download the content of an url and retries at most 'tries' times on failure
    pub fn get(&self, url: &Url) -> Result<Response, reqwest::Error> {
        let mut error: Option<reqwest::Error> = None;
        for _ in 0..self.tries {
            match self.make_request(url) {
                Ok(response) => return Ok(response),
                Err(e) => error = Some(e),
            }
        }

        Err(error.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_url() {
        let url: Url = Url::parse("https://lwn.net").unwrap();
        match Downloader::new(1, "suckit", &[], &url).get(&url) {
            Err(e) => assert!(false, "Fail to download lwn.net: {:?}", e),
            _ => {}
        }
    }

    #[test]
    fn test_parse_auth() {
        assert_eq!(
            parse_auth(
                &["".to_string(), "pw".to_string()],
                &Url::parse("https://example.com/").unwrap(),
            ),
            Err("Invalid arguments supplied to auth".to_string())
        );
        assert_eq!(
            parse_auth(
                &["username".to_string()],
                &Url::parse("https://example.com/").unwrap(),
            ),
            Ok(("username".to_string(), None, "example.com".to_string()))
        );
        assert_eq!(
            parse_auth(
                &[
                    "un".to_string(),
                    "pw".to_string(),
                    "h".to_string(),
                    "t".to_string()
                ],
                &Url::parse("https://example.com/").unwrap(),
            ),
            Ok(("un".to_string(), Some("pw".to_string()), "h".to_string()))
        )
    }
}
