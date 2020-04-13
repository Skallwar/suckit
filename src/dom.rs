use std::ops::Deref;

use kuchiki::traits::*;

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
                None => match unsafe { (*attributes).get_mut("href") } {
                    Some(url) => vec.push(url),
                    None => (),
                },
            }
        }

        return vec;
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
