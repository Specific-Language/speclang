use std::collections::HashMap;

use serde_json::Value;

pub mod expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    InvalidInput,
    InvalidType,
    InvalidValue,
    InvalidReference,
    InvalidExpression,
    InvalidExpressionSyntax(String)
}

pub fn validate(parsed: &Value, context: &mut HashMap<String, Value>) -> Result<(), ValidationError> {
    match parsed {
        Value::String(string) => {
            let expressions = expression::find(string);
            if !expressions.is_empty() {
                println!("\tExpressions: {:?}", expressions);
                expressions.iter().for_each(|expression| {
                    expression::validate(expression, &context).unwrap();
                });
            }
        },
        Value::Number(..) => {},
        Value::Bool(..) => {},
        Value::Array(list) => {
            list.iter().for_each(|value| {
                validate(value, context).unwrap();
            });
        },
        Value::Object(map) => {
            for (key, value) in map.iter() {
                validate(value, context).unwrap();
                context.insert(key.to_string(), value.clone());
            }
        },
        _ => return Err(ValidationError::InvalidType),
    }
    Ok(())
}
