// aes stuff
use aes_gcm::{
    aead::{KeyInit, Aead, generic_array::GenericArray, Payload},
    Aes128Gcm, Nonce,
};
use rand::Rng;
//use hex;
use base64::{engine::general_purpose, Engine as _};

use crate::my_modules::networking::output::error::HttpError;

pub trait CryptoAES {
    fn aes_encrypt(&self) -> Result<([u8; 16], String, String), &str>;
    fn aes_decrypt(&self, key_bytes: Vec<u8>, nonce: &str) -> Result<String, HttpError>;
}
impl CryptoAES for String {
    /**
     * Encrypts data
     * Returns (key, nonce, encrypted string)
     */
    fn aes_encrypt(&self) -> Result<([u8; 16], String, String), &str> {
        let mut key_bytes = [0u8; 16];
        rand::thread_rng().fill(&mut key_bytes);

        let key = GenericArray::from_slice(&key_bytes);

        let cipher = Aes128Gcm::new(key);

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let nonce_64 = general_purpose::STANDARD.encode(nonce_bytes);

        let cipher_text_result = cipher.encrypt(
            nonce.into(), 
            Payload { 
                msg: self.as_bytes(), 
                aad: b"" 
            });
        if cipher_text_result.is_err() {
            return Err("Error A21");
        }
        let cipher_text = general_purpose::STANDARD.encode(cipher_text_result.unwrap());
        
        return Ok((key_bytes, nonce_64, cipher_text));
    }

    /**
     * Decrypts an encrypted json string given:
     * the key as a Vec<u8>
     * the nonce, and
     * the encrypted string
     * 
     * Returns a json string or an error message
     */
    fn aes_decrypt(&self, key_bytes: Vec<u8>, nonce: &str) -> Result<String, HttpError> {
        // let mut key_bytes = [0u8];
        let key_slice_result = general_purpose::STANDARD.decode(key_bytes);
        if key_slice_result.is_err() {
            return Err(format!("Error A41: {:?}", key_slice_result.unwrap_err()).into());
        }
        let key_bytes = key_slice_result.unwrap();

        // let mut nonce_bytes = [0u8; 12];
        let nonce_slice_result = general_purpose::STANDARD.decode(nonce);
        if nonce_slice_result.is_err() {
            return Err((500, format!("Error A46: {:?}" , nonce_slice_result.unwrap_err())).into());
        }
        let nonce_bytes = nonce_slice_result.unwrap();
        
        let key = GenericArray::from_slice(&key_bytes);
        let cipher = Aes128Gcm::new(key);
        
        let ciphertext_result = general_purpose::STANDARD.decode(self);
        if ciphertext_result.is_err() {
            return Err((500, "Error A59".to_string()).into());
        }
        let ciphertext = ciphertext_result.unwrap();

        let decrypt_result =  cipher.decrypt(Nonce::from_slice(&nonce_bytes), Payload { msg: &ciphertext, aad: b"" });
        
        if decrypt_result.is_err() {
            
            return Err(format!("Error A43: {:?}", decrypt_result.unwrap_err()).into());
        }
        let decrypt = decrypt_result.unwrap();
        let string_result = String::from_utf8(decrypt);
        if string_result.is_err() {
            return Err("Error A48".into());
        }
        return Ok(string_result.unwrap());
        
    }
}

