use serde_json::{Value, Map};
use crate::validator::ValidationError;

pub fn validate(reference: &str, context: &Map<String, Value>) -> Result<(), ValidationError> {
    println!("reference::validate {}", reference);
    match reference {
        "string" | "number" | "bool" | "list" | "map" => Ok(()),
        _ => match context.get(reference) {
            Some(_) => Ok(()),
            None => Err(ValidationError::InvalidReference),
        }
    }
}
