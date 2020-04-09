use kuchiki::traits::*;
use kuchiki::NodeData::*;
use kuchiki::NodeRef;

///Struct containing a dom tree of a web page
pub struct Dom {
    pub tree: kuchiki::NodeRef,
}

impl Dom {
    pub fn new(str: &str) -> Dom {
        Dom {
            tree: kuchiki::parse_html().one(str),
        }
    }

    pub fn find_urls_string(&self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.append(&mut self.find_src_values());

        return vec;
    }

    fn find_src_values(&self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        let nodes = match self.tree.select("[src]") {
            Ok(nodes) => nodes,
            Err(_) => return vec,
        };

        for node in nodes {
            let element_data = match node.as_node().as_element() {
                Some(data) => data,
                None => continue,
            };

            let url: String = match element_data.attributes.borrow().get("src") {
                Some(url) => String::from(url),
                None => continue,
            };

            vec.push(url.clone());
        }

        return vec;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dom_tree() {
        let dom = Dom::new("<html></html>");
    }

    #[test]
    fn test_find_urls() {
        let url1 = "https://upload.wikimedia.org/wikipedia/commons/thumb/3/34/Anser_anser_1_%28Piotr_Kuczynski%29.jpg/800px-Anser_anser_1_%28Piotr_Kuczynski%29.jpg";
        let url2 = "test";
        let dom = Dom::new("<img src=https://upload.wikimedia.org/wikipedia/commons/thumb/3/34/Anser_anser_1_%28Piotr_Kuczynski%29.jpg/800px-Anser_anser_1_%28Piotr_Kuczynski%29.jpg>
            <img src=test>");
        let vec = dom.find_urls_string();

        assert_eq!(vec[0], url1);
        assert_eq!(vec[1], url2);
    }
}
