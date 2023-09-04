use std::collections::HashMap;

use regex::Regex;
use serde_json::Value;

pub mod expression;
pub mod structure;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    InvalidInput,
    InvalidType,
    InvalidValue,
    InvalidReference,
    InvalidExpression,
    InvalidExpressionSyntax(String)
}

fn find_expressions(input: &str) -> Vec<String> {
    let re = Regex::new(r"\$\{([^\}]+)\}").unwrap();
    re.captures_iter(input)
        .map(|cap| cap[1].to_string())
        .collect()
}

pub fn validate(parsed: &Value, context: &mut HashMap<String, Value>) -> Result<(), ValidationError> {
    let obj = parsed.as_object().ok_or(ValidationError::InvalidInput)?;
    
    for (key, value) in obj.iter() {
        match value {
            Value::String(string) => {
                let expressions = find_expressions(string);
                if !expressions.is_empty() {
                    println!("\tExpressions: {:?}", expressions);
                    expressions.iter().for_each(|expression| {
                        expression::validate(expression, &context).unwrap();
                    });
                }
                context.insert(key.to_owned(), value.to_owned());
            },
            Value::Number(..) => {},
            Value::Bool(..) => {},
            Value::Array(..) => {},
            Value::Object(..) => {},
            _ => return Err(ValidationError::InvalidType),
        }
    }
    
    Ok(())
}
