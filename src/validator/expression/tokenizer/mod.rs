use serde_json::Value;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Value(Value),
    Operator(Operator),
    OpenParenthesis,
    CloseParenthesis,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Math(MathOp),
    Logic(LogicOp),
}

#[derive(Debug, PartialEq, Clone)]
pub enum MathOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulo,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogicOp {
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    String(String), // is this used?
    Reference(String),
    Math(Box<Expression>, MathOp, Box<Expression>),
    Logic(Box<Expression>, LogicOp, Box<Expression>),
}

const OPERATOR_MAPPINGS: &[( &str, Token )] = &[
    // Math
    ("+", Token::Operator(Operator::Math(MathOp::Plus))),
    ("-", Token::Operator(Operator::Math(MathOp::Minus))),
    ("*", Token::Operator(Operator::Math(MathOp::Multiply))),
    ("/", Token::Operator(Operator::Math(MathOp::Divide))),
    ("^", Token::Operator(Operator::Math(MathOp::Power))),
    ("%", Token::Operator(Operator::Math(MathOp::Modulo))),
    // Logic
    ("!", Token::Operator(Operator::Logic(LogicOp::Not))),
    ("&&", Token::Operator(Operator::Logic(LogicOp::And))),
    ("||", Token::Operator(Operator::Logic(LogicOp::Or))),
];

pub fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut value = String::new();
    let mut chars = expression.chars().peekable();

    while let Some(current_char) = chars.next() {
        if current_char.is_whitespace() {
            collect_value(&mut value, &mut tokens);
            continue;
        }

        let frag = collect_fragment(&mut chars, current_char);
        match frag.as_str() {
            "(" => {
                collect_value(&mut value, &mut tokens);
                tokens.push(Token::OpenParenthesis);
            }
            ")" => {
                collect_value(&mut value, &mut tokens);
                tokens.push(Token::CloseParenthesis);
            }
            frag => match OPERATOR_MAPPINGS.iter().find(|&&(o, _)| o == frag) {
                Some((_, token)) => {
                    collect_value(&mut value, &mut tokens);
                    tokens.push(token.clone());
                },
                None => value.push_str(&frag),
            },
        }
    }

    collect_value(&mut value, &mut tokens);
    tokens
}

fn collect_value(value: &mut String, tokens: &mut Vec<Token>) {
    if !value.is_empty() {
        tokens.push(Token::Value(serde_json::json!(value)));
    }
}

fn collect_fragment(chars: &mut Peekable<Chars>, current_char: char) -> String {
    let mut fragment = String::new();
    fragment.push(current_char);

    while let Some(&next_char) = chars.peek() {
        fragment.push(next_char);
        if OPERATOR_MAPPINGS.iter().any(|(o, _)| o.starts_with(&fragment)) {
            chars.next();
        } else {
            fragment.pop();
            break;
        }
    }

    fragment
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn simple() {
        let input = r#"1 + 2"#;
        let result = tokenize(input);
        assert_eq!(
            result,
            vec![
                super::Token::Value("1".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Plus)),
                super::Token::Value("2".to_string()),
            ]
        );
    }

    #[test]
    fn complex() {
        let input = r#"1 + 2 * 3"#;
        let result = tokenize(input);
        assert_eq!(
            result,
            vec![
                super::Token::Value("1".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Plus)),
                super::Token::Value("2".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Multiply)),
                super::Token::Value("3".to_string()),
            ]
        );
    }

    #[test]
    fn parentheses() {
        let input = r#"1 + (2 * 3)"#;
        let result = super::tokenize(input);
        assert_eq!(
            result,
            vec![
                super::Token::Value("1".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Plus)),
                super::Token::OpenParenthesis,
                super::Token::Value("2".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Multiply)),
                super::Token::Value("3".to_string()),
                super::Token::CloseParenthesis,
            ]
        );
    }

    #[test]
    fn complex_parentheses() {
        let input = r#"1 + (2 * (3 + 4))"#;
        let result = super::tokenize(input);
        assert_eq!(
            result,
            vec![
                super::Token::Value("1".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Plus)),
                super::Token::OpenParenthesis,
                super::Token::Value("2".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Multiply)),
                super::Token::OpenParenthesis,
                super::Token::Value("3".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Plus)),
                super::Token::Value("4".to_string()),
                super::Token::CloseParenthesis,
                super::Token::CloseParenthesis,
            ]
        );
    }

    #[test]
    fn invalid_parentheses() {
        let input = r#"x || (y || z"#;
        let result = super::tokenize(input);
        assert_eq!(
            result,
            vec![
                super::Token::Value("x".to_string()),
                super::Token::Operator(Operator::Logic(LogicOp::Or)),
                super::Token::OpenParenthesis,
                super::Token::Value("y".to_string()),
                super::Token::Operator(Operator::Logic(LogicOp::Or)),
                super::Token::Value("z".to_string()),
            ]
        );
    }

    #[test]
    fn whitespace_between_values() {
        let input = r#"2 x"#;
        let result = super::tokenize(input);
        assert_eq!(
            result,
            vec![
                super::Token::Value("2".to_string()),
                super::Token::Value("x".to_string()),
            ]
        );
    }

    #[test]
    fn alphanumeric_value() {
        let input = r#"2x+b"#;
        let result = super::tokenize(input);
        assert_eq!(
            result,
            vec![
                super::Token::Value("2x".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Plus)),
                super::Token::Value("b".to_string()),
            ]
        );
    }

    #[test]
    fn reference_property_value() {
        let input = r#"x.property + 1"#;
        let result = super::tokenize(input);
        assert_eq!(
            result,
            vec![
                super::Token::Value("x.property".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Plus)),
                super::Token::Value("1".to_string()),
            ]
        );
    }

    #[test]
    fn irregular_whitespace() {
        let input = r#"1+ 2 /3/(4/5 * 6* 3)*0.1/2"#;
        let result = super::tokenize(input);
        assert_eq!(
            result,
            vec![
                super::Token::Value("1".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Plus)),
                super::Token::Value("2".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Divide)),
                super::Token::Value("3".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Divide)),
                super::Token::OpenParenthesis,
                super::Token::Value("4".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Divide)),
                super::Token::Value("5".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Multiply)),
                super::Token::Value("6".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Multiply)),
                super::Token::Value("3".to_string()),
                super::Token::CloseParenthesis,
                super::Token::Operator(Operator::Math(MathOp::Multiply)),
                super::Token::Value("0.1".to_string()),
                super::Token::Operator(Operator::Math(MathOp::Divide)),
                super::Token::Value("2".to_string()),
            ]
        );
    }
}