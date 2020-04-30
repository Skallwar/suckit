use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use url::Url;

const FILE_NAME_MAX_LENGTH: usize = 255;
const FRAGMENT: &AsciiSet = &CONTROLS.add(b'?');

///Encode special character with '%' representation
pub fn encode(path: &str) -> String {
    utf8_percent_encode(path, FRAGMENT).to_string()
}

///Convert a str to an Url
pub fn to_path(url: &Url) -> String {
    let url = url.as_str().split("://").collect::<Vec<&str>>()[1];

    let mut url = url.replace('/', "_").replace('.', "_");
    if url.len() >= FILE_NAME_MAX_LENGTH {
        url = url.split_at(FILE_NAME_MAX_LENGTH).0.to_string(); //Shrink too long file name
    }
    let url = url.trim_end_matches('_'); //Remaining '/'

    url.to_string()
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
    fn url_to_path_long() {
        let str = super::to_path(&Url::parse("https://e8v0pez1lofdxoxgg5vwrnaqkjuvpowp9wtgc2eknlfpjdwmmfti8fcwyjzfdgys3nrgyqyeqjkulpyg9kfiqajza2bwxkinhhpohyrnnoy2bak374tcaxh1ycpboolmx8so9yq9kbcj5wu5cgymqndeqasdak0nvl0ijka6fkkmhhvt43l73bn38rewicd4h1ff2omhpni752jtqyzsjub5coh8dlnr3i35udmkzhxo4db3is9gnqmf3hl.comtest").unwrap());

        assert_eq!(str, "e8v0pez1lofdxoxgg5vwrnaqkjuvpowp9wtgc2eknlfpjdwmmfti8fcwyjzfdgys3nrgyqyeqjkulpyg9kfiqajza2bwxkinhhpohyrnnoy2bak374tcaxh1ycpboolmx8so9yq9kbcj5wu5cgymqndeqasdak0nvl0ijka6fkkmhhvt43l73bn38rewicd4h1ff2omhpni752jtqyzsjub5coh8dlnr3i35udmkzhxo4db3is9gnqmf3hl_com");
    }
}
