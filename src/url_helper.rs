use std::path::Path;

use md5;
use url::Url;

///Max file name size supported by the file system
const FILE_NAME_MAX_LENGTH: usize = 255;

/// Convert an Url to the corresponding path
pub fn to_path(url: &Url) -> String {
    let url_domain = url.host_str().unwrap();
    let url_path = url.path();
    let url_query = url.query();

    let path = Path::new(url_path);
    let mut filename = path.file_name().map_or(String::from(""), |filename| {
        filename.to_str().unwrap().to_string()
    });
    let mut parent = path
        .parent()
        .map_or("", |filename| filename.to_str().unwrap());

    if url_path.ends_with("/") {
        filename = "index.html".to_string();
        parent = url_path.trim_end_matches("/");
    } else if path.extension().is_none() {
        parent = url_path;
        filename = "index_no_slash.html".to_string();
    }

    if url_query.is_some() {
        filename.push('?');
        filename.push_str(url_query.unwrap());
    }

    if filename.len() > FILE_NAME_MAX_LENGTH {
        let digest = md5::compute(filename);
        filename = format!("{:x}.html", digest);
    }

    format!("{}{}/{}", url_domain, parent, filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_to_path_domain_only() {
        let str = super::to_path(&Url::parse("https://lwn.net/").unwrap());

        assert_eq!(str, "lwn.net/index.html");
    }

    #[test]
    fn url_to_path_domain_only_no_slash() {
        let str = super::to_path(&Url::parse("https://lwn.net").unwrap());

        assert_eq!(str, "lwn.net/index.html");
    }

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
    fn url_to_path_index_no_slash() {
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel").unwrap());

        assert_eq!(str, "lwn.net/Kernel/index_no_slash.html");
    }

    #[test]
    fn url_to_path_fragment() {
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel/#fragment").unwrap());

        assert_eq!(str, "lwn.net/Kernel/index.html");
    }

    #[test]
    fn url_to_path_to_long_md5() {
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.html").unwrap());

        assert_eq!(str, "lwn.net/Kernel/5ca82767de71fe8930587e82bb994903.html");
    }
}
