use serde_json::{Value, Map};
use crate::validator::{ValidationError, expression::{tokenizer::{self, Token, Operator, LogicOp}, fail}};
use super::reference;

pub fn validate(value: &str, context: &Map<String, Value>) -> Result<(), ValidationError> {
    println!("logic::validate {}", value);

    let mut open_parentheses_count = 0;
    let mut last_token: Option<&Token> = None;

    let tokens = tokenizer::tokenize(value);
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
        // Logic::Not
        (None, Token::Operator(Operator::Logic(LogicOp::Not)))
            => None, 
        (Some(Token::Operator(Operator::Logic(_))), Token::Operator(Operator::Logic(LogicOp::Not))) 
            => None,
        (Some(Token::Value(_)), Token::Operator(Operator::Logic(LogicOp::Not))) 
            => Some("Unexpected Logic::Not operator after a value"),
        (Some(Token::Operator(Operator::Math(_))), Token::Operator(Operator::Logic(LogicOp::Not))) 
            => Some("Unexpected Logic::Not operator after a Math operator"),

        // Other operators
        (None, Token::Operator(_)) 
            => Some("Expression starts with a binary operator"),
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
    use serde_json::{Value, Map, json};

    #[test]
    fn intersection() {
        let input = r#"x && y"#;
        let mut context: Map<String, Value> = Map::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn union() {
        let input = r#"x || y"#;
        let mut context: Map<String, Value> = Map::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn negation() {
        let input = r#"!x"#;
        let mut context: Map<String, Value> = Map::new();
        context.insert("x".to_owned(), json!(true));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn parentheses() {
        let input = r#"(x || y)"#;
        let mut context: Map<String, Value> = Map::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn invalid_beginning() {
        let input = r#"&& x || y"#;
        let mut context: Map<String, Value> = Map::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn invalid_ending() {
        let input = r#"x && y ||"#;
        let mut context: Map<String, Value> = Map::new();
        context.insert("x".to_owned(), json!(true));
        context.insert("y".to_owned(), json!(false));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), false);
    }
    
    #[test]
    fn invalid_parentheses() {
        let input = r#"x || (y && z"#;
        let mut context: Map<String, Value> = Map::new();
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
        let mut context: Map<String, Value> = Map::new();
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
        let mut context: Map<String, Value> = Map::new();
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
        let mut context: Map<String, Value> = Map::new();
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
        let mut context: Map<String, Value> = Map::new();
        context.insert("x".to_owned(), json!({}));
        context.insert("x.property".to_owned(), json!(false));
        context.insert("y".to_owned(), json!(true));
        let result = super::validate(input, &context);
        println!("result {:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
