use lambda_http::{Body, Request, RequestExt};

use crate::my_modules::networking::output::error::HttpError;

/**
 * Trait to handle a lambda_http::Request
 */
pub trait MagicalProcess {
    /**
     * Validate the request, returns the body's text contents.
     */
    fn extract_json(&self) -> Result<String, HttpError>;
}

impl MagicalProcess for Request {
    /**
     * Convert a request to JSON, and only allows POST methods
     */
    fn extract_json(&self) -> Result<String, HttpError> {
        // only allow post methods
        if self.method() != lambda_http::http::Method::POST {
            return Err((403, "Only POST Requests allowed.").into());
        }
        // no query strings this time
        let query_string = self.query_string_parameters();
        if !query_string.is_empty() {
            return Err((403, "Query strings shall not pass.").into());
        }

        // get contents
        if let Body::Text(contents) = self.body() {
            return Ok(contents.to_string());
        }
        return Err((400, "Body has no text contents.").into());
    }
}