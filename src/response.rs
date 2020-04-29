/// Separates HTML responses and other content (PDFs, images...)
pub enum ResponseData {
    Html(String),
    Other(Vec<u8>),
}

/// Wrapper around `ResponseData`
pub struct Response {
    data: ResponseData,
    filename: Option<String>,
}

impl Response {
    pub fn new(data: ResponseData, filename: Option<String>) -> Response {
        Response {
            data,
            filename,
        }
    }

    pub fn get_data(&self) -> &ResponseData {
        &self.data
    }

    pub fn get_filename(&self) -> &Option<String> {
        &self.filename
    }
}
