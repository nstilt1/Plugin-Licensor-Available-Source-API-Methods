use std::collections::HashMap;
use rusoto_dynamodb::{AttributeValue, DynamoDbClient, DynamoDb, GetItemInput};
use rand::prelude::*;
use super::crypto::private::encrypt_id;

pub static LICENSE_TABLE_NAME: &str = "Licenses";

pub fn create_permanent_secret() -> String {
    // let mut contains_profanity = true;

    let mut result = "".to_owned();
    let dict = "BCDFGHJLMNPQRSTVWXYZ256789".as_bytes();
    // block is used to prevent RNG code from interfering with
    // the async code
    let mut rng = rand::thread_rng();
    while result.len() != 5 {
        result.push(dict[rng.gen_range(0..dict.len())] as char);
    }
    return result.to_owned();
}


/**
 * If successful, it returns (bool, LicenseIndex, License_Hashmap)
 * The bool is true if the license already exists, and false if it does not
 * The License_Hashmap will return either the existing license key or a new one
 * The hashmap is just the partition key for the license
 */
pub async fn create_license_code(
    company: &str) 
-> Result<String, String> {

    let client = DynamoDbClient::new(rusoto_core::Region::UsEast1);

    // generate code and hashmap
    let dict = "BCDFGHJLMNPQRSTVWXYZ256789".as_bytes();

    // this variable is theoretically slightly more efficient 
    // than calling dict.len() repeatedly
    let dict_len = dict.len();


    let mut exists = true;
    let mut license_code_string = "".to_owned();

    let mut license_map: HashMap<String, AttributeValue> = HashMap::new();

    // this will generate license codes until it determines that it has
    // generated an unused license code
    while exists {
        let mut result = "".to_owned();
        // block is used to prevent RNG code from interfering with
        // the async code
        {
            let mut rng = rand::thread_rng();
            while result.len() != 20 {
                result.push(dict[rng.gen_range(0..dict_len)] as char);
            }
        }
        

        let joined = format!("{}{}", &company, &result);
        let encrypted_license_id = encrypt_id(&joined, true, true);
        
        // reset the license_map if the last license generated exists
        if license_map.contains_key("id1") {
            license_map = HashMap::new();
        }
        
        license_map.insert(
            "id1".to_owned(),
            AttributeValue {
                s: Some(encrypted_license_id.to_owned()),
                ..Default::default()
            }
        );
        license_code_string = encrypted_license_id.to_owned();
        license_map.insert(
            "id2".to_owned(),
            AttributeValue {
                s: Some("all".to_owned()),
                ..Default::default()
            }
        );
        
        let get_license_input = &client.get_item(
            GetItemInput {
                table_name: LICENSE_TABLE_NAME.to_owned(),
                key: license_map.to_owned(),
                consistent_read: Some(true),
                ..Default::default()
            }
        ).await;

        if get_license_input.is_err() {
            return Err(format!("Error 323: {}", get_license_input.as_ref().unwrap_err().to_string()));
        }

        let get_license_item = get_license_input.as_ref().unwrap().item.as_ref();
        if get_license_item.is_none() {
            exists = false;
        }
    }
    
    return Ok(license_code_string);
}