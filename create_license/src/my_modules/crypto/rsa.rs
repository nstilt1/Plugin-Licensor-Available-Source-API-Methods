extern crate openssl;

use openssl::rsa::{Rsa, Padding};

use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Verifier;
use openssl::sign::Signer;

use crate::my_modules::utils::to_base64::*;

use crate::my_modules::networking::output::error::HttpError;

use super::sha::Hashing;

pub trait Crypto {
    fn rsa_decrypt (&self) -> Result<Vec<u8>, HttpError>;
    fn sign(&self) -> Result<String, HttpError>;
    fn rsa_encrypt(&self, data: [u8; 16]) -> Result<String, String>;
}
impl Crypto for String {

    /**
     * Decrypts an AES Private Key using our private key.
     * Could return a 500 server error
     */
    fn rsa_decrypt (&self) -> Result<Vec<u8>, HttpError> {
        let private_key = super::private::PRIVATE_KEY;

        //let private_pkey = PKey::private_key_from_pem(&private_string.as_bytes()).unwrap();
        let private_rsa_result = Rsa::private_key_from_pem(&private_key.as_bytes());
        if private_rsa_result.is_err() {
            return Err((0, format!("Error R41: {:?}", private_rsa_result.unwrap_err())).into());
        }
        let private_rsa = private_rsa_result.unwrap();

        // convert encrypted key from base64 to bytes
        let key_slice_result = self.from_base64();
        if key_slice_result.is_err() {
            return Err((0, format!("Error A41: {:?}", key_slice_result.unwrap_err())).into());
        }
        let key_bytes = key_slice_result.unwrap();

        let mut buf = vec![0; private_rsa.size() as usize];
        let decrypt_result = private_rsa.private_decrypt(&key_bytes, &mut buf, Padding::PKCS1);
        if decrypt_result.is_err() {
            return Err((0,format!("Error R57: {:?}", decrypt_result.unwrap_err())).into());
        }
        //let decrypted_bytes_amt = decrypt_result.unwrap();
        return Ok(buf.to_owned());
    }
    /**
     * Encrypts string, the string is usually an AES Key.
     */
    fn rsa_encrypt (&self, data: [u8; 16]) -> Result<String, String> {
        let pubkey_result = Rsa::public_key_from_pem(self.as_bytes());
        if pubkey_result.is_err() {
            return Err(format!("Error CCR78: {:?}", pubkey_result.unwrap_err()));
        }
        let pubkey = pubkey_result.unwrap();
        let mut buf = vec![0; 2048];
        let encrypted = pubkey.public_encrypt(&data, &mut buf, Padding::PKCS1);
        if encrypted.is_err() {
            return Err(format!("Error CCR84: {:?}", encrypted.unwrap_err()));
        }
        
        let encoded = buf.to_base64();
        return Ok(encoded);
    }
    /**
     * Signs data with our private key.
     */
    fn sign(&self) -> Result<String, HttpError> {
        let private = super::private::PRIVATE_KEY;
        let private_pkey = PKey::private_key_from_pem(&private.as_bytes());
        if private_pkey.is_err() {
            return Err((500, format!("Error CLRS18e: {}", private_pkey.unwrap_err())).into());
        }
        let private_p = private_pkey.unwrap();
        let signer_res = Signer::new(MessageDigest::sha256(), &private_p);
        if signer_res.as_ref().is_err() {
            return Err((500, "Error CLCR23").into());
        }
        let mut signer = signer_res.unwrap();
        let mut buf: Vec<u8> = vec![0; private_p.size() * 8];
        let update = signer.update(self.as_bytes());
        if update.as_ref().is_err() {
            return Err((500, format!("CLCR38: {}", update.unwrap_err())).into());
        }
        let sign_result = signer.sign(&mut buf);
        if sign_result.as_ref().is_err() {
            return Err((500, format!("CLCR42: {}", sign_result.unwrap_err())).into());
        }
        let num_bytes_signed = sign_result.unwrap();
        return Ok(buf[0..num_bytes_signed].to_base64());
    }
}


/**
 * Hashes signed_stuff, then it takes a signature and compares it
 */
pub fn verify_signature(public_key: &str, signed_stuff: &str, signature: &str)
-> Result<(), HttpError> {
    //let public_key = public_key_opt.unwrap().s.as_ref().unwrap();
    
    let public_pkey = PKey::public_key_from_pem(&public_key.to_owned().as_bytes()).unwrap();
            
    let mut verifier = Verifier::new(MessageDigest::sha256(), &public_pkey).unwrap();
    let hash = signed_stuff.to_hash();
    
    verifier.update(&hash).unwrap();

    let sig = signature.from_base64();
    if sig.is_err() {
        return Err((400, "Error: Signature must be base64 encoded.").into())
    }

    if !verifier.verify(sig.unwrap().as_slice()).unwrap() {
        return Err((403, "You shall not pass".to_owned()).into());
    }

    return Ok(());
}



pub fn validate_key_size(pub_key: &str, bit_requirement: u32)
 -> Result<(), String> {
    let rsa_result = Rsa::public_key_from_pem(pub_key.as_bytes());
    if rsa_result.is_err() {
        return Err(rsa_result.unwrap_err().to_string());
    }
    let rsa = rsa_result.unwrap();
    if rsa.size() * 8 != bit_requirement {
        return Err(format!("Error CCR18: Key needs to be {:?} bits", &bit_requirement));
    }
    return Ok(());
}



