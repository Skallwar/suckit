/// Separates HTML responses and other content (PDFs, images...)
pub enum ResponseData {
    Html(Vec<u8>),
    Css(Vec<u8>),
    Other(Vec<u8>),
}

/// Wrapper around `ResponseData`
pub struct Response {
    pub data: ResponseData,
    pub filename: Option<String>,
    pub charset: Option<String>,
}

impl Response {
    ///Create a new Response
    pub fn new(data: ResponseData, filename: Option<String>, charset: Option<String>) -> Response {
        Response {
            data,
            filename,
            charset,
        }
    }
}
