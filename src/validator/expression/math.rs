use std::collections::{HashSet, HashMap};
use serde_json::Value;

use crate::validator::{ValidationError, expression::{tokenizer::{self, Token}, fail}};

use super::reference;

const MATH_PLUS: &str = "+";
const MATH_MINUS: &str = "-";
const MATH_MULTIPLY: &str = "*";
const MATH_DIVIDE: &str = "/";
const MATH_POWER: &str = "^";
const MATH_MODULO: &str = "%";
const MATH_OPERATORS: [&str; 6] = [MATH_PLUS, MATH_MINUS, MATH_MULTIPLY, MATH_DIVIDE, MATH_POWER, MATH_MODULO];

pub fn validate(value: &str, context: &HashMap<String, Value>) -> Result<(), ValidationError> {
    println!("math::validate {}", value);

    let operators: HashSet<_> = MATH_OPERATORS.iter().cloned().collect();
    let tokens = tokenizer::tokenize(value, &operators);

    let mut open_parentheses_count = 0;
    let mut last_token: Option<&Token> = None;

    for token in &tokens {
        match token {
            Token::Operator(_) => {
                match last_token {
                    None => return fail("Expression starts with an operator"),
                    Some(Token::Operator(_)) => return fail("Operator after another operator"),
                    Some(Token::OpenParenthesis) => return fail("Operator after an open parenthesis"),
                    _ => {}
                }
            }
            Token::Value(value) => {
                match last_token {
                    Some(Token::Value(_)) => return fail("Value after another value"),
                    Some(Token::CloseParenthesis) => return fail("Value after a close parenthesis"),
                    _ => {}
                }
                match serde_json::from_str::<serde_json::Value>(value) {
                    Ok(_) => {}
                    Err(_) => reference::validate(value, context)?,
                }
            }
            Token::OpenParenthesis => {
                open_parentheses_count += 1;
            }
            Token::CloseParenthesis => {
                open_parentheses_count -= 1;
                if open_parentheses_count < 0 {
                    return fail("Close parenthesis but none were open");
                }
            }
        }
        last_token = Some(token);
    }    

    if open_parentheses_count != 0 {
        return fail("Unbalanced parentheses at the end");
    }

    if let Some(Token::Operator(_)) = last_token {
        return fail("Expression ends with an operator");
    }

    Ok(())
}

#[cfg(test)]
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
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Operator after another operator".to_string())));
    }

    #[test]
    fn whitespace_between_values() {
        let input = r#"1 + 2 x"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(6));
        let result = super::validate(input, &context);
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Value after another value".to_string())));
    }
}