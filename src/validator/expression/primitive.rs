use crate::validator::ValidationError;

pub fn validate(value: &str) -> Result<(), ValidationError> {
    println!("primitive::validate {}", value);
    // will this also match objects and arrays?
    match serde_json::from_str::<serde_json::Value>(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::InvalidValue),
    }
}

