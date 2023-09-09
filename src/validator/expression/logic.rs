use std::collections::{HashSet, HashMap};
use serde_json::Value;
use crate::validator::{ValidationError, expression::tokenizer::{self, TokenType}};
use super::reference;

pub fn validate(value: &str, context: &HashMap<String, Value>) -> Result<(), ValidationError> {
    println!("logic::validate {}", value);

    let operators: HashSet<_> = ["&&", "||", "!"].iter().cloned().collect();
    let tokens = tokenizer::tokenize(value, &operators);

    let mut open_parentheses_count = 0;
    let mut last_part_type: Option<&str> = None;

    for token in tokens.iter() {
        match token {
            TokenType::Operator(op) => {
                if op == "!" && (last_part_type.is_none()
                    || last_part_type == Some("operator")
                    || last_part_type == Some("(")) {
                    // "!" is a unary operator, so it has special rules. 
                    last_part_type = Some("operator");
                    continue;
                } else if last_part_type.is_none() 
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

    if last_part_type == Some("action") || open_parentheses_count != 0 {
        println!("Debug: Expression ends with an operator or unbalanced parentheses.");
        return Err(ValidationError::InvalidExpressionSyntax("Invalid ending or unbalanced parentheses".to_string()));
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
