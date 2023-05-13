use std::collections::HashMap;

use rusoto_core::RusotoError;
use rusoto_dynamodb::{AttributeValue, BatchGetItemOutput, BatchGetItemError};

use crate::my_modules::networking::output::error::HttpError;


pub trait BatchGetUtils {
    fn get_item_vec_map(&self) -> Result<HashMap<String, Vec<HashMap<String, AttributeValue>>>, HttpError>;
}

impl BatchGetUtils for Result<BatchGetItemOutput, RusotoError<BatchGetItemError>> {
    fn get_item_vec_map(&self) -> Result<HashMap<String, Vec<HashMap<String, AttributeValue>>>, HttpError> {
        if self.as_ref().is_err() {
            let err = self.as_ref().unwrap_err().to_string();
            return Err((500, format!("Error CLBGU15: {}", err.as_str())).into());
            
        }
        let responses_opt = &self.as_ref().unwrap().responses;

        if responses_opt.as_ref().is_none() {
            return Err((403, "Invalid request.").into());
        }

        return Ok(responses_opt.as_ref().unwrap().to_owned());
    }
}
