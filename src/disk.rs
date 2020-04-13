use std::fs;
use std::io::Write;
use std::path::PathBuf;

use reqwest::Url;

const FILE_NAME_MAX_LENGTH: usize = 255;

//TODO: Recover insted of panic
pub fn save_file(file_name: &String, content: &String, path: &Option<PathBuf>) {
    let path = match path {
        Some(path) => path.join(file_name),
        None => PathBuf::from(file_name),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_to_path() {
        let str = super::url_to_path(&Url::parse("https://lwn.net/Kernel/").unwrap());

        assert_eq!(str, "lwn_net_Kernel");
    }

    #[test]
    fn url_to_path_long() {
        let str = super::url_to_path(&Url::parse("https://e8v0pez1lofdxoxgg5vwrnaqkjuvpowp9wtgc2eknlfpjdwmmfti8fcwyjzfdgys3nrgyqyeqjkulpyg9kfiqajza2bwxkinhhpohyrnnoy2bak374tcaxh1ycpboolmx8so9yq9kbcj5wu5cgymqndeqasdak0nvl0ijka6fkkmhhvt43l73bn38rewicd4h1ff2omhpni752jtqyzsjub5coh8dlnr3i35udmkzhxo4db3is9gnqmf3hl.comtest").unwrap());

        assert_eq!(str, "e8v0pez1lofdxoxgg5vwrnaqkjuvpowp9wtgc2eknlfpjdwmmfti8fcwyjzfdgys3nrgyqyeqjkulpyg9kfiqajza2bwxkinhhpohyrnnoy2bak374tcaxh1ycpboolmx8so9yq9kbcj5wu5cgymqndeqasdak0nvl0ijka6fkkmhhvt43l73bn38rewicd4h1ff2omhpni752jtqyzsjub5coh8dlnr3i35udmkzhxo4db3is9gnqmf3hl_com");
    }
}
