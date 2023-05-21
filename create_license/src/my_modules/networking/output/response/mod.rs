use std::time::{SystemTime, UNIX_EPOCH};

use serde_derive::{Deserialize, Serialize};

use crate::my_modules::{
    utils::{maps::{Maps, S}, to_json::ToJson}, 
    networking::input::decrypted::Decrypted, 
    crypto::{
        aes::CryptoAES, 
        rsa::Crypto, 
        custom::decrypt_license_code
    },
};

use super::{error::HttpError, license::License};

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    data: String,
    nonce: String,
    key: String,
    timestamp: String,
    signature: String,
}

impl HttpResponse {
    pub fn new(d: Decrypted) -> Result<HttpResponse, HttpError> {
        // initialize a license Object
        let license_code = decrypt_license_code(&d.license_index.unwrap());

        let user = d.user_item.unwrap();
        let user_licenses = user.get_m("licenses",
            "Error CLOR35");
        if user_licenses.as_ref().is_err() {return Err(user_licenses.unwrap_err());}

        let license_init = License::init_license(
            &license_code, 
            user_licenses.unwrap()
        );
        if license_init.as_ref().is_err() {
            return Err(license_init.unwrap_err());
        }

        let license_json = license_init.unwrap().to_json();

        if license_json.as_ref().is_err() {return Err(license_json.unwrap_err());}

        // aes encrypt and then rsa encrypt the aes key
        let aes_result = license_json.as_ref().unwrap().aes_encrypt();
        if aes_result.as_ref().is_err() {
            return Err(aes_result.unwrap_err());
        }

        let pub_key_result = d.company_item.unwrap().get_data("publicKeyA", S, "CLNOR54");
        if pub_key_result.as_ref().is_err() {
            return Err((500, "Error CLOR57s").into());
        }
        
        let aes_stuff = aes_result.unwrap();
        let aes_key_result = pub_key_result.unwrap().rsa_encrypt(aes_stuff.0);
        if aes_key_result.as_ref().is_err() {return Err(aes_key_result.unwrap_err());}
        
        let aes_key = aes_key_result.unwrap();
        let aes_nonce = &aes_stuff.1;
        let data = aes_stuff.2;
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
        let sign_result = format!("{}{}{}{}", &data, &aes_nonce, &aes_key, &time).sign();
        if sign_result.as_ref().is_err() {
            return Err(sign_result.unwrap_err());
        }
        return Ok(HttpResponse { 
            data, 
            nonce: aes_nonce.to_string(), 
            key: aes_key.to_owned(),
            timestamp: time, 
            signature: sign_result.unwrap()
        });
    }
}