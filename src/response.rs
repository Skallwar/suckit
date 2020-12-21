/// Separates HTML responses and other content (PDFs, images...)
pub enum ResponseData {
    Html(String),
    Other(Vec<u8>),
}

/// Wrapper around `ResponseData`
pub struct Response {
    pub data: ResponseData,
    pub charset: Option<String>,
    pub filename: Option<String>,
}

impl Response {
    ///Create a new Response
    pub fn new(data: ResponseData, charset: Option<String>, filename: Option<String>) -> Response {
        Response {
            data,
            charset,
            filename,
        }
    }
}
