use std::path::Path;

use md5;
use url::Url;

///Max file name size supported by the file system
const FILE_NAME_MAX_LENGTH: usize = 255;

/// Convert an Url to the corresponding path
pub fn to_path(url: &Url, with_fragment: bool) -> String {
    let url_domain = url.host_str().unwrap();

    let mut url_path_and_query = url.path().to_string();
    if let Some(query) = url.query() {
        url_path_and_query.push_str("__querystring__");
        url_path_and_query.push_str(query);
    }

    let path = Path::new(&url_path_and_query);
    let mut filename = path.file_name().map_or(String::from(""), |filename| {
        filename.to_str().unwrap().to_string()
    });
    let mut parent = path
        .parent()
        .map_or("", |filename| filename.to_str().unwrap())
        .to_string();

    if url_path_and_query.ends_with('/') {
        filename = "index.html".to_string();
        parent = url_path_and_query.trim_end_matches('/').to_string();
    } else if Path::new(&filename).extension().is_none() {
        parent = url_path_and_query.trim_end_matches('/').to_string();
        filename = "index_no_slash.html".to_string();
    }

    if filename.len() > FILE_NAME_MAX_LENGTH {
        let digest = md5::compute(filename);
        filename = format!("{:x}.html", digest);
    }

    match (url.fragment(), with_fragment) {
        (Some(fragment), true) => format!("{}{}/{}#{}", url_domain, parent, filename, fragment),
        (_, _) => format!("{}{}/{}", url_domain, parent, filename),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_to_path_domain_only() {
        let str = super::to_path(&Url::parse("https://lwn.net/").unwrap(), false);

        assert_eq!(str, "lwn.net/index.html");
    }

    #[test]
    fn url_to_path_domain_only_no_slash() {
        let str = super::to_path(&Url::parse("https://lwn.net").unwrap(), false);

        assert_eq!(str, "lwn.net/index.html");
    }

    #[test]
    fn url_to_path() {
        let str = super::to_path(
            &Url::parse("https://lwn.net/Kernel/index.html").unwrap(),
            false,
        );

        assert_eq!(str, "lwn.net/Kernel/index.html");
    }

    #[test]
    fn url_to_path_index() {
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel/").unwrap(), false);

        assert_eq!(str, "lwn.net/Kernel/index.html");
    }

    #[test]
    fn url_to_path_index_no_slash() {
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel").unwrap(), false);

        assert_eq!(str, "lwn.net/Kernel/index_no_slash.html");
    }

    #[test]
    fn url_to_path_fragment() {
        let str = super::to_path(
            &Url::parse("https://lwn.net/Kernel/#fragment").unwrap(),
            true,
        );

        assert_eq!(str, "lwn.net/Kernel/index.html#fragment");
    }

    #[test]
    fn url_to_path_no_fragment() {
        let str = super::to_path(
            &Url::parse("https://lwn.net/Kernel/#fragment").unwrap(),
            false,
        );

        assert_eq!(str, "lwn.net/Kernel/index.html");
    }

    #[test]
    fn url_to_path_to_long_md5() {
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.html").unwrap(), false);

        assert_eq!(str, "lwn.net/Kernel/5ca82767de71fe8930587e82bb994903.html");
    }

    #[test]
    fn url_to_path_querystrings() {
        let str = super::to_path(
            &Url::parse(
                "https://google.com/foobar/platform-redirect/?next=/configuration/releases/",
            )
            .unwrap(),
            false,
        );
        assert_eq!(str, "google.com/foobar/platform-redirect/__querystring__next=/configuration/releases/index.html");
    }
}
