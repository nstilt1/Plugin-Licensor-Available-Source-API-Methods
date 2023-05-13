

use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;

use serde::{Serialize, Deserialize};

use super::error::HttpError;


#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    id: String,
    computer_name: String,
    os: String,
}
impl Machine {
    pub fn new(new_id: &str, c_name: &str, new_os: &str) -> Self {
        Machine {
            id: new_id.to_string(),
            computer_name: c_name.to_string(),
            os: new_os.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plugin {
    id: String,
    machines: Vec<Machine>,
    max_machines: String,
    license_type: String,
}
impl Plugin {
    pub fn new(new_id: &str, machine_limit: &str, l_type: &str) -> Self {
        Plugin {
            id: new_id.to_owned(),
            machines: Vec::new(),
            max_machines: machine_limit.to_owned(),
            license_type: l_type.to_owned(),
        }
    }
    pub fn add_machine(&mut self, machine: Machine) {
        self.machines.push(machine);
    }
    pub fn add_machines(&mut self, map: HashMap<String, AttributeValue>) -> Result <(), String> {
        for (mach_id, value) in map {
            let node = value.m.as_ref().unwrap().to_owned();

            let computer_name_opt = node.get("computer_name");
            if computer_name_opt.is_none() {
                return Err("Error GL45e".to_owned());
            }
            let computer_name = computer_name_opt.unwrap().s.as_ref().unwrap().to_owned();
            
            let os_opt = node.get("os");
            if os_opt.is_none() {
                return Err("Error GL51g".to_owned());
            }
            let os = os_opt.unwrap().s.as_ref().unwrap().to_owned();

            self.add_machine(Machine::new(&mach_id, &computer_name, &os));
        }
        return Ok(());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct License {
    code: String,
    plugins: Vec<Plugin>,
}
impl License {
    pub fn new(new_code: &str) -> Self {
        License {
            code: new_code.to_string(),
            plugins: Vec::new(),
        }
    }
    /**
     * Create a filled License Object with 3 lines.
     */
    pub fn init_license(new_code: &str, user_licenses_map: HashMap<String, AttributeValue>) -> Result<License, HttpError> {
        let mut new_license = License::new(new_code);
        for (plugin_id, value) in user_licenses_map {
            let node = value.m.as_ref().unwrap().to_owned();
            let max_machines_opt = node.get("maxMachines");
            if max_machines_opt.is_none() {
                return Err((500, "Error GLU056".to_owned()).into());
            }
            let max_machines = max_machines_opt.unwrap().n.as_ref().unwrap().to_owned();
            
            let license_type_opt = node.get("license_type");
            if license_type_opt.is_none() {
                return Err((500, "Error GLU062".to_owned()).into());
            }
            let license_type = license_type_opt.unwrap().s.as_ref().unwrap().to_string();
            

            let mut plugin = Plugin::new(&plugin_id, &max_machines, &license_type);

            let machine_map_opt = node.get("machines");
            if machine_map_opt.is_none() {
                return Err((500, "Error GLU070".to_owned()).into());
            }
            let machine_map = machine_map_opt.unwrap().m.as_ref().unwrap().to_owned();
            let added_machines = plugin.add_machines(machine_map);
            if added_machines.is_err() {
                return Err((500, added_machines.unwrap_err()).into());
            }
            new_license.add_plugin(plugin);
        }

        return Ok(new_license);
    }
    pub fn add_plugin(&mut self, plugin: Plugin) {
        self.plugins.push(plugin);
    }
}