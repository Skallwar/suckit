use reqwest::Url;

/// Wrapper around a reqwest client, used to get the content of web pages
pub struct Downloader {
    client: reqwest::blocking::Client,
    tries: usize,
}

impl Downloader {
    /// Create a new Downloader
    pub fn new(tries: usize) -> Downloader {
        Downloader {
            client: reqwest::blocking::Client::new(),
            tries: tries,
        }
    }

    /// Download the content located at a given URL
    pub fn get(&self, url: &Url) -> Result<String, reqwest::Error> {
        let mut error: Option<reqwest::Error> = None;
        for _ in 0..self.tries {
            match self.client.get(url.clone()).send() {
                Ok(data) => return Ok(data.text().unwrap()),
                Err(e) => {
                    println!("Downloader.get() has encountered an error: {}", e);
                    error = Some(e);
                }
            };
        }

        return Err(error.unwrap());
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
