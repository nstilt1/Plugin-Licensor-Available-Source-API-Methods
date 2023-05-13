use super::HttpError;

impl HttpError {
    pub fn new(c: u16, m: &str) -> Self {
        HttpError {
            code: c,
            message: m.to_owned()
        }
    }
}