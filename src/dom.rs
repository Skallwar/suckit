use std::ops::Deref;

use kuchiki::traits::*;

use crate::error;

static CSS_SELECTORS: &str = "[src],[href]";
static CSS_ATTRIBUTES: [&str; 2] = ["src", "href"];

///Struct containing a dom tree of a web page
pub struct Dom {
    tree: kuchiki::NodeRef,
}

impl Dom {
    pub fn new(str: &str) -> Dom {
        Dom {
            tree: kuchiki::parse_html().one(str),
        }
    }

    pub fn serialize(&self) -> String {
        let mut vec: Vec<u8> = Vec::new();

        if let Err(err) = self.tree.serialize(&mut vec) {
            error!("Couldn't serialize domtree: {}", err)
        }

        String::from_utf8(vec).unwrap()
    }

    pub fn find_urls_as_strings(&self) -> Vec<&mut String> {
        let mut vec: Vec<&mut String> = Vec::new();

        let nodes = match self.tree.select(CSS_SELECTORS) {
            Ok(nodes) => nodes,
            Err(_) => return vec,
        };

        for node in nodes {
            let attributes = node.deref().attributes.as_ptr();
            for attribute in CSS_ATTRIBUTES.iter() {
                if let Some(url) = unsafe { (*attributes).get_mut(*attribute) } {
                    vec.push(url);
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
