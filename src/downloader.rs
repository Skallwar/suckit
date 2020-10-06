use super::response::{Response, ResponseData};
use std::collections::HashMap;
use url::Url;

use crate::warn;

///A Downloader to download web content
pub struct Downloader {
    client: reqwest::blocking::Client,
    tries: usize,
    auth_map: HashMap<Option<String>, (String, Option<String>)>,
}

///Parse HTTP authentication credentials from string iterable
fn parse_auth(auth: &[String]) -> Option<(String, Option<String>, Option<String>)> {
    // Convert any empty strings to None
    let auth: Vec<Option<String>> = auth
        .iter()
        .map(|s| match s.as_ref() {
            "" => None,
            s => Some(s.to_string()),
        })
        .collect();
    match auth.as_slice() {
        [] => None,
        [None, ..] => None, // Empty username should be considered no-op
        [Some(username)] => Some((username.to_string(), None, None)),
        [Some(username), password] => Some((username.to_string(), password.clone(), None)),
        [Some(username), password, host, ..] => {
            Some((username.to_string(), password.clone(), host.clone()))
        }
    }
}

impl Downloader {
    /// Create a new Downloader
    pub fn new(tries: usize, user_agent: &str, auth: &[String]) -> Downloader {
        // Create a mapping of hosts to username, password tuples for authentication
        let mut auth_map = HashMap::new();
        if let Some((username, password, host)) = parse_auth(auth) {
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
        match self.auth_map.get(&url.host_str().map(String::from)) {
            Some(auth) => Some(auth),
            None => self.auth_map.get(&None),
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
                let data_type = match data.headers().get("content-type") {
                    Some(data_type) => data_type.to_str().unwrap(),
                    None => "text/html",
                };

                let filename = if !Downloader::is_html(data_type) {
                    Downloader::get_filename(data.headers())
                } else {
                    None
                };

                let data = if Downloader::is_html(data_type) {
                    ResponseData::Html(data.text().unwrap())
                } else {
                    let mut raw_data: Vec<u8> = Vec::new();
                    data.copy_to(&mut raw_data).unwrap();
                    ResponseData::Other(raw_data)
                };

                Ok(Response::new(data, filename))
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
        match Downloader::new(1, "suckit", &[]).get(&url) {
            Err(e) => assert!(false, "Fail to download lwn.net: {:?}", e),
            _ => {}
        }
    }

    #[test]
    fn test_parse_auth() {
        assert_eq!(parse_auth(&["".to_string(), "pw".to_string()]), None);
        assert_eq!(
            parse_auth(&["username".to_string()]),
            Some(("username".to_string(), None, None))
        );
        assert_eq!(
            parse_auth(&[
                "un".to_string(),
                "pw".to_string(),
                "h".to_string(),
                "t".to_string()
            ]),
            Some((
                "un".to_string(),
                Some("pw".to_string()),
                Some("h".to_string())
            ))
        )
    }
}
