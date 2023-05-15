use std::collections::HashMap;
use rusoto_dynamodb::AttributeValue;

use crate::my_modules::networking::output::error::*;

pub static S: char = 's';
pub static N: char = 'n';

pub trait Maps {

    /**
     * Insert a String or a Number into a hashmap, use S or N to pick
     * Returns None if data was not overwritten
     * Returns Some if data was overwritten
     */
    fn insert_data(&mut self, key: &str, data: &str, t: char) -> Option<AttributeValue>;

    /**
     * Inserts license info into a License item hashmap
     */
    fn insert_license(
        &mut self, 
        custom_success: Option<String>, 
        first_name: &str, 
        last_name: &str, 
        license_type: &str, 
        machines_allowed: u16,
        order_number: &str,
        should_increase: bool
    ) -> Result<HashMap<String, AttributeValue>, HttpError>;

    /**
     * Insert a bool into a map
     */
    fn insert_bool(&mut self, key: &str, data: Option<bool>);

    /**
     * Insert a map into a map. Leave empty to add a new map
     */
    fn insert_map(&mut self, key: &str, data:Option<HashMap<String, AttributeValue>>);
    
    fn get_data(&self, key: &str, t: char) -> Result<String, HttpError>;

    /**
     * Get a Map from the hashmap, or return an error
     */
    fn get_m(&self, key: &str, err: &str) -> Result<HashMap<String, AttributeValue>, HttpError>;
    
    /**
     * Returns a hashmap filled with any String Primary keys
     * Supply it with a vec of (PrimaryKeyId, PrimaryKeyValue)
     */
    fn insert_strings(&mut self, keys: Vec<(&str, &str)>) -> Self;

    fn new_map(keys: Vec<(&str, &str)>) -> Self;

    /**
     * Increase a number in a hashmap
     * or insert the number if it doesn't exist
     */
    fn increase_number(&mut self, key: &str, to_add: u16) -> Result<(), HttpError>;

    fn increase_float(&mut self, key: &str, to_add: &str) -> Result<(), HttpError>;

    /**
     * Insert a string into a hashmap, or append it to the existing one, separated by commas
     */
    fn append_string(&mut self, key: &str, to_add: &str) -> Result<(), HttpError>;

    /**
     * Insert a List into a hashmap, or overwrite one.
     * Leave to_add empty for a blank list
     */
    fn insert_l(&mut self, key: &str, to_add: Option<Vec<AttributeValue>>);
}

impl Maps for HashMap<String, AttributeValue> {

    fn insert_l(&mut self, key: &str, to_add: Option<Vec<AttributeValue>>) {
        let to_insert: Vec<AttributeValue>;
        if to_add.is_some(){to_insert = to_add.unwrap();}
        else{to_insert = Vec::new();}

        self.insert(key.to_owned(), AttributeValue { l: Some(to_insert), ..Default::default()});
    }

    fn append_string(&mut self, key: &str, to_add: &str) -> Result<(), HttpError> {
        let existed = self.insert_data(&key, &to_add.to_string(), S);
        if existed.as_ref().is_some(){
            let old_result = existed.as_ref().unwrap().s.as_ref().unwrap();
            self.insert_data(&key, &format!("{},{}", &old_result, &to_add), S);
        }
        return Ok(());
    }

    fn increase_float(&mut self, key: &str, to_add: &str) -> Result<(), HttpError> {
        let existed = self.insert_data(&key, &to_add.to_string(), N);
        if existed.as_ref().is_some(){
            let old_result = existed.unwrap().n.as_ref().unwrap().parse::<f64>();
            if old_result.as_ref().is_err() {
                return Err((500, format!("Error CLMA63dw; key={}", key)).into());
            }
            let float_result = to_add.parse::<f64>();
            if float_result.as_ref().is_err() {
                return Err((500, "Error CLM76b").into());
            }
            
            let old = old_result.unwrap();
            self.insert_data(&key, &(float_result.unwrap()+old).to_string(), N);
        }
        return Ok(());
    }

    fn increase_number(&mut self, key: &str, to_add: u16) -> Result<(), HttpError> {
        let existed = self.insert_data(&key, &to_add.to_string(), N);
        if existed.as_ref().is_some(){
            let old_result = existed.unwrap().n.as_ref().unwrap().parse::<u16>();
            if old_result.as_ref().is_err() {
                return Err((500, format!("Error CLMA63dw; key={}", key)).into());
            }
            let old = old_result.unwrap();
            self.insert_data(&key, &(to_add+old).to_string(), N);
        }
        return Ok(());

    }

