extern crate reqwest;

use reqwest::Url;

pub fn download_url(url: Url) -> Result<String, reqwest::Error> {
    reqwest::get(url)?.text()
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_download_url() {
        let url: Url = Url::parse("https://lwn.net").unwrap();
        match download_url(url) {
            Err(e) => assert!(false, "Fail to download lwn.net: {:?}", e),
            _ => {}
        }
    }

    #[test]
    fn test_url_content() {
        let url: Url = Url::parse("https://example.com").unwrap();
        match download_url(url) {
            Err(e) => assert!(false, "Fail to download lwn.net: {:?}", e),
            Ok(content) => assert_eq!(content, "<!doctype html>
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
