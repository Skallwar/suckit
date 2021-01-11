use std::path::Path;

use url::Url;

/// Convert an Url to the corresponding path
pub fn to_path(url: &Url) -> String {
    let domain = url.host_str().unwrap();
    let path = url.path();
    let query = url.query();

    let path_str = format!("{}{}", domain, path);
    let path = Path::new(&path_str);

    let mut path = if path.ends_with("/") {
        format!("{}index.html", path_str)
    } else {
        match path.extension() {
            None => format!("{}/index_no_slash.html", path_str),
            _ => path_str,
        }
    };

    if query.is_some() {
        path = format!("{}?{}", path, query.unwrap());
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
