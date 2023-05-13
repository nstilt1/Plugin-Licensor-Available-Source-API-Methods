use crate::my_modules::{
    networking::{input::{
        decrypted::*
    }, output::error::HttpError},
    utils::{
        maps::*
    },
    crypto::{
        custom::encrypt_company_id
    },
    db::{
        create_license_code,
        create_permanent_secret
    }
};

impl Decrypted {
    /**
     * Sets self.license_item and self.license_index, and returns the user item.
     * If the user item doesn't exist, it returns a new user item, just waiting for license info
     */
    pub async fn prepare_to_license(&mut self) -> Result<HashMap<String, AttributeValue>, HttpError> {
        
        let mut user_info: HashMap<String, AttributeValue>;
        if self.user_item.is_some() {
            // verify that the order number isn't already in the orders list
            let get_user_item = self.user_item.as_ref().unwrap();
            let orders_list_opt = get_user_item.get("orders");
            if orders_list_opt.is_some() {
                let orders_list = orders_list_opt.unwrap().l.as_ref().unwrap().to_owned();
                let needle = AttributeValue {
                    s: Some(self.order_id.to_owned()), ..Default::default()
                };
                if orders_list.contains(&needle) {
                    return Err((425, "Error: duplicate order numbers").into());
                }
            }
            user_info = get_user_item.to_owned();
            let license_index_result = user_info.get_data("licenseIndex", S);
            if license_index_result.as_ref().is_err() {
                return Err(license_index_result.unwrap_err());
            }
            self.license_index = Some(license_index_result.unwrap());
            let get_license_result = self.get_license_item().await;
            if get_license_result.as_ref().is_err() {
                return Err(get_license_result.unwrap_err());
            }
            self.license_item = Some(get_license_result.unwrap().to_owned());
        }else{ // get_user_opt was none
            // create a user item but without the updated license maps
            let license_index_result = create_license_code(&self.company).await;
            if license_index_result.as_ref().is_err() {
                return Err((500,license_index_result.unwrap_err().as_str()).into());
            }
            self.license_index = Some(license_index_result.unwrap());

            user_info = HashMap::new_map(
                vec![
                    ("company", &encrypt_company_id(&self.company)),
                    ("uuid", &self.uuid),
                    ("emailHash", &self.email.as_ref().unwrap()),
                    ("licenseIndex", &self.license_index.as_ref().unwrap()),
                    ("OfflineSecret", &create_permanent_secret())
                ]
            );
            user_info.insert_map("licenses", None);

            let license = self.create_license_item();
            if license.as_ref().is_err() {
                return Err(license.unwrap_err().into());
            }
            self.license_item = Some(license.unwrap().to_owned());

        }

        return Ok(user_info.to_owned());
    }
}