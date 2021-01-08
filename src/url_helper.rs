use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use url::Url;

///Max file name size supported by the file system
const FILE_NAME_MAX_LENGTH: usize = 255;
///Characters that need to be replaced by encode()
const FRAGMENT: &AsciiSet = &CONTROLS.add(b'?');

///Encode special character with '%' representation
pub fn encode(path: &str) -> String {
    utf8_percent_encode(path, FRAGMENT).to_string()
}

///Convert an Url to the corresponding path
pub fn to_path(url: &Url) -> String {
    let domain = url.host_str().unwrap();
    let path = url.path();

    let mut path = format!("{}{}", domain, path);
    if path.ends_with("/") {
        path = format!("{}index.html", path);
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_to_path() {
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel/index.html").unwrap());

        assert_eq!(str, "lwn.net/Kernel/index.html");
    }

    #[test]
    fn url_to_path_index() {
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel/").unwrap());

        assert_eq!(str, "lwn.net/Kernel/index.html");
    }

    #[test]
    fn url_to_path_fragment() {
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel/#fragment").unwrap());

        assert_eq!(str, "lwn.net/Kernel/index.html");
    }
}
