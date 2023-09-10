use std::collections::{HashSet, HashMap};
use serde_json::Value;
use crate::validator::{ValidationError, expression::{tokenizer::{self, TokenType, Token}, fail}};
use super::reference;

pub fn validate(value: &str, context: &HashMap<String, Value>) -> Result<(), ValidationError> {
    println!("logic::validate {}", value);

    let operators: HashSet<_> = ["&&", "||", "!"].iter().cloned().collect();
    let tokens = tokenizer::tokenize(value, &operators);

    let mut open_parentheses_count = 0;
    let mut last_token_type = TokenType::None;

    for token in tokens.iter() {
        match token {
            Token::Operator(op) => {
                if op == "!" && matches!(
                    last_token_type,
                    TokenType::None | TokenType::Operator | TokenType::OpenParenthesis
                ) {
                    // "!" is a unary operator, so it has special rules. 
                    last_token_type = TokenType::Operator;
                    continue;
                } else if matches!(
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
    use serde_json::{Value, json};

    #[test]
    fn intersection() {
        let input = r#"x && y"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn union() {
        let input = r#"x || y"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn negation() {
        let input = r#"!x"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn parentheses() {
        let input = r#"(x || y)"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn invalid_beginning() {
        let input = r#"&& x || y"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn invalid_ending() {
        let input = r#"x && y ||"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), false);
    }
    
    #[test]
    fn invalid_parentheses() {
        let input = r#"x || (y && z"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        context.insert("z".to_owned(), json!(true));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn invalid_parentheses_2() {
        let input = r#"x || y) || z"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        context.insert("z".to_owned(), json!(true));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn complex_expression() {
        let input = r#"x && y || z && !a"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        context.insert("z".to_owned(), json!(true));
        context.insert("a".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn ultra_complex_expression() {
        let input = r#"x && y || (z && !a || (b && c)) || (d && e) || f"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        context.insert("z".to_owned(), json!(true));
        context.insert("a".to_owned(), json!(false));
        context.insert("b".to_owned(), json!(false));
        context.insert("c".to_owned(), json!(false));
        context.insert("d".to_owned(), json!(false));
        context.insert("e".to_owned(), json!(false));
        context.insert("f".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn reference_property() {
        let input = r#"x.property && y"#;
        let mut context: HashMap<String, Value> = HashMap::new();
        context.insert("x".to_owned(), json!({}));
        context.insert("x.property".to_owned(), json!(false));
        context.insert("y".to_owned(), json!(true));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
