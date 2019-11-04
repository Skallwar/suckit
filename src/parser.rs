use regex::Regex;
use reqwest::Url;

pub fn find_urls(str: String) -> Vec<Url> {
    let regex = Regex::new(r#"(href|src) *= *"([^ "]*)""#).unwrap();
    regex
        .captures_iter(&str)
        .map(|matched| Url::parse(&matched[2]))
        .filter_map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_href_url() {
        let vec = find_urls("href= \"https://lol.com\"\nsrc  = \"ulr2\"".to_string());
        assert_eq!(vec, [Url::parse("https://lol.com").unwrap()]);
    }
}
