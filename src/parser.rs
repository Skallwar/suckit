use kuchiki::traits::*;
use kuchiki::NodeData::*;
use regex::Regex;
use reqwest::Url;
use std::borrow::BorrowMut;

pub fn find_urls(str: String) -> Vec<String> {
    let html = kuchiki::parse_html().one(str);
    let mut elm = html.select_first("[src]").unwrap();
    let mut elm_mut = elm.borrow_mut();
    let mut elm_data = match elm_mut.as_node().data() {
        Element(_elm) => _elm,
        _ => panic!(),
    };

    let expandedname = kuchiki::ExpandedName::new("", "src");
    let mut elm_attr = &mut elm_data.borrow_mut().attributes.borrow_mut().map;
    let mut attribute = &mut elm_attr.get(&expandedname).unwrap();
    let mut attribute_mut = attribute.borrow_mut();
    let url = (&mut attribute_mut).value.borrow_mut();

    // url.clear();
    // url.push_str("tropfort");
    println!("{:?}", elm_attr.get(&expandedname).unwrap().value);

    vec![]
    // let regex = Regex::new(r#"(href|src) *= *"([^ "]*)""#).unwrap();
    // regex
    //     .captures_iter(&str)
    //     .map(|matched| String::from(&matched[2]))
    //     .collect()
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
