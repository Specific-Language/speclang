use std::collections::HashMap;
use serde_json::Value;
use crate::validator::ValidationError;

pub fn validate(value: &str, context: &HashMap<String, Value>) -> Result<(), ValidationError> {
    println!("reference::validate {}", value);
    match value {
        "string" | "number" | "bool" | "list" | "map" => Ok(()),
        _ => match context.get(value) {
            Some(_) => Ok(()),
            None => Err(ValidationError::InvalidReference),
        }
    }
}

