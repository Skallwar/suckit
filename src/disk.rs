use std::fs;
use std::io::Write;
use std::path::PathBuf;

use reqwest::Url;

//TODO: Recover insted of panic
pub fn save_to_disk(url: &Url, content: &String, path: &Option<PathBuf>) {
    let path_url = url_to_path(url);
    let path = match path {
        Some(path) => path.join(path_url),
        None => PathBuf::new().join(path_url),
    };

    match path.parent() {
        Some(parent) => match fs::create_dir_all(parent) {
            Err(err) => panic!("Couldn't create folder {}: {}", parent.display(), err),
            Ok(()) => (),
        },
        None => (),
    }

    let mut file = match fs::File::create(&path) {
        Err(err) => panic!("Couldn't create {}: {}", path.display(), err),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(err) => panic!("Couldn't write to {}: {}", path.display(), err),
        Ok(_) => (),
    };
}

fn url_to_path(url: &Url) -> String {
    let scheme_size = url.scheme().len() + 3; // 3 = "://".len()
    let url = url.as_str();

    let mut url = url.replace('/', "_").replace('.', "_");
    url.replace_range(0..scheme_size, ""); //Strip scheme
    let url = url.trim_end_matches('_'); //Remaining '/'

    return url.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_to_path() {
        let str = super::url_to_path(&Url::parse("https://lwn.net/Kernel/").unwrap());

        assert_eq!(str, "lwn_net_Kernel");
    }
}
