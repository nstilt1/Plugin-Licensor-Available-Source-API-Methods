use crate::my_modules::networking::{input::{encrypted::*}, output::error::HttpError};

impl Encrypted {
    pub fn new(json: &str) -> Result<Self, HttpError> {
        let serde_result: Result<Encrypted, serde_json::Error> = serde_json::from_str(json);
        if serde_result.as_ref().is_err() {
            return Err((400, format!("Error: {:?}", serde_result.unwrap_err())).into());
        }
        return Ok(serde_result.unwrap());
    }
}