use url::Url;

/// Wrapper around a reqwest client, used to get the content of web pages
pub struct Downloader {
    client: reqwest::blocking::Client,
    tries: usize,
}

pub enum ResponseData {
    Html(String),
    Other(Vec<u8>),
}

pub struct Response {
    data: ResponseData,
    filename: Option<String>,
}

impl Downloader {
    /// Create a new Downloader
    pub fn new(tries: usize) -> Downloader {
        Downloader {
            client: reqwest::blocking::ClientBuilder::new()
                .cookie_store(true)
                .build()
                .unwrap(),
            tries: tries,
        }
    }

    fn is_html(content_type: &str) -> bool {
        content_type.contains("text/html")
    }

    fn get_filename(content_disposition: &String) -> String {
        let content_disposition = content_disposition.clone();
        let index = content_disposition.find("=").unwrap() + 1;

        content_disposition[index..].to_string()
    }

    /// Download the content located at a given URL
    pub fn get(&self, url: &Url) -> Result<Response, reqwest::Error> {
        let mut error: Option<reqwest::Error> = None;
        for _ in 0..self.tries {
            match self.client.get(url.clone()).send() {
                Ok(mut data) => {
                    let data_type = match data.headers().get("content-type") {
                        Some(data_type) => data_type.to_str().unwrap().to_string(),
                        None => "text/html".to_string(),
                    };

                    let filename = if Downloader::is_html(&data_type) {
                        None
                    } else {
                        match data.headers().get("content-disposition") {
                            Some(content_disposition) => {
                                let content_disposition =
                                    content_disposition.to_str().unwrap().to_string();
                                Some(Downloader::get_filename(&content_disposition))
                            }
                            None => None,
                        }
                    };

                    let data = match Downloader::is_html(&data_type) {
                        true => ResponseData::Html(data.text().unwrap()),
                        false => {
                            let mut raw_data: Vec<u8> = Vec::new();
                            data.copy_to(&mut raw_data).unwrap();
                            ResponseData::Other(raw_data)
                        }
                    };

                    return Ok(Response::new(data, filename));
                }
                Err(e) => {
                    println!("Downloader.get() has encounter an error: {}", e);
                    error = Some(e);
                }
            };
        }

        return Err(error.unwrap());
    }
}

impl Response {
    pub fn new(data: ResponseData, filename: Option<String>) -> Response {
        Response {
            data: data,
            filename: filename,
        }
    }

    pub fn get_data(&self) -> &ResponseData {
        &self.data
    }

    pub fn get_filename(&self) -> &Option<String> {
        &self.filename
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_url() {
        let url: Url = Url::parse("https://lwn.net").unwrap();
        match Downloader::new(1).get(&url) {
            Err(e) => assert!(false, "Fail to download lwn.net: {:?}", e),
            _ => {}
        }
    }
}
