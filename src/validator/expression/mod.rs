use std::collections::HashMap;
use serde_json::Value;
use regex::Regex;

use crate::validator::ValidationError;

pub mod tokenizer;
pub mod primitive;
pub mod reference;
pub mod logic;
pub mod math;

pub enum ExpressionType {
    Primitive,
    Math,
    Logic,
    Reference,
    Unknown
}

pub fn validate(value: &str, context: &HashMap<String, Value>) -> Result<(), ValidationError> {
    let identify_expression = || -> ExpressionType {
        if primitive::validate(value).is_ok() {
            return ExpressionType::Primitive;
        }
        if reference::validate(value, context).is_ok() {
            return ExpressionType::Reference;
        }
        if logic::validate(value, context).is_ok() {
            return ExpressionType::Logic;
        }
        if math::validate(value, context).is_ok() {
            return ExpressionType::Math;
        }
        ExpressionType::Unknown
    };

    match identify_expression() {
        | ExpressionType::Primitive
        | ExpressionType::Math
        | ExpressionType::Logic
        | ExpressionType::Reference
            => Ok(()),
        | ExpressionType::Unknown
            => Err(ValidationError::InvalidExpression),
    }
}

pub fn find(input: &str) -> Vec<String> {
    let re = Regex::new(r"\$\{([^\}]+)\}").unwrap();
    re.captures_iter(input)
        .map(|cap| cap[1].to_string())
        .collect()
}

pub fn fail(message: &str) -> Result<(), ValidationError> {
    println!("Debug: {}", message);
    Err(ValidationError::InvalidExpressionSyntax(message.to_string()))
}
