use kuchiki::traits::*;
use kuchiki::NodeData::*;
use regex::Regex;
use reqwest::Url;
use std::borrow::BorrowMut;

pub fn find_urls(str: String) -> Vec<String> {
    let html = kuchiki::parse_html().one(str);
    let elm = html.select_first("[src]").unwrap();
    let node = elm.as_node();
    let elm = node.as_element().unwrap();
    let mut attributes_map = elm.attributes.try_borrow_mut().unwrap();
    let string = attributes_map.get_mut("src").unwrap();
    string.clear();
    string.push_str("lololo");

    println!("{:?}", attributes_map.get_mut("src").unwrap());

    // elm.lol();

    vec![]
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
