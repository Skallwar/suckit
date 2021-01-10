use url::Url;

/// Convert an Url to the corresponding path
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
