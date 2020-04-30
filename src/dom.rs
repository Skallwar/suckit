use std::ops::Deref;

use kuchiki::traits::*;

///Struct containing a dom tree of a web page
pub struct Dom {
    tree: kuchiki::NodeRef,
}

impl Dom {
    ///Create a new dom tree
    pub fn new(str: &str) -> Dom {
        Dom {
            tree: kuchiki::parse_html().one(str),
        }
    }

    ///Serialize the dom tree
    pub fn serialize(&self) -> String {
        let mut vec: Vec<u8> = Vec::new();

        if let Err(err) = self.tree.serialize(&mut vec) {
            panic!("Couldn't serialize domtree: {}", err)
        }

        String::from_utf8(vec).unwrap()
    }

    ///Returns all urls in the dom tree
    pub fn find_urls_as_strings(&self) -> Vec<&mut String> {
        let mut vec: Vec<&mut String> = Vec::new();

        let nodes = match self.tree.select("[src],[href]") {
            Ok(nodes) => nodes,
            Err(_) => return vec,
        };

        for node in nodes {
            let attributes = node.deref().attributes.as_ptr();

            //TODO: Prettify this, we may need more than src and href in the futur
            match unsafe { (*attributes).get_mut("src") } {
                Some(url) => vec.push(url),
                None => {
                    if let Some(url) = unsafe { (*attributes).get_mut("href") } {
                        vec.push(url)
                    }
                }
            }
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _dom = Dom::new("<html></html>");
    }

    #[test]
    fn find_urls_as_strings() {
        let url1 = "https://upload.wikimedia.org/wikipedia/commons/thumb/3/34/Anser_anser_1_%28Piotr_Kuczynski%29.jpg/800px-Anser_anser_1_%28Piotr_Kuczynski%29.jpg";
        let url2 = "test";
        let dom = Dom::new("<img src=https://upload.wikimedia.org/wikipedia/commons/thumb/3/34/Anser_anser_1_%28Piotr_Kuczynski%29.jpg/800px-Anser_anser_1_%28Piotr_Kuczynski%29.jpg>
            <img src=test>");
        let vec = dom.find_urls_as_strings();

        assert_eq!(vec[0], url1);
        assert_eq!(vec[1], url2);
    }
}
