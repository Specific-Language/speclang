use serde_json::{Value, Map};
use regex::Regex;

use crate::validator::ValidationError;

// pub mod tokenizer;
pub mod primitive;
pub mod reference;
pub mod logic;
pub mod math;

pub type ValidationFn = fn(&str, &Map<String, Value>) -> Result<(), ValidationError>;

pub fn validate(value: &str, context: &Map<String, Value>) -> Result<(), ValidationError> {
    let validation_functions: &[ValidationFn] = &[
        primitive::validate,
        reference::validate,
        logic::validate,
        math::validate
    ];    

    for validate_fn in validation_functions {
        if validate_fn(value, context).is_ok() {
            return Ok(());
        }
    }

    Err(ValidationError::InvalidExpression)
}

pub fn find(input: &str) -> Vec<String> {
    // where does this belong?
    let re = Regex::new(r"\$\{([^\}]+)\}").unwrap();
    re.captures_iter(input)
        .map(|cap| cap[1].to_string())
        .collect()
}

pub fn fail(message: &str) -> Result<(), ValidationError> {
    println!("Debug: {}", message);
    Err(ValidationError::InvalidExpressionSyntax(message.to_string()))
}
