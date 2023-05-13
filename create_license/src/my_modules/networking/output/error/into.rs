use super::{HttpError, DEFAULT_ERROR_STATUS_CODE};



impl From<(u16, &str)> for HttpError {
    fn from(tuple: (u16, &str)) -> Self {
        HttpError::new(tuple.0, tuple.1)
    }
}

impl From<(u16, String)> for HttpError {
    fn from(tuple: (u16, String)) -> Self {
        HttpError::new(tuple.0, tuple.1.as_str())
    }
}

impl From<&str> for HttpError {
    /**
     * Allows creating the default error with just the error message.
     */
    fn from(error: &str) -> Self {
        HttpError::new(DEFAULT_ERROR_STATUS_CODE, error)
    }
}

impl From<String> for HttpError {
    /**
     * Allows creating the default error with just the error message.
     */
    fn from(error: String) -> Self {
        HttpError::new(DEFAULT_ERROR_STATUS_CODE, &error)
    }
}