use crate::my_modules::{
    networking::{input::{
        encrypted::Encrypted,
        decrypted::*
    }, output::error::HttpError},
    crypto::{
        aes::CryptoAES,
        sha::Hashing,
        custom::encrypt_plugin_id,
        rsa::Crypto
    },
    utils::utils::cleanse
};

impl Decrypted {
    /**
     * Turns an Encrypted Object to a MyRequest object.
     */
    pub async fn new(encrypted: &Encrypted) -> Result<Decrypted, HttpError> {
        // decrypt AES key, decrypt data
        let decrypted_key_result = encrypted.key.rsa_decrypt();
        if decrypted_key_result.as_ref().is_err() {
            return Err(decrypted_key_result.unwrap_err())
        }
        let decrypted_data_result = encrypted.data.aes_decrypt(
            decrypted_key_result.unwrap(), 
            &encrypted.nonce
        );
        if decrypted_data_result.is_err() {
            return Err(decrypted_data_result.unwrap_err());
        }

        // make a Decrypted obj and do some simple validation before the Batch Get
        let serde_result: Result<Decrypted, serde_json::Error> = serde_json::from_str(&decrypted_data_result.unwrap());
        if serde_result.as_ref().is_err() {
            return Err((400, format!("Error with encrypted request parameters: {:?}", serde_result.unwrap_err())).into());
        }
        let mut request = serde_result.unwrap();
        // check first_name
        if request.first_name.is_none() {
            request.first_name = Some("Not disclosed".to_owned());
        }

        // check last_name
        if request.last_name.is_none() {
            request.last_name = Some("Not disclosed".to_owned());
        }

        // handle the email
        if request.email.is_none() {
            request.email = Some("Not disclosed".to_owned());
        }else{
            request.email = Some(request.email.unwrap().hash_email());
        }
        // cleanse and encrypt company ID and plugin IDs
        request.store_id = cleanse(&request.store_id, "", true);
        request.plugins
            .iter_mut()
            .for_each(|obj| 
                obj.id = encrypt_plugin_id(
                    &request.store_id, 
                    &cleanse(
                        &obj.id, 
                        "", 
                        true
                    )
                )
        );
        //return Err("Made it to 69".into());
        // error below here
        let validated = request.validate();
        if validated.as_ref().is_err() {
            return validated;
        }
        request = validated.unwrap();

        //return Err("Made it to 77".into());
        // error below here

        // do the batch_get_items request to initialize the data
        let b_get = request.batch_get().await;
        if b_get.is_err() {
            return Err(b_get.unwrap_err());
        }

        
        return Ok(request);
    }
}