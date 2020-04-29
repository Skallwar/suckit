use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use url::{ParseError, Url};

const FILE_NAME_MAX_LENGTH: usize = 255;
const FRAGMENT: &AsciiSet = &CONTROLS.add(b'?');

pub fn str_percent_encode(path: &str) -> String {
    utf8_percent_encode(path, FRAGMENT).to_string()
}

pub fn str_to_url(path: &str) -> Result<Url, ParseError> {
    let path = str_percent_encode(path);

    Url::parse(&path)
}

pub fn url_to_path(url: &Url) -> String {
    let scheme_size = url.scheme().len() + 3; // 3 = "://".len()
    let url = url.as_str();

    let mut url = url.replace('/', "_").replace('.', "_");
    url.replace_range(0..scheme_size, ""); //Strip scheme
    if url.len() >= FILE_NAME_MAX_LENGTH {
        url.replace_range(FILE_NAME_MAX_LENGTH.., ""); //Shrink too long file name
    }
    let url = url.trim_end_matches('_'); //Remaining '/'

    return url.to_string();
}
