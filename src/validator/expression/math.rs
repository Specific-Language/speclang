use serde_json::{Value, Map};
use crate::validator::{ValidationError, expression::{tokenizer::{self, Token}, fail}};
use super::reference;

pub fn validate(value: &str, context: &Map<String, Value>) -> Result<(), ValidationError> {
    println!("math::validate {}", value);

    let mut open_parentheses_count = 0;
    let mut last_token: Option<&Token> = None;

    let tokens: Vec<Token> = tokenizer::tokenize(value);
    for token in &tokens {
        match detect_sequence_error(last_token, token, context) {
            Some(error) => return fail(error),
            None => {}
        }
        match token {
            Token::OpenParenthesis => open_parentheses_count += 1,
            Token::CloseParenthesis => {
                open_parentheses_count -= 1;
                if open_parentheses_count < 0 {
                    return fail("Close parenthesis but none were open");
                }
            }
            _ => {}
        }
        last_token = Some(token);
    }

    if open_parentheses_count != 0 {
        return fail("Unbalanced parentheses at the end");
    }

    if matches!(last_token, Some(Token::Operator(_))) {
        return fail("Expression ends with an operator");
    }

    Ok(())
}

fn detect_sequence_error(last_token: Option<&Token>, token: &Token, context: &Map<String, Value>) -> Option<&'static str> {
    match (last_token, token) {
        (None, Token::Operator(_)) 
            => Some("Expression starts with an operator"),
        (Some(Token::Operator(_)), Token::Operator(_)) 
            => Some("Operator after another operator"),
        (Some(Token::OpenParenthesis), Token::Operator(_)) 
            => Some("Operator after an open parenthesis"),
        (Some(Token::Value(_)), Token::Value(_)) 
            => Some("Value after another value"),
        (Some(Token::CloseParenthesis), Token::Value(_)) 
            => Some("Value after a close parenthesis"),
        (_, Token::Value(value_token)) => {
            match serde_json::from_str::<serde_json::Value>(value_token) {
                Ok(_) => None,
                Err(_) => {
                    reference::validate(value_token, context).ok();
                    None
                }
            }
        },
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Map, Value};

    #[test]
    fn simple() {
        let input = r#"x + 1"#;
        let mut context: Map<String, Value> = Map::new();
        context.insert("x".to_owned(), json!(true));
        let result = super::validate(input, &context);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn complex() {
        let input = r#"x + 2 * y"#;
        let mut context: Map<String, Value> = Map::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn parentheses() {
        let input = r#"1 + (2 * 3)"#;
        let mut context: Map<String, Value> = Map::new();
        let result = super::validate(input, &context);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn invalid_ending() {
        let input = r#"1 + 2 *"#;
        let mut context: Map<String, Value> = Map::new();
        let result = super::validate(input, &context);
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Expression ends with an operator".to_string())));
    }

    #[test]
    fn unbalanced_parentheses() {
        let input = r#"1 + (2 * 3"#;
        let mut context: Map<String, Value> = Map::new();
        let result = super::validate(input, &context);
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Unbalanced parentheses at the end".to_string())));
    }

    #[test]
    fn invalid_operator() {
        let input = r#"1 + 2 ++ 3"#;
        let mut context: Map<String, Value> = Map::new();
        let result = super::validate(input, &context);
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Operator after another operator".to_string())));
    }

    #[test]
    fn whitespace_between_values() {
        let input = r#"1 + 2 x"#;
        let mut context: Map<String, Value> = Map::new();
        context.insert("x".to_owned(), json!(6));
        let result = super::validate(input, &context);
        assert_eq!(result, Err(super::ValidationError::InvalidExpressionSyntax("Value after another value".to_string())));
    }
}