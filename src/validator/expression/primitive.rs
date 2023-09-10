use crate::validator::ValidationError;

pub fn validate(value: &str) -> Result<(), ValidationError> {
    println!("primitive::validate {}", value);
    // will this also match objects and arrays?
    if serde_json::from_str::<serde_json::Value>(value).is_ok() {
        return Ok(());
    }
    Err(ValidationError::InvalidValue)
}
