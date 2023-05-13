use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Encrypted {
    pub data: String,
    pub nonce: String,
    pub key: String,
    pub timestamp: String,
    pub signature: String,
}

pub mod new;
pub mod decrypt;