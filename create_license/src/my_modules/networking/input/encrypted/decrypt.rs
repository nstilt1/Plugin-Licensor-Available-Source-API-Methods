use crate::my_modules::{
    networking::{input::{
        decrypted::*,
        encrypted::*
    }, output::error::HttpError}
};
use std::time::{SystemTime, UNIX_EPOCH};

impl Encrypted {
    /**
     * Decrypt the request and validate it
     */
    pub async fn decrypt(&self) -> Result<Decrypted, HttpError> {
        let current_time = SystemTime::duration_since (&SystemTime::now(), UNIX_EPOCH).unwrap().as_secs();
        if self.timestamp.parse::<u64>().unwrap() < current_time - 300 {
            return Err((400, "Error: Timestamp invalid").into());
        }
        let signature = self.signature.to_owned();
        let signed_stuff = format!("{}{}{}{}", self.data, self.nonce, self.key, self.timestamp);
        let result = Decrypted::new(self).await;
        if result.as_ref().is_err() {
            return Err(result.unwrap_err());
        }
        
        let req = result.unwrap();
        let verified = req.verify_signature_db(&signature, &signed_stuff);
        if verified.as_ref().is_err() {
            return Err(verified.unwrap_err());
        }
        return Ok(req);
    }
}