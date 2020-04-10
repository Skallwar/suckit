use crate::dom::Dom;

// FIXME: Return only URLs in same domain to avoid infinite loop in Scraper::run()
pub fn find_urls(str: String) -> Vec<String> {
    let dom = Dom::new(str.as_str());

    return dom.find_urls_as_strings();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_urls() {
        let vec = find_urls(
            "<a href= \"https://lol.com\">
            <img src  = \"url2\">"
                .to_string(),
        );

        assert_eq!(vec, ["https://lol.com", "url2"]);
    }
}
