use std::collections::HashMap;
use serde_json::Value;
use crate::validator::ValidationError;

pub fn validate(reference: &str, context: &HashMap<String, Value>) -> Result<(), ValidationError> {
    println!("reference::validate {}", reference);
    match reference {
        "string" 
            | "number"
            | "bool"
            | "list"
            | "map" => Ok(()),
        _ => match context.get(reference) {
            Some(_) => Ok(()),
            None => Err(ValidationError::InvalidReference),
        }
    }
}

