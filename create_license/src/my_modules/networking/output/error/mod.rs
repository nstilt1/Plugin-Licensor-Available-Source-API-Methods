#[derive(Debug)]
pub struct HttpError {
    code: u16,
    message: String
}

const debug_mode: bool = true;

const DEFAULT_ERROR_STATUS_CODE: u16 = 500;

impl HttpError {
    /**
     * Turns an error into a 202 error
     */
    pub fn _202(&self, new_code: &str) -> Self {
        HttpError {
            code: 202,
            message: format!(
                "We encountered an error, but your request went through. Error {}:{}", 
                new_code, 
                &self.message
            ),
        }
    }
}

pub mod new;
pub mod respond;
pub mod into;