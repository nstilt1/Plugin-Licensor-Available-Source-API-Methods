use crate::my_modules::{
    networking::{
        input::{
            decrypted::*
        },
        output::{
            error::*
        },
    },
    crypto::custom::{encrypt_company_id, decrypt_plugin_id},
    utils::{
        maps::*,
        batch_get_utils::*
    }
};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, KeysAndAttributes, BatchGetItemInput};
use rusoto_core::Region;

impl Decrypted {
    /**
     * Run a batch_get_items request to fill in some blanks
     */
    pub async fn batch_get(&mut self) -> Result<(), HttpError> {

        if self.company_item.is_some() || self.user_item.is_some() || self.plugin_items.is_some() {
            return Err("self.batch_get() was called twice".into());
        }

        let client: DynamoDbClient = DynamoDbClient::new(Region::UsEast1);

        let mut req_items: HashMap<String, KeysAndAttributes> = HashMap::new();

        let encrypted_comp_id = encrypt_company_id(&self.store_id);
        req_items.insert(
            COMPANY_TABLE_NAME.to_owned(),
            KeysAndAttributes {
                consistent_read: Some(true),
                keys: vec![HashMap::new_map(vec![("id", &encrypted_comp_id)])],
                ..Default::default()
            }
        );
        req_items.insert(
            USERS_TABLE_NAME.to_owned(),
            KeysAndAttributes {
                consistent_read: Some(true),
                keys: vec![HashMap::new_map(vec![("company", &self.store_id),
                ("uuid", &self.uuid)])],
                ..Default::default()
            }
        );

        // get ids of each plugin, and machine limits
        let mut plugin_mach_limits: HashMap<String, Option<u32>> = HashMap::new();
        let mut plugin_ids: Vec<HashMap<String, AttributeValue>> = Vec::new();
        for plugin in self.plugins.iter() {
            let plugin_id = plugin.id.to_owned();
            let id_map: HashMap<String, AttributeValue> = HashMap::new_map(
                vec![("id", &plugin_id)]
            );

            if !plugin_ids.contains(&id_map) {
                plugin_ids.push(
                    HashMap::new_map(
                        vec![(
                            "id", 
                            &plugin_id
                        )]
                    )
                );
                plugin_mach_limits.insert(
                    plugin_id.to_owned(),
                    None
                );
            }
        }

        if plugin_ids.len() < 1 {
            return Err(HttpError::new(400, "missing plugin info."));
        }

        req_items.insert(
            PLUGINS_TABLE_NAME.to_owned(),
            KeysAndAttributes {
                consistent_read: Some(true),
                keys: plugin_ids.to_owned(),
                ..Default::default()
            }
        );
        
        let batch_get_out = client.batch_get_item(
            BatchGetItemInput {
                request_items: req_items.to_owned(),
                ..Default::default()
            }
        ).await;

        //return Err("made it to 97".into());
        // error below here

        let batch_get_item_vec_map = batch_get_out.get_item_vec_map();
        if batch_get_item_vec_map.is_err() {
            return Err(batch_get_item_vec_map.unwrap_err());
        }
        let responses = batch_get_item_vec_map.unwrap().to_owned();

        //return Err("Made it to 106".into());
        // error below here

        let company_opt = responses.get(COMPANY_TABLE_NAME).to_owned();
        let user_opt = responses.get(USERS_TABLE_NAME).to_owned();
        let plugins_opt = responses.get(PLUGINS_TABLE_NAME).cloned();

        if plugins_opt.is_none() || company_opt.is_none() {
            return Err((403, "Forbidden").into());
        }
        let comp_vec = company_opt.unwrap();
        let plugins_vec = plugins_opt.unwrap();
        
        if comp_vec.len() == 1 {
            self.company_item = Some(comp_vec[0].to_owned());
        }else{
            return Err((403, "Forbidden").into());
        }

        // error above here
        //return Err("Made it to 122".into());

        if plugins_vec.len() == plugin_ids.len() {
            let mut plugin_map: HashMap<String, HashMap<String, AttributeValue>> = HashMap::new();
            for plugin in plugins_vec {
                let plugin_id_result = plugin.get_data("id", S, "CLNIDBG131");
                if plugin_id_result.as_ref().is_err() {
                    return Err(plugin_id_result.unwrap_err());
                }
                // insert id, plugin
                let plugin_id = plugin_id_result.unwrap();
                plugin_map.insert(
                    decrypt_plugin_id(&plugin_id),
                    plugin.to_owned()
                );
                
            }
            self.plugin_items = Some(plugin_map.to_owned());
        }else{
            return Err((500, "Error CLMR222").into());
        }
        // error above here
        //return Err("Made it to 136".into());
        // user doesn't need to exist yet
        if user_opt.is_some() {
            let user_vec = user_opt.unwrap();
            let len = user_vec.len();
            if len == 1 {
                self.user_item = Some(user_vec[0].to_owned())
            }else if len != 0 {
                return Err((500, "Error CLMR212").into());
            }
        }


        return Ok(());
    }

}