use crate::my_modules::{
    networking::{input::{
        decrypted::*,
    }, output::error::HttpError},
    utils::{
        maps::*
    },
    db::{
        create_permanent_secret
    }
};
use std::collections::HashMap;
use rusoto_dynamodb::AttributeValue;

impl Decrypted {
    /**
     * Initialize a license with blank Plugins map.
     * self.license_item should be empty,
     * self.license_index should be set,
     * and self.email should be set before running this
     */
    pub fn create_license_item(&self) -> Result<HashMap<String,AttributeValue>, HttpError> {
        if self.license_item.is_some() {
            return Err((500, "Error CLMR357f").into());
        }
        if self.license_index.is_none() || self.email.is_none() {
            return Err((500, "Error CLMR360q").into());
        }
        let mut license_map = HashMap::new_map(vec![
            ("id1", &self.license_index.as_ref().unwrap()),
            ("id2", "all"),
            ("Email", &self.email.as_ref().unwrap()),
            ("OfflineSecret", &create_permanent_secret()),
            ("uuid", &self.uuid)
        ]);
        license_map.insert_map("Plugins", None);

        return Ok(license_map.to_owned());
    }
}