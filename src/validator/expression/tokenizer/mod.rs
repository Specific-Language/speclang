use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Operand(String),
    Operator(String),
    Parenthesis(String),
}

const PARENTHESES: [&str; 2] = ["(", ")"];

pub fn tokenize(expr: &str, operators: &HashSet<&str>) -> Vec<TokenType> {
    let mut tokens = Vec::new();
    let mut operand = String::new();

    let mut chars: Peekable<Chars> = expr.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            if !operand.is_empty() {
                tokens.push(TokenType::Operand(operand.clone()));
                operand.clear();
            }
            continue;
        }
        if c == '.' {
            operand.push('.');
            continue;
        }

        let mut op = String::new();
        op.push(c);

        while let Some(&next_char) = chars.peek() {
            let mut candidate_op = op.clone();
            candidate_op.push(next_char);
            
            if operators.contains(candidate_op.as_str()) {
                op.push(chars.next().unwrap());
            } else {
                break;
            }
        }

        if PARENTHESES.contains(&op.as_str()) {
            if !operand.is_empty() {
                tokens.push(TokenType::Operand(operand.clone()));
                operand.clear();
            }
            tokens.push(TokenType::Parenthesis(op));
            continue;
        }

        if operators.contains(&op.as_str()) {
            if !operand.is_empty() {
                tokens.push(TokenType::Operand(operand.clone()));
                operand.clear();
            }
            tokens.push(TokenType::Operator(op));
        } else {
            operand.push_str(&op);
        }
    }

    if !operand.is_empty() {
        tokens.push(TokenType::Operand(operand));
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
                super::TokenType::Operand("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Operand("2".to_string()),
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
                super::TokenType::Operand("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Operand("2".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Operand("3".to_string()),
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
                super::TokenType::Operand("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Parenthesis("(".to_string()),
                super::TokenType::Operand("2".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Operand("3".to_string()),
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
                super::TokenType::Operand("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Parenthesis("(".to_string()),
                super::TokenType::Operand("2".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Parenthesis("(".to_string()),
                super::TokenType::Operand("3".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Operand("4".to_string()),
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
                super::TokenType::Operand("x".to_string()),
                super::TokenType::Operator("||".to_string()),
                super::TokenType::Parenthesis("(".to_string()),
                super::TokenType::Operand("y".to_string()),
                super::TokenType::Operator("||".to_string()),
                super::TokenType::Operand("z".to_string()),
            ]
        );
    }

    #[test]
    fn whitespace_between_operands() {
        let input = r#"1 + 2 x"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Operand("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Operand("2".to_string()),
                super::TokenType::Operand("x".to_string()),
            ]
        );
    }

    #[test]
    fn alphanumeric_operand() {
        let input = r#"2x+b"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Operand("2x".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Operand("b".to_string()),
            ]
        );
    }

    #[test]
    fn reference_property_operand() {
        let input = r#"x.property + 1"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
        assert_eq!(
            result,
            vec![
                super::TokenType::Operand("x.property".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Operand("1".to_string()),
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
                super::TokenType::Operand("1".to_string()),
                super::TokenType::Operator("+".to_string()),
                super::TokenType::Operand("2".to_string()),
                super::TokenType::Operator("/".to_string()),
                super::TokenType::Operand("3".to_string()),
                super::TokenType::Operator("/".to_string()),
                super::TokenType::Parenthesis("(".to_string()),
                super::TokenType::Operand("4".to_string()),
                super::TokenType::Operator("/".to_string()),
                super::TokenType::Operand("5".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Operand("6".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Operand("3".to_string()),
                super::TokenType::Parenthesis(")".to_string()),
                super::TokenType::Operator("*".to_string()),
                super::TokenType::Operand("0.1".to_string()),
                super::TokenType::Operator("/".to_string()),
                super::TokenType::Operand("2".to_string()),
            ]
        );
    }
}