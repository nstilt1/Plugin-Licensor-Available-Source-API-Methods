use super::{HttpError, debug_mode};
use lambda_http::{Response, Body, Error};
use substring::Substring;
use crate::my_modules::utils::utils::error_resp;

impl HttpError {
    /**
     * Responds. Turn off debug mode once RSA/AES is working correctly.
     */
    pub fn respond(&self) -> Result<Response<Body>, Error> {
        if self.code == 0 && !debug_mode {
            let colon = self.message.find(':');
            let error_message: &str;
            if colon.is_some() {
                error_message = self.message.substring(0, colon.unwrap());
            }else if self.message.len() < 14 {
                error_message = &self.message;
            }else{
                error_message = "Error 0";
            }
            return error_resp(500, error_message);
        }else if self.code == 0 {
            return error_resp(500, &self.message);
        }
        return error_resp(self.code, &self.message);
    }
}