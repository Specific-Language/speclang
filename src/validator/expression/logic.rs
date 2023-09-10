use std::collections::{HashSet, HashMap};
use serde_json::Value;
use crate::validator::{ValidationError, expression::{tokenizer::{self, Token}, fail}};
use super::reference;

const LOGICAL_AND: &str = "&&";
const LOGICAL_OR: &str = "||";
const LOGICAL_NOT: &str = "!";
const LOGIC_OPERATORS: [&str; 3] = [LOGICAL_AND, LOGICAL_OR, LOGICAL_NOT];

pub fn validate(value: &str, context: &HashMap<String, Value>) -> Result<(), ValidationError> {
    println!("logic::validate {}", value);

    let operators: HashSet<_> = LOGIC_OPERATORS.iter().cloned().collect();
    let tokens = tokenizer::tokenize(value, &operators);

    let mut open_parentheses_count = 0;
    let mut last_token: Option<&Token> = None;

    for token in &tokens {
        if let Token::Operator(op) = token {
            if op == LOGICAL_NOT {
                match last_token {
                    Some(Token::Value(_)) => return fail("Unexpected negation operator after a value"),
                    Some(Token::CloseParenthesis) => return fail("Unexpected negation operator after a close parenthesis"),
                    _ => {}
                }
            } else {
                match last_token {
                    None => return fail("Expression starts with a binary operator"),
                    Some(Token::Operator(_)) => return fail("Operator after another operator"),
                    Some(Token::OpenParenthesis) => return fail("Operator after an open parenthesis"),
                    _ => {}
                }
            }
        }

        if let Token::Value(value_token) = token {
            match last_token {
                Some(Token::Value(_)) => return fail("Value after another value"),
                Some(Token::CloseParenthesis) => return fail("Value after a close parenthesis"),
                _ => {}
            }
            match serde_json::from_str::<serde_json::Value>(value_token) {
                Ok(_) => {},
                Err(_) => reference::validate(value_token, context)?,
            }
        } else if token == &Token::OpenParenthesis {
            open_parentheses_count += 1;
        } else if token == &Token::CloseParenthesis {
            open_parentheses_count -= 1;
            if open_parentheses_count < 0 {
                return fail("Close parenthesis but none were open");
            }
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
