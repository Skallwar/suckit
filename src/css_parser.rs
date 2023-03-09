use cssparser;

pub struct css {}

impl css {
    pub fn new() -> css {
        css {}
    }

    pub fn parse(&self, css: &str) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        let parserinput = cssparser::ParserInput::new(css);

        let mut parser = cssparser::Parser::new(&mut parserinput);

        while let Ok(token) = parser.next() {
            match token {
                cssparser::Token::Url(url) => vec.push(url.to_string()),
                _ => (),
            }
        }

        vec
    }
}
