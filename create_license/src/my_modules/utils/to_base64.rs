use base64::{engine::general_purpose, Engine as _};


use crate::my_modules::networking::output::error::HttpError;




pub trait Base64String {
    fn from_base64(&self) -> Result<Vec<u8>, HttpError>;
}
pub trait Base64Vec {
    fn to_base64(&self) -> String;
}
impl Base64Vec for [u8] {
    fn to_base64(&self) -> String {
        return general_purpose::STANDARD.encode(self.to_vec());
    }
}
impl Base64Vec for Vec<u8> {
    fn to_base64(&self) -> String {
        return general_purpose::STANDARD.encode(self);
    }
}
impl Base64String for String {
    
    fn from_base64(&self) -> Result<Vec<u8>, HttpError> {

        let result = general_purpose::STANDARD.decode(self);
        if result.as_ref().is_err() {
            return Err((500, format!("Error CLV27: {}", result.unwrap_err())).into());
        }
        return Ok(result.unwrap());
    }
}

impl Base64String for &str {
    fn from_base64(&self) -> Result<Vec<u8>, HttpError> {
        let result = general_purpose::STANDARD.decode(self);
        if result.as_ref().is_err() {
            return Err((500, format!("Error CLV27: {}", result.unwrap_err())).into());
        }
        return Ok(result.unwrap());
    }
}