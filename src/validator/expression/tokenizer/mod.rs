use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

// todo: consolidate tokenvalue and tokentype
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Value(String),
    Operator(String),
    Parenthesis(String),
}

pub enum TokenType {
    Operator,
    Value,
    OpenParenthesis,
    CloseParenthesis,
    None,
}

pub fn tokenize(expression: &str, operators: &HashSet<&str>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut value = String::new();

    let mut chars: Peekable<Chars> = expression.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            if !value.is_empty() {
                tokens.push(Token::Value(value.clone()));
                value.clear();
            }
            continue;
        }
        if c == '.' {
            value.push(c);
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
                tokens.push(Token::Value(value.clone()));
                value.clear();
            }
            tokens.push(Token::Parenthesis(op));
            continue;
        }

        if operators.contains(&op.as_str()) {
            if !value.is_empty() {
                tokens.push(Token::Value(value.clone()));
                value.clear();
            }
            tokens.push(Token::Operator(op));
        } else {
            value.push_str(&op);
        }
    }

    if !value.is_empty() {
        tokens.push(Token::Value(value));
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
                super::Token::Value("1".to_string()),
                super::Token::Operator("+".to_string()),
                super::Token::Value("2".to_string()),
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
                super::Token::Value("1".to_string()),
                super::Token::Operator("+".to_string()),
                super::Token::Value("2".to_string()),
                super::Token::Operator("*".to_string()),
                super::Token::Value("3".to_string()),
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
                super::Token::Value("1".to_string()),
                super::Token::Operator("+".to_string()),
                super::Token::Parenthesis("(".to_string()),
                super::Token::Value("2".to_string()),
                super::Token::Operator("*".to_string()),
                super::Token::Value("3".to_string()),
                super::Token::Parenthesis(")".to_string()),
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
                super::Token::Value("1".to_string()),
                super::Token::Operator("+".to_string()),
                super::Token::Parenthesis("(".to_string()),
                super::Token::Value("2".to_string()),
                super::Token::Operator("*".to_string()),
                super::Token::Parenthesis("(".to_string()),
                super::Token::Value("3".to_string()),
                super::Token::Operator("+".to_string()),
                super::Token::Value("4".to_string()),
                super::Token::Parenthesis(")".to_string()),
                super::Token::Parenthesis(")".to_string()),
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
                super::Token::Value("x".to_string()),
                super::Token::Operator("||".to_string()),
                super::Token::Parenthesis("(".to_string()),
                super::Token::Value("y".to_string()),
                super::Token::Operator("||".to_string()),
                super::Token::Value("z".to_string()),
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
                super::Token::Value("1".to_string()),
                super::Token::Operator("+".to_string()),
                super::Token::Value("2".to_string()),
                super::Token::Value("x".to_string()),
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
                super::Token::Value("2x".to_string()),
                super::Token::Operator("+".to_string()),
                super::Token::Value("b".to_string()),
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
                super::Token::Value("x.property".to_string()),
                super::Token::Operator("+".to_string()),
                super::Token::Value("1".to_string()),
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
                super::Token::Value("1".to_string()),
                super::Token::Operator("+".to_string()),
                super::Token::Value("2".to_string()),
                super::Token::Operator("/".to_string()),
                super::Token::Value("3".to_string()),
                super::Token::Operator("/".to_string()),
                super::Token::Parenthesis("(".to_string()),
                super::Token::Value("4".to_string()),
                super::Token::Operator("/".to_string()),
                super::Token::Value("5".to_string()),
                super::Token::Operator("*".to_string()),
                super::Token::Value("6".to_string()),
                super::Token::Operator("*".to_string()),
                super::Token::Value("3".to_string()),
                super::Token::Parenthesis(")".to_string()),
                super::Token::Operator("*".to_string()),
                super::Token::Value("0.1".to_string()),
                super::Token::Operator("/".to_string()),
                super::Token::Value("2".to_string()),
            ]
        );
    }
}