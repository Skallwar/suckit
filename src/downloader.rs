use reqwest::Url;

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
            client: reqwest::blocking::Client::new(),
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
                    let data_type = data
                        .headers()
                        .get("content-type")
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();

                    let filename = if Downloader::is_html(&data_type) {
                        None
                    } else {
                        let content_disposition = data
                            .headers()
                            .get("content-disposition")
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();

                        Some(Downloader::get_filename(&content_disposition))
                    };

                    let data = match Downloader::is_html(&data_type) {
                        true => ResponseData::Html(data.text().unwrap()),
                        false => {
                            let mut raw_data: Vec<u8> = Vec::new();
                            data.copy_to(&mut raw_data);
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

    use pretty_assertions::assert_eq;

    #[test]
    fn test_download_url() {
        let url: Url = Url::parse("https://lwn.net").unwrap();
        match Downloader::new(1).get(&url) {
            Err(e) => assert!(false, "Fail to download lwn.net: {:?}", e),
            _ => {}
        }
    }

    #[test]
    fn test_url_content() {
        let url: Url = Url::parse("https://example.com").unwrap();
        match Downloader::new(1).get(&url) {
            Err(e) => assert!(false, "Fail to download example.com: {:?}", e),
            Ok(content) => assert_eq!(content,
"<!doctype html>
<html>
<head>
    <title>Example Domain</title>

    <meta charset=\"utf-8\" />
    <meta http-equiv=\"Content-type\" content=\"text/html; charset=utf-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
    <style type=\"text/css\">
    body {
        background-color: #f0f0f2;
        margin: 0;
        padding: 0;
        font-family: -apple-system, system-ui, BlinkMacSystemFont, \"Segoe UI\", \"Open Sans\", \"Helvetica Neue\", Helvetica, Arial, sans-serif;
        
    }
    div {
        width: 600px;
        margin: 5em auto;
        padding: 2em;
        background-color: #fdfdff;
        border-radius: 0.5em;
        box-shadow: 2px 3px 7px 2px rgba(0,0,0,0.02);
    }
    a:link, a:visited {
        color: #38488f;
        text-decoration: none;
    }
    @media (max-width: 700px) {
        div {
            margin: 0 auto;
            width: auto;
        }
    }
    </style>    
</head>

<body>
<div>
    <h1>Example Domain</h1>
    <p>This domain is for use in illustrative examples in documents. You may use this
    domain in literature without prior coordination or asking for permission.</p>
    <p><a href=\"https://www.iana.org/domains/example\">More information...</a></p>
</div>
</body>
</html>\n"),
        }
    }
}
