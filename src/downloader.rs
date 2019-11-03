extern crate reqwest;

use reqwest::Url;

pub fn download_url(url: Url) -> Result<String, reqwest::Error> {
    reqwest::get(url)?.text()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_url() {
        let url: Url = Url::parse("https://lwn.net").unwrap();
        match download_url(url) {
            Err(e) => assert!(false, "Fail to download lwn.net: {:?}", e),
            _ => {}
        }
    }
}
