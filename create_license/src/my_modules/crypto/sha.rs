use crate::my_modules::utils::utils::clean_email;
use super::private::salt_email;

use openssl::sha::Sha256;
use sha3::{Digest, Sha3_512};

/**
 * Salts and hashes an email address
 */
impl Hashing for &str {
    fn hash_email(&self) -> String{
        let mut hasher = Sha3_512::new();

        // remove unnecessary chars
        let email = clean_email(self);

        // salt the email algorithmically
        let salted = salt_email(&email);

        hasher.update(salted.as_bytes());

        return format!("{:x}", hasher.finalize());
    }
    fn to_hash(&self) -> Vec<u8> {
        
        let mut hasher = Sha256::new();

        hasher.update(self.as_bytes());

        return hasher.finish().to_vec();
        
    }
}
impl Hashing for String {
    fn hash_email(&self) -> String {
        return self.as_str().hash_email();
    }
    fn to_hash(&self) -> Vec<u8> {
        return self.as_str().to_hash();
    }
}
/**
 * Hashes a string
 */

pub trait Hashing {
    fn hash_email(&self) -> String;
    fn to_hash(&self) -> Vec<u8>;
}