    fn new_map(keys: Vec<(&str, &str)>) -> Self {
        return HashMap::new().insert_strings(keys);
    }

    fn insert_license(
            &mut self, 
            custom_success: Option<String>, 
            first_name: &str, 
            last_name: &str, 
            license_type: &str, 
            machines_allowed: u16,
            order_number: &str,
            should_increase: bool
    ) -> Result<HashMap<String, AttributeValue>, HttpError> {
        let cust_success: &str;
        if custom_success.is_none() {
            cust_success = "";
        }else{
            cust_success = custom_success.as_ref().unwrap();
        }
        let mut result =  self.insert_strings(vec![
            ("ActivationTime", "0"),
            ("CustomSuccess", cust_success),
            ("ExpiryTime", "0"),
            ("FirstName", first_name),
            ("LastName", last_name),
            ("LicenseType", license_type),
            ("OrderID", order_number)
        ]).to_owned();

        if should_increase {
            let f = result.increase_number("MachinesAllowed", machines_allowed);
            if f.as_ref().is_err() {return Err(f.unwrap_err());};
        }else{
            result.insert_data("MachinesAllowed", machines_allowed.to_string().as_str(), N);
        }
        result.insert_bool("LicenseActive", Some(true));
        
        result.insert(
            "Offline".to_owned(),
            AttributeValue {
                l: Some(Vec::new()),
                ..Default::default()
            }
        );
        result.insert(
            "Online".to_owned(),
            AttributeValue {
                l: Some(Vec::new()),
                ..Default::default()
            }
        );
        result.insert_bool("SubscriptionActive", Some(true));

        return Ok(result);
        
    }
    fn get_data(&self, key: &str, t: char) -> Result<String, HttpError>{
        let get_opt = self.get(key);
        if get_opt.is_none() {
            return Err((500, "Oopsie whoopsy, there's been an error. CLM34").into());
        }
        let get: Option<&String>;
        if t == 's' {
            get = get_opt.unwrap().s.as_ref();
        }else if t == 'n' {
            get = get_opt.unwrap().n.as_ref();
        }else{
            return Err((500, "Error CLMU160").into());
        }
        if get.as_ref().is_none() {
            return Err((500, "Oopsie whoopsie. Let us know you hit this error, pls. CLM38").into());
        }
        return Ok(get.unwrap().to_owned());
    }
    fn insert_strings(&mut self, keys: Vec<(&str, &str)>) -> Self {
        for key in keys {
            self.insert_data(&key.0, &key.1, S);
        }
        return self.to_owned();
    }
    fn get_m(&self, key: &str, er: &str) -> Result<HashMap<String, AttributeValue>, HttpError> {
        let opt = self.get(key);
        if opt.is_some() {
            let opt_2 = opt.unwrap().m.as_ref();
            if opt_2.is_some() {
                return Ok(opt_2.unwrap().to_owned());
            }
            return Err((500, format!("Error M22, {}", er)).into());
        }
        return Err((500, format!("Error M24, {}", er)).into());
    }
    fn insert_data(&mut self, key: &str, data: &str, t: char) -> Option<AttributeValue> {
        let result: Option<AttributeValue>;
        if t == 's' {
            result = self.insert(
                key.to_owned(),
                AttributeValue {
                    s: Some(data.to_string()),
                    ..Default::default()
                }
            );
        }else if t == 'n' {
            result = self.insert(
                key.to_owned(),
                AttributeValue {
                    n: Some(data.to_string()),
                    ..Default::default()
                }
            );
        }else{
            return None;
        }
        return result;
    } 
    fn insert_bool(&mut self, key: &str, data: Option<bool>) {
        self.insert(
            key.to_owned(),
            AttributeValue {
                bool: data,
                ..Default::default()
            }
        );
    }
    fn insert_map(&mut self, key: &str, data:Option<HashMap<String, AttributeValue>>) {
        if data.as_ref().is_some() {
            self.insert(
                key.to_owned(),
                AttributeValue {
                    m: data.to_owned(),
                    ..Default::default()
                }
            );
        }else{
            self.insert(
                key.to_owned(),
                AttributeValue {
                    m: if data.as_ref().is_some() {
                        data.to_owned()
                    }else{
                        Some(HashMap::new())
                    },
                    ..Default::default()
                }
            );
        }
    }
}

pub trait Maps2ElectricBoogaloo {

}