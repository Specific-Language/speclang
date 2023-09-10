use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Value(String),
    Operator(String),
    OpenParenthesis,
    CloseParenthesis,
}

const OPEN_PARENTHESIS: &str = "(";
const CLOSE_PARENTHESIS: &str = ")";

pub fn tokenize(expression: &str, operators: &HashSet<&str>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut value = String::new();
    let mut chars = expression.chars().peekable();

    while let Some(current_char) = chars.next() {
        if current_char.is_whitespace() {
            collect_value(&mut value, &mut tokens);
            continue;
        }

        let fragment = collect_fragment(&mut chars, current_char, operators);
        if fragment == OPEN_PARENTHESIS {
            collect_value(&mut value, &mut tokens);
            tokens.push(Token::OpenParenthesis);
        } else if fragment == CLOSE_PARENTHESIS {
            collect_value(&mut value, &mut tokens);
            tokens.push(Token::CloseParenthesis);
        } else if operators.contains(fragment.as_str()) {
            collect_value(&mut value, &mut tokens);
            tokens.push(Token::Operator(fragment));
        } else {
            value.push_str(&fragment);
        }
    }

    collect_value(&mut value, &mut tokens);
    tokens
}

fn collect_value(value: &mut String, tokens: &mut Vec<Token>) {
    if !value.is_empty() {
        tokens.push(Token::Value(value.clone()));
        value.clear();
    }
}

fn collect_fragment(chars: &mut Peekable<Chars>, current_char: char, operators: &HashSet<&str>) -> String {
    let mut fragment = String::new();
    fragment.push(current_char);

    while let Some(&next_char) = chars.peek() {
        fragment.push(next_char);
        if operators.contains(fragment.as_str()) {
            chars.next();
        } else {
            fragment.pop();
            break;
        }
    }

    fragment
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
                super::Token::OpenParenthesis,
                super::Token::Value("2".to_string()),
                super::Token::Operator("*".to_string()),
                super::Token::Value("3".to_string()),
                super::Token::CloseParenthesis,
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
                super::Token::OpenParenthesis,
                super::Token::Value("2".to_string()),
                super::Token::Operator("*".to_string()),
                super::Token::OpenParenthesis,
                super::Token::Value("3".to_string()),
                super::Token::Operator("+".to_string()),
                super::Token::Value("4".to_string()),
                super::Token::CloseParenthesis,
                super::Token::CloseParenthesis,
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
                super::Token::OpenParenthesis,
                super::Token::Value("y".to_string()),
                super::Token::Operator("||".to_string()),
                super::Token::Value("z".to_string()),
            ]
        );
    }

    #[test]
    fn whitespace_between_values() {
        let input = r#"2 x"#;
        let operators: std::collections::HashSet<_> = ["+", "-", "*", "/", "^", "%"].iter().cloned().collect();
        let result = super::tokenize(input, &operators);
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
                super::Token::OpenParenthesis,
                super::Token::Value("4".to_string()),
                super::Token::Operator("/".to_string()),
                super::Token::Value("5".to_string()),
                super::Token::Operator("*".to_string()),
                super::Token::Value("6".to_string()),
                super::Token::Operator("*".to_string()),
                super::Token::Value("3".to_string()),
                super::Token::CloseParenthesis,
                super::Token::Operator("*".to_string()),
                super::Token::Value("0.1".to_string()),
                super::Token::Operator("/".to_string()),
                super::Token::Value("2".to_string()),
            ]
        );
    }
}