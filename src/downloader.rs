use super::response::{Response, ResponseData};
use url::Url;

use crate::warn;

///A Downloader to download web content
pub struct Downloader {
    client: reqwest::blocking::Client,
    tries: usize
}

impl Downloader {
    /// Create a new Downloader
    pub fn new(tries: usize, user_agent: &str) -> Downloader {
        Downloader {
            client: reqwest::blocking::ClientBuilder::new()
                .cookie_store(true)
                .user_agent(user_agent)
                .build()
                .unwrap(),
            tries
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

    ///Download the content at this url
    fn make_request(&self, url: &Url) -> Result<Response, reqwest::Error> {
        match self.client.get(url.clone()).send() {
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
        match Downloader::new(1, "suckit").get(&url) {
            Err(e) => assert!(false, "Fail to download lwn.net: {:?}", e),
            _ => {}
        }
    }
}
