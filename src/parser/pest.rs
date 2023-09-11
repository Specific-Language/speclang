use pest::Parser;
use serde_json::{Map, Value};
use crate::parser::specific::Specific;

#[derive(pest_derive::Parser)]
#[grammar = "parser/grammar.pest"]
pub struct LanguageParser;

#[derive(Debug, Clone)]
enum Expr {
    Unary { op: UnaryOp, expr: Box<Expr> },
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Literal(Specific),
    Reference(String),
}

#[derive(Debug, Clone)]
enum UnaryOp {
    LogicNot,
}

#[derive(Debug, Clone)]
enum BinaryOp {
    MathAdd,
    MathSubtract,
    MathMultiply,
    MathDivide,
    MathModulus,
    MathPower,
    LogicAnd,
    LogicOr,
    LogicLessThan,
    LogicGreaterThan,
    LogicLessThanOrEqual,
    LogicGreaterThanOrEqual,
    LogicEquals,
}

impl BinaryOp {
    fn from_str(s: &str) -> Self {
        match s {
            "+" => BinaryOp::MathAdd,
            "-" => BinaryOp::MathSubtract,
            "*" => BinaryOp::MathMultiply,
            "/" => BinaryOp::MathDivide,
            "%" => BinaryOp::MathModulus,
            "^" => BinaryOp::MathPower,
            "&&" => BinaryOp::LogicAnd,
            "||" => BinaryOp::LogicOr,
            "<" => BinaryOp::LogicLessThan,
            ">" => BinaryOp::LogicGreaterThan,
            "<=" => BinaryOp::LogicLessThanOrEqual,
            ">=" => BinaryOp::LogicGreaterThanOrEqual,
            "==" => BinaryOp::LogicEquals,
            _ => panic!("Unexpected binary operator: {}", s),
        }
    }
}

impl UnaryOp {
    fn from_str(s: &str) -> Self {
        match s {
            "!" => UnaryOp::LogicNot,
            _ => panic!("Unexpected unary operator: {}", s),
        }
    }
}

impl Expr {
    fn from(pair: pest::iterators::Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::expression => {
                let mut inner = pair.into_inner();
                let mut expr = Expr::from(inner.next().unwrap());
                while let Some(binary_operation_pair) = inner.next() {
                    let mut binary_inner = binary_operation_pair.into_inner();
                    let op = BinaryOp::from_str(binary_inner.next().unwrap().as_str());
                    let right = Expr::from(binary_inner.next().unwrap());
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    };
                }
                expr
            }
            Rule::number => {
                let num: f64 = pair.as_str().parse().expect("Failed to parse number");
                Expr::Literal(Specific::Number(num))
            }
            Rule::bool => {
                let b: bool = pair.as_str().parse().expect("Failed to parse boolean");
                Expr::Literal(Specific::Bool(b))
            }
            Rule::reference => Expr::Reference(pair.as_str().to_owned()),
            Rule::unary_expr => {
                let mut inner = pair.into_inner();
                let op = UnaryOp::from_str(inner.next().unwrap().as_str());
                let expr = Expr::from(inner.next().unwrap());
                Expr::Unary {
                    op,
                    expr: Box::new(expr),
                }
            }
            Rule::value 
            | Rule::atomic 
            | Rule::group 
                => Expr::from(pair.into_inner().next().unwrap()),
            _ => panic!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }

    pub fn eval(&self, context: &Map<String, Value>) -> Specific {
        match self {
            Expr::Binary { left, op, right } => {
                let left_val = left.eval(context);
                let right_val = right.eval(context);
                Self::binary_operation(op, left_val, right_val)
            },
            Expr::Unary { op, expr } => {
                let val = expr.eval(context);
                match op {
                    UnaryOp::LogicNot => match val {
                        Specific::Bool(b) => Specific::Bool(!b),
                        _ => panic!("Invalid type for NOT operation"),
                    },
                }
            }
            Expr::Literal(val) => val.clone(),
            Expr::Reference(name) => {
                Specific::deserialize(context.get(name).expect("Variable not found in context"))
            },
        }
    }

    fn binary_operation(op: &BinaryOp, left_val: Specific, right_val: Specific) -> Specific {
        match (left_val, right_val) {
            (Specific::Number(l), Specific::Number(r)) => {
                match op {
                    BinaryOp::MathAdd => Specific::Number(l + r),
                    BinaryOp::MathSubtract => Specific::Number(l - r),
                    BinaryOp::MathMultiply => Specific::Number(l * r),
                    BinaryOp::MathDivide => {
                        if r == 0.0 {
                            panic!("Division by zero");
                        }
                        Specific::Number(l / r)
                    },
                    BinaryOp::MathModulus => Specific::Number(l % r),
                    BinaryOp::MathPower => Specific::Number(l.powf(r)),
                    BinaryOp::LogicLessThan => Specific::Bool(l < r),
                    BinaryOp::LogicGreaterThan => Specific::Bool(l > r),
                    BinaryOp::LogicLessThanOrEqual => Specific::Bool(l <= r),
                    BinaryOp::LogicGreaterThanOrEqual => Specific::Bool(l >= r),
                    BinaryOp::LogicEquals => Specific::Bool(l == r),
                    _ => panic!("Invalid mathematical operation for numbers: {:?}", op),
                }
            },

            (Specific::Bool(l), Specific::Bool(r)) => {
                match op {
                    BinaryOp::LogicAnd => Specific::Bool(l && r),
                    BinaryOp::LogicOr => Specific::Bool(l || r),
                    BinaryOp::LogicEquals => Specific::Bool(l == r),
                    _ => panic!("Invalid logical operation for booleans: {:?}", op),
                }
            },

            _ => panic!("Invalid types for binary operation"),
        }
    }
}

