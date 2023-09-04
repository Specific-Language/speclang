use std::collections::HashMap;
use serde_json::Value;
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

fn identify_expression_type(value: &str, context: &HashMap<String, Value>) -> ExpressionType {
    match primitive::validate(value) {
        Ok(()) => return ExpressionType::Primitive,
        _ => (),
    }
    match reference::validate(value, context) {
        Ok(()) => return ExpressionType::Reference,
        _ => (),
    }
    match logic::validate(value, context) {
        Ok(()) => return ExpressionType::Logic,
        _ => (),
    }
    match math::validate(value, context) {
        Ok(()) => return ExpressionType::Math,
        _ => (),
    }
    ExpressionType::Unknown
}

pub fn validate(value: &str, context: &HashMap<String, Value>) -> Result<(), ValidationError> {
    match identify_expression_type(value, context) {
        ExpressionType::Primitive
            | ExpressionType::Math
            | ExpressionType::Logic
            | ExpressionType::Reference => {
            return Ok(());
        },
        ExpressionType::Unknown => Err(ValidationError::InvalidExpression)
    }
}
