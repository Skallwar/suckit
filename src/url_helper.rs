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
pub fn to_path(url: &Url, current_path: Option<&str>) -> String {
    let fragment = url.fragment();
    let domain = url.domain().unwrap();
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
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel/").unwrap());

        assert_eq!(str, "lwn_net_Kernel");
    }

    #[test]
    fn url_to_path_fragment() {
        let str = super::to_path(&Url::parse("https://lwn.net/Kernel/#fragment").unwrap());

        assert_eq!(str, "lwn_net_Kernel#fragment");
    }

    #[test]
    fn url_to_path_long() {
        let str = super::to_path(&Url::parse("https://e8v0pez1lofdxoxgg5vwrnaqkjuvpowp9wtgc2eknlfpjdwmmfti8fcwyjzfdgys3nrgyqyeqjkulpyg9kfiqajza2bwxkinhhpohyrnnoy2bak374tcaxh1ycpboolmx8so9yq9kbcj5wu5cgymqndeqasdak0nvl0ijka6fkkmhhvt43l73bn38rewicd4h1ff2omhpni752jtqyzsjub5coh8dlnr3i35udmkzhxo4db3is9gnqmf3hl.comtest").unwrap());

        assert_eq!(str, "e8v0pez1lofdxoxgg5vwrnaqkjuvpowp9wtgc2eknlfpjdwmmfti8fcwyjzfdgys3nrgyqyeqjkulpyg9kfiqajza2bwxkinhhpohyrnnoy2bak374tcaxh1ycpboolmx8so9yq9kbcj5wu5cgymqndeqasdak0nvl0ijka6fkkmhhvt43l73bn38rewicd4h1ff2omhpni752jtqyzsjub5coh8dlnr3i35udmkzhxo4db3is9gnqmf3hl_com");
    }
}
