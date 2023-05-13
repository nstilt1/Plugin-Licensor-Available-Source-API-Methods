use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Plugin {
    pub id: String,
    pub license_type: String,
    // use quantity to stick with the preset amount of licenses for this plugin
    pub quantity: Option<String>,
    // use machine_limit to set a custom machine_limit for this plugin
    pub machine_limit: Option<String>,
    pub subtotal: String,
}