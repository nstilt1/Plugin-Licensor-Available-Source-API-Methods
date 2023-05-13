use rusoto_dynamodb::{DynamoDbClient, BatchWriteItemInput, WriteRequest, PutRequest, DynamoDb};

use crate::my_modules::{
    networking::{input::{decrypted::*}, output::error::HttpError},
};

impl Decrypted {
    /**
     * Batch-writes any pertinent information
     */
    pub async fn batch_write(&self) -> Result<(), HttpError> {
        let client = DynamoDbClient::new(rusoto_core::Region::UsEast1);
        
        let user_write = WriteRequest {
            put_request: Some(PutRequest {
                item: self.user_item.to_owned().unwrap(),
            }), ..Default::default()
        };
        let license_write = WriteRequest {
            put_request: Some(PutRequest {
                item: self.license_item.to_owned().unwrap(),
            }), ..Default::default()
        };
        
        let mut req_items: HashMap<String, Vec<WriteRequest>> = HashMap::new();
        req_items.insert(LICENSE_TABLE_NAME.to_owned(), vec![license_write]);
        req_items.insert(USERS_TABLE_NAME.to_owned(), vec![user_write]);
        let batch_write_items = BatchWriteItemInput {
            request_items: req_items, ..Default::default()
        };
        let batch_out = client.batch_write_item(batch_write_items).await;
        if batch_out.as_ref().is_err() {
            return Err((500, format!("Error CLNB33: {}", &batch_out.unwrap_err().to_string())).into());
        }

        return Ok(());
    }
}