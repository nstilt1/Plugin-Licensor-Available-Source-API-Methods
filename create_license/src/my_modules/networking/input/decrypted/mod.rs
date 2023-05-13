use rusoto_dynamodb::AttributeValue;
use serde_derive::Deserialize;
use std::collections::HashMap;

use super::plugin::*;

pub static USERS_TABLE_NAME: &str = "PluginUsers";
pub static PLUGINS_TABLE_NAME: &str = "Plugins";
pub static COMPANY_TABLE_NAME: &str = "Companies";
pub static LICENSE_TABLE_NAME: &str = "Licenses";

#[derive(Deserialize, Debug)]
pub struct Decrypted {
    company: String,
    uuid: String,
    plugins: Vec<Plugin>,
    order_id: String,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,

    pub user_item: Option<HashMap<String, AttributeValue>>,
    pub company_item: Option<HashMap<String, AttributeValue>>,
    pub plugin_items: Option<HashMap<String, HashMap<String, AttributeValue>>>,
    pub license_index: Option<String>,
    pub license_item: Option<HashMap<String, AttributeValue>>,
}

pub mod new;
pub mod batch_get;
pub mod validate;
pub mod verify_signature;
pub mod prepare_to_license;
pub mod get_license_item;
pub mod create_license_item;
//pub mod license;
//pub mod get_user_license_maps;
pub mod update_license_data;
pub mod batch_write;