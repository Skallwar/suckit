use regex::Regex;
use reqwest::Url;

// FIXME: Return only URLs in same domain to avoid infinite loop in Scraper::run()
pub fn find_urls(str: String) -> Vec<String> {
    let regex = Regex::new(r#"(href|src) *= *"([^ "]*)""#).unwrap();
    regex
        .captures_iter(&str)
        .map(|matched| String::from(&matched[2]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_urls() {
        let vec = find_urls("href= \"https://lol.com\"\nsrc  = \"url2\"".to_string());
        assert_eq!(vec, ["https://lol.com", "url2"]);
    }
}