pub fn evaluate(input: &str, context: &Map<String, Value>) -> Specific {
    let mut parsed = LanguageParser::parse(Rule::expression, input).expect("Failed to parse expression");
    let expr = Expr::from(parsed.next().unwrap());
    expr.eval(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let expression = "1 + 2 + 3";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Number(6.0));
    }

    #[test]
    fn test_subtraction() {
        let expression = "1 - 2 - 3";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Number(-4.0));
    }

    #[test]
    fn test_multiplication() {
        let expression = "1 * 2 * 3";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Number(6.0));
    }

    #[test]
    fn test_division() {
        let expression = "1 / 2";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Number(0.5));
    }

    #[test]
    fn test_modulus() {
        let expression = "1 % 2 % 3";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Number(1.0));
    }

    #[test]
    fn test_power() {
        let expression = "1 ^ 2 ^ 3";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Number(1.0));
    }

    #[test]
    fn test_and() {
        let expression = "true && false && true";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Bool(false));
    }

    #[test]
    fn test_or() {
        let expression = "true || false || true";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Bool(true));
    }

    #[test]
    fn test_less_than() {
        let expression = "1 < 2";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Bool(true));
    }

    #[test]
    fn test_greater_than() {
        let expression = "2 > 3";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Bool(false));
    }

    #[test]
    fn test_less_than_or_equal() {
        let expression = "3 <= 3";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Bool(true));
    }

    #[test]
    fn test_greater_than_or_equal() {
        let expression = "2 >= 3";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Bool(false));
    }

    #[test]
    fn test_equals() {
        let expression = "1 == 2";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Bool(false));

        let expression2 = "2 == 2";
        let result2 = evaluate(expression2, &context);
        assert_eq!(result2, Specific::Bool(true));
    }

    #[test]
    fn test_not() {
        let expression = "!true";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Bool(false));
    }

    #[test]
    fn test_parentheses() {
        let expression = "(1 + 2) * 3";
        let context = Map::new();
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Number(9.0));
    }

    #[test]
    fn test_variables() {
        let expression = "a + b";
        let mut context = Map::new();
        context.insert("a".to_owned(), Value::from(1.0));
        context.insert("b".to_owned(), Value::from(2.0));
        let result = evaluate(expression, &context);
        assert_eq!(result, Specific::Number(3.0));
    }
}
