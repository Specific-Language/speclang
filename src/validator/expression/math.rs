use std::collections::{HashSet, HashMap};
use serde_json::Value;

use crate::validator::{ValidationError, expression::{tokenizer::{self, Token, TokenType}, fail}};

use super::reference;

pub fn validate(value: &str, context: &HashMap<String, Value>) -> Result<(), ValidationError> {
    println!("math::validate {}", value);

    let operators: HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
    let tokens = tokenizer::tokenize(value, &operators);

    let mut open_parentheses_count = 0;
    let mut last_token_type = TokenType::None;

    for token in tokens.iter() {
        match token {
            Token::Operator(_) => {
                if matches!(
                    last_token_type,
                    TokenType::None | TokenType::Operator | TokenType::OpenParenthesis
                ) {
                    return fail("Unexpected operator");
                }
                last_token_type = TokenType::Operator;
            },
            Token::Value(value_token) => {
                if matches!(
                    last_token_type,
                    TokenType::Value | TokenType::CloseParenthesis
                ) {
                    return fail("Unexpected value");
                }
                match serde_json::from_str::<serde_json::Value>(value_token) {
                    Ok(_) => {},
                    Err(_) => reference::validate(value_token, context)?,
                }
                last_token_type = TokenType::Value;
            },
            Token::Parenthesis(p) => {
                if p == "(" {
                    open_parentheses_count += 1;
                    last_token_type = TokenType::OpenParenthesis;
                } else if p == ")" {
                    open_parentheses_count -= 1;
                    last_token_type = TokenType::CloseParenthesis;
                }
                if open_parentheses_count < 0 {
                    return fail("Unbalanced parentheses");
                }
            }
        }
    }

    if open_parentheses_count != 0 {
        return fail("Unbalanced parentheses at the end");
    }

    if matches!(last_token_type, TokenType::Operator) {
        return fail("Expression ends with an operator");
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
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Expression ends with an operator".to_string())));
    }

    #[test]
    fn unbalanced_parentheses() {
        let input = r#"1 + (2 * 3"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        let result = super::validate(input, &context);
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Unbalanced parentheses at the end".to_string())));
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