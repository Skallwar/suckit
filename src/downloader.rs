extern crate reqwest;

use reqwest::Url;

pub fn download_url(url: Url) -> Result<String, reqwest::Error> {
    reqwest::get(url)?.text()
}
