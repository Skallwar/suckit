/// Separates HTML responses and other content (PDFs, images...)
pub enum ResponseData {
    Html(String),
    Other(Vec<u8>),
}

/// Wrapper around `ResponseData`
pub struct Response {
    pub data: ResponseData,
    pub filename: Option<String>,
}

impl Response {
    pub fn new(data: ResponseData, filename: Option<String>) -> Response {
        Response { data, filename }
    }
}
