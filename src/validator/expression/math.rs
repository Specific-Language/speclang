use std::collections::{HashSet, HashMap};
use serde_json::Value;

use crate::validator::{ValidationError, expression::tokenizer::{self, TokenType}};

use super::reference;

pub fn validate(value: &str, context: &HashMap<String, Value>) -> Result<(), ValidationError> {
    println!("math::validate {}", value);

    let operators: HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
    let tokens = tokenizer::tokenize(value, &operators);

    let mut open_parentheses_count = 0;
    let mut last_part_type: Option<&str> = None;

    for token in tokens.iter() {
        match token {
            TokenType::Operator(_) => {
                if last_part_type.is_none() 
                    || last_part_type == Some("operator") 
                    || last_part_type == Some("(") {
                    println!("Debug: Operator found where it shouldn't be.");
                    return Err(ValidationError::InvalidExpressionSyntax("Unexpected operator".to_string()));
                }
                last_part_type = Some("operator");
            },
            TokenType::Value(op) => {
                if last_part_type == Some("value") 
                    || last_part_type == Some(")") {
                    println!("Debug: Value found where it shouldn't be.");
                    return Err(ValidationError::InvalidExpressionSyntax("Unexpected value".to_string()));
                }
                match serde_json::from_str::<serde_json::Value>(&op) {
                    Ok(_) => {},
                    Err(_) => reference::validate(&op, context)?,
                }
                last_part_type = Some("value");
            },
            TokenType::Parenthesis(p) => {
                if p == "(" {
                    open_parentheses_count += 1;
                    last_part_type = Some("(");
                } else {
                    open_parentheses_count -= 1;
                    last_part_type = Some(")");
                }
                if open_parentheses_count < 0 {
                    println!("Debug: Parentheses are unbalanced.");
                    return Err(ValidationError::InvalidExpressionSyntax("Unbalanced parentheses".to_string()));
                }
            }
        }
    }

    if last_part_type == Some("operator") 
        || open_parentheses_count != 0 {
        println!("Debug: Expression ends with an operator or unbalanced parentheses.");
        return Err(ValidationError::InvalidExpressionSyntax("Invalid ending or unbalanced parentheses".to_string()));
    }

    Ok(())
}

mod tests {
    use std::collections::HashMap;

    use serde_json::{json, Value};

    #[test]
    fn simple() {
        let input = r#"x + 1"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        let result = super::validate(input, &context);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn complex() {
        let input = r#"x + 2 * y"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn parentheses() {
        let input = r#"1 + (2 * 3)"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        let result = super::validate(input, &context);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn invalid_ending() {
        let input = r#"1 + 2 *"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        let result = super::validate(input, &context);
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Invalid ending or unbalanced parentheses".to_string())));
    }

    #[test]
    fn unbalanced_parentheses() {
        let input = r#"1 + (2 * 3"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        let result = super::validate(input, &context);
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Invalid ending or unbalanced parentheses".to_string())));
    }

    #[test]
    fn invalid_operator() {
        let input = r#"1 + 2 ++ 3"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        let result = super::validate(input, &context);
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Unexpected operator".to_string())));
    }

    #[test]
    fn invalid_whitespace_between_values() {
        let input = r#"1 + 2 x"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(6));
        let result = super::validate(input, &context);
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Unexpected value".to_string())));
    }
}