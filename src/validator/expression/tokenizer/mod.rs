use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Value(String),
    Operator(String),
    Parenthesis(String),
}

pub fn tokenize(expression: &str, operators: &HashSet<&str>) -> Vec<TokenType> {
    let mut tokens = Vec::new();
    let mut value = String::new();

    let mut chars: Peekable<Chars> = expression.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            if !value.is_empty() {
                tokens.push(TokenType::Value(value.clone()));
                value.clear();
            }
            continue;
        }
        if c == '.' {
            value.push('.');
            continue;
        }

        let mut op = String::new();
        op.push(c);

        while let Some(&next_char) = chars.peek() {
            let mut candidate = op.clone();
            candidate.push(next_char);
            
            if operators.contains(candidate.as_str()) {
                op.push(chars.next().unwrap());
            } else {
                break;
            }
        }

        if ["(", ")"].contains(&op.as_str()) {
            if !value.is_empty() {
                tokens.push(TokenType::Value(value.clone()));
                value.clear();
            }
            tokens.push(TokenType::Parenthesis(op));
            continue;
        }

        if operators.contains(&op.as_str()) {
            if !value.is_empty() {
                tokens.push(TokenType::Value(value.clone()));
                value.clear();
            }
            tokens.push(TokenType::Operator(op));
        } else {
            value.push_str(&op);
        }
    }

    if !value.is_empty() {
        tokens.push(TokenType::Value(value));
    }

    tokens
}

mod tests {    
    #[test]
    fn simple() {
        let input = r#"1 + 2"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Value("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Value("2".to_string()),
            ]
        );
    }

    #[test]
    fn complex() {
        let input = r#"1 + 2 * 3"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Value("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Value("2".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Value("3".to_string()),
            ]
        );
    }

    #[test]
    fn parentheses() {
        let input = r#"1 + (2 * 3)"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Value("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Parenthesis("(".to_string()),
                super::TokenType::Value("2".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Value("3".to_string()),
                super::TokenType::Parenthesis(")".to_string()),
            ]
        );
    }

    #[test]
    fn complex_parentheses() {
        let input = r#"1 + (2 * (3 + 4))"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Value("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Parenthesis("(".to_string()),
                super::TokenType::Value("2".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Parenthesis("(".to_string()),
                super::TokenType::Value("3".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Value("4".to_string()),
                super::TokenType::Parenthesis(")".to_string()),
                super::TokenType::Parenthesis(")".to_string()),
            ]
        );
    }

    #[test]
    fn invalid_parentheses() {
        let input = r#"x || (y || z"#;
        let operators: std::collections::HashSet<_> = ["||", "&&", "!"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Value("x".to_string()),
                super::TokenType::Operator("||".to_string()),
                super::TokenType::Parenthesis("(".to_string()),
                super::TokenType::Value("y".to_string()),
                super::TokenType::Operator("||".to_string()),
                super::TokenType::Value("z".to_string()),
            ]
        );
    }

    #[test]
    fn whitespace_between_potential_values() {
        let input = r#"1 + 2 x"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Value("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Value("2".to_string()),
                super::TokenType::Value("x".to_string()),
            ]
        );
    }

    #[test]
    fn alphanumeric_value() {
        let input = r#"2x+b"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Value("2x".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Value("b".to_string()),
            ]
        );
    }

    #[test]
    fn reference_property_value() {
        let input = r#"x.property + 1"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Value("x.property".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Value("1".to_string()),
            ]
        );
    }

    #[test]
    fn irregular_whitespace() {
        let input = r#"1+ 2 /3/(4/5 * 6* 3)*0.1/2"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Value("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Value("2".to_string()),
                super::TokenType::Operator("/".to_string()),
                super::TokenType::Value("3".to_string()),
                super::TokenType::Operator("/".to_string()),
                super::TokenType::Parenthesis("(".to_string()),
                super::TokenType::Value("4".to_string()),
                super::TokenType::Operator("/".to_string()),
                super::TokenType::Value("5".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Value("6".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Value("3".to_string()),
                super::TokenType::Parenthesis(")".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Value("0.1".to_string()),
                super::TokenType::Operator("/".to_string()),
                super::TokenType::Value("2".to_string()),
            ]
        );
    }
}