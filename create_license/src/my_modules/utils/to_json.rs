use crate::my_modules::networking::output::error::HttpError;
use serde::Serialize;


pub trait ToJson {
    fn to_json(&self) -> Result<String, HttpError>;
}

impl<T: Serialize> ToJson for T {
    fn to_json(&self) -> Result<String, HttpError> {
        let result = serde_json::to_string(self);
        if result.as_ref().is_err() {
            return Err((500, format!("error CLT15: {}", result.unwrap_err().to_string())).into());
        }
        return Ok(result.unwrap());
    }
}