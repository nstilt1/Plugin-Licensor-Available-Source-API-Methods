use crate::my_modules::{
    networking::{input::{
        decrypted::*,
    }, output::error::HttpError},
    utils::{
        maps::*
    }
};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput};
use rusoto_core::Region;



impl Decrypted {
    /**
     * Gets the license item for the user.
     * Returns error if self.license_index hasn't been set
     */
    pub async fn get_license_item(&self) -> Result<HashMap<String, AttributeValue>, HttpError> {
        if self.license_index.as_ref().is_some() && self.license_item.is_none() {
            // get the license from the DB
            let client = DynamoDbClient::new(Region::UsEast1);
            let out_result = client.get_item(
                GetItemInput {  
                    consistent_read: Some(true),  
                    key: HashMap::new_map(vec![
                        ("id1", self.license_index.as_ref().unwrap()),
                        ("id2", "all")
                    ]),
                    table_name: LICENSE_TABLE_NAME.to_owned(),
                    ..Default::default() 
                }
            ).await;
            if out_result.as_ref().is_err() {
                return Err((500, "Error CLMR337wr").into());
            }
            let out = out_result.unwrap().item;
            if out.is_none() {
                return Err((500, "Error CLMR341fj").into());
            }
            return Ok(out.unwrap().to_owned());
        }

        return Err((500, "Error CLMR347uj").into());
    }
}