use crate::my_modules::{
    networking::{input::decrypted::*, output::error::HttpError}
};


impl Decrypted {
    /**
     * Validate some of the data. Still need to check signature.
     */
    pub fn validate(self) -> Result<Self, HttpError> {
        let error = "Error: ";
        let mut errors: Vec<&str> = Vec::new();
        // validate company
        if self.store_id.len() < 10 || self.store_id.len() > 15 {
            errors.push("Invalid company");
        }

        // validate uuid
        if self.uuid.len() < 10 {
            errors.push("Invalid uuid");
        }

        if self.plugins.len() < 1 {
            errors.push("Missing plugin info.");
        }

        if errors.len() > 0 {
            return Err((400, format!("{}{}", &error, &errors.join(", "))).into());
        }
        return Ok(self);
    }
}