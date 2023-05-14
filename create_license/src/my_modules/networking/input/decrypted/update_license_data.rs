use std::collections::HashMap;
use rusoto_dynamodb::AttributeValue;
use crate::my_modules::{
    utils::
    {maps::*, 
        utils::Comparing
    }, 
    networking::{
        output::error::HttpError
    }, crypto::custom::decrypt_plugin_id
};

impl super::Decrypted {
    /**
     * Insert license data into user item and license item
     */
    pub async fn update_license_data(&mut self, user_info_res: Result<HashMap<String, AttributeValue>, HttpError>) -> Result<(), HttpError>  {

        if user_info_res.as_ref().is_err() {
            return Err(user_info_res.unwrap_err());
        }

        // get the plugin items for finding the max_machines attribute
        if self.plugin_items.as_ref().is_none() {
            return Err((400, "Error CLMR385g").into());
        }
        let plugins = self.plugin_items.to_owned().unwrap();


        let mut user_info = user_info_res.unwrap();

        // get user item licenses
        let user_item_licenses_res = user_info.get_m("licenses", "Error CLMR347e");
        if user_item_licenses_res.as_ref().is_err() {
            return Err(user_item_licenses_res.unwrap_err());
        }

        // get license item
        let license_item_opt = self.license_item.to_owned();
        if license_item_opt.as_ref().is_none() {
            return Err((500, "Error CLNIDU27b").into());
        }
        let mut license_item = license_item_opt.unwrap();

        let mut user_item_licenses = user_item_licenses_res.unwrap();
        let license_plugins_res = license_item.get_m("Plugins", "Error CLNIDU43");
        if license_plugins_res.as_ref().is_err() {
            return Err(license_plugins_res.unwrap_err());
        }
        let mut license_plugins = license_plugins_res.unwrap();

        for product in self.plugins.iter() {

            // check if a license exists for this plugin for this user
            let product_id = decrypt_plugin_id(&product.id);
            let user_plugin_result = user_item_licenses.get_m(&product_id, "");
            let mut user_license: HashMap<String, AttributeValue>;
            let mut license_plugin: HashMap<String, AttributeValue>;
            let license_plugin_result = license_plugins.get_m(&product_id, "");
            if user_plugin_result.as_ref().is_ok() != license_plugin_result.as_ref().is_ok() {
                return Err((500, "Error CLNIDU57g").into());
            }
            let should_increase: bool;
            // if license for this plugin exists in both license and user
            if user_plugin_result.as_ref().is_ok() && license_plugin_result.as_ref().is_ok() {
                // user already has product
                // need to check if the license type is the same
                user_license = user_plugin_result.unwrap();
                
                license_plugin = license_plugin_result.unwrap();
                let current_license_type_result = license_plugin.get_data("license_type", S);
                if current_license_type_result.is_err() {
                    return Err(current_license_type_result.unwrap_err());
                }
                let current_license_type = current_license_type_result.unwrap().to_lowercase();

                // check for errors
                if product.license_type.eq_ignore_ascii_case("trial") {
                    // if the license exists already and want another trial/beta
                    // then this should throw an error
                    return Err(
                        (   
                            403, 
                            "Forbidden. Previous license found."
                        ).into()
                    );
                    // allow if old_license_type == new_license_type
                }else if !product.license_type.eq_ignore_ascii_case(&current_license_type) {
                    should_increase = false;
                    // don't allow a subscription license if the user already has perpetual
                    if product.license_type.eq_ignore_ascii_case("subscription"){
                        // check whether user already owns the perpetual version
                        if current_license_type.as_str().exists_in(vec!["perpetual", "online", "offline"]) {
                            return Err((500, format!("Error: Cannot switch to subscription from a {} license.", &current_license_type)).into());
                        }
                    }
                }else{
                    should_increase = true;
                }

            }else{
                // license doesn't exist for this plugin yet
                user_license = HashMap::new();
                user_license.insert_map("machines", None);
                license_plugin = HashMap::new();
                should_increase = false;
            }

            // get maxMachines
            let plugin_db_opt = plugins.get(&product_id);
            if plugin_db_opt.is_none() {
                return Err((500, "Error CLMR394v").into());
            }
            let plugin_db = plugin_db_opt.unwrap().to_owned();

            let max_machines_result = plugin_db.get_data("MaxMachinesPerLicense", N);
            if max_machines_result.as_ref().is_err() {
                return Err(max_machines_result.unwrap_err());
            }
            let max_machines_parse_result = max_machines_result.unwrap().parse::<u16>();
            if max_machines_parse_result.as_ref().is_err() {
                return Err((500, "Error CLMR405f").into());
            }
            let max_machines = max_machines_parse_result.unwrap();
            
            let machine_count_purchased: u16;
            if product.machine_limit.as_ref().is_some() {
                let mach_limit_result = product.machine_limit.as_ref().unwrap().parse::<u16>();
                if mach_limit_result.as_ref().is_err() {
                    return Err((500, "Error CLDU133i").into());
                }
                machine_count_purchased = mach_limit_result.unwrap();
            }else{
                if product.quantity.is_none() {
                    return Err((400, "Error: product quantity or machine_limit must be set.").into());
                }
                let quantity_res = product.quantity.as_ref().unwrap().parse::<u16>();
                if quantity_res.is_err() {
                    return Err((500, "Error CLU72").into());
                }
                
                machine_count_purchased = quantity_res.unwrap() * max_machines;
            }

            // check if machine count is over max
            let machines_res = license_plugin.get_m("machines","Error CLNIDU97");
            if machines_res.as_ref().is_err() {
                return Err(machines_res.unwrap_err());
            }
            let machines = machines_res.unwrap();
            let machine_count = machines.into_keys().len();
            if machine_count > machine_count_purchased as usize {
                user_license.insert_map("machines", None);
                license_plugin.insert_l("Online", None);
                license_plugin.insert_l("Offline", None);
            }
            user_license.insert_strings(vec![
                ("license_type", &product.license_type)]
            );
            
            let increase_result = user_license.increase_float("subtotal", &product.subtotal);
            license_plugin = license_plugin.insert_license(
                product.custom_success_message, 
                self.first_name.as_ref().unwrap().as_str(), 
                self.last_name.as_ref().unwrap().as_str(), 
                &product.license_type, 
                machine_count_purchased, 
                &self.order_id,
                should_increase
            );
            if incr.as_ref().is_err() {
                return Err(incr.unwrap_err());
            }
            if mini_license_item.as_ref().is_err() {
                return Err(mini_license_item.unwrap_err());
            }

            user_item_licenses.insert_map(&product_id, Some(user_license));

            license_plugins.insert_map(&product_id, Some(license_plugin.to_owned()));
        
        } // for loop over self.product_info

        license_item.insert_map("Plugins", Some(license_plugins.to_owned()));
        user_info.insert_map("licenses", Some(user_item_licenses.to_owned()));

        self.license_item = Some(license_item.to_owned());
        self.user_item = Some(user_info);
        

        return Ok(());
    }
}