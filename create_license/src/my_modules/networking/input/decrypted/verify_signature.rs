use crate::my_modules::{
    networking::{input::decrypted::*, output::error::HttpError},
    utils::maps::*,
    crypto::rsa::verify_signature
};
use std::collections::HashMap;

impl Decrypted {
    /**
     * Verifies the signature by looking up the key stored in company_item, then
     * it returns the company table item as a HashMap in case it is needed
     */
    pub fn verify_signature_db(&self, signature: &str, signed_stuff: &str) -> Result<(),HttpError> {
        
        //let batch_get = self.batch_get().await;
        let company_opt = &self.company_item.to_owned();

        let company_item: HashMap<String, AttributeValue>;
        
        if company_opt.as_ref().is_some() {
            company_item = company_opt.to_owned().unwrap().to_owned();
        }else{
            // company_id did not belong to any company
            return Err((403, "Forbidden").into());
        }

        let pub_key_result = company_item.get_data("publicKey", S);
        if pub_key_result.as_ref().is_err() {
            return Err(pub_key_result.unwrap_err());
        }

        let verification = verify_signature(
            &pub_key_result.unwrap(), 
            signed_stuff, 
            signature
        );

        if verification.as_ref().is_err() {
            return Err(verification.unwrap_err().into());
        }

        return Ok(());
    }
}