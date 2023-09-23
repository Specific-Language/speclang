use std::collections::BTreeMap;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "parser/grammar.pest"]
pub struct LanguageParser;

#[derive(Debug, Clone)]
pub enum UnaryOp {
    LogicNot,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    MathAdd,
    MathSubtract,
    MathMultiply,
    MathDivide,
    MathModulus,
    MathPower,
    LogicAnd,
    LogicOr,
    LogicComparison(ComparisonOp),
}

#[derive(Debug, Clone)]
pub enum ComparisonOp {
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equals,
}

impl BinaryOp {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "+" => Ok(BinaryOp::MathAdd),
            "-" => Ok(BinaryOp::MathSubtract),
            "*" => Ok(BinaryOp::MathMultiply),
            "/" => Ok(BinaryOp::MathDivide),
            "%" => Ok(BinaryOp::MathModulus),
            "^" => Ok(BinaryOp::MathPower),
            "&&" => Ok(BinaryOp::LogicAnd),
            "||" => Ok(BinaryOp::LogicOr),
            "<" => Ok(BinaryOp::LogicComparison(ComparisonOp::LessThan)),
            ">" => Ok(BinaryOp::LogicComparison(ComparisonOp::GreaterThan)),
            "<=" => Ok(BinaryOp::LogicComparison(ComparisonOp::LessThanOrEqual)),
            ">=" => Ok(BinaryOp::LogicComparison(ComparisonOp::GreaterThanOrEqual)),
            "==" => Ok(BinaryOp::LogicComparison(ComparisonOp::Equals)),
            _ => Err("Unexpected binary operator"),
        }
    }
}

impl UnaryOp {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "!" => Ok(UnaryOp::LogicNot),
            _ => Err("Unexpected unary operator"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TemplateItem {
    Interpolated(Expr),
    Literal(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Specific {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Specific {
    fn to_string(&self) -> String {
        match self {
            Specific::String(s) => s.clone(),
            Specific::Bool(b) => b.to_string(),
            Specific::Number(n) => n.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Literal(Specific),
    Reference(String),
    Group(Box<Expr>),
    Template(Vec<TemplateItem>),
    Interpolated(Box<Expr>),
}

impl Expr {
    pub fn from(input: &str) -> Result<Self, pest::error::Error<Rule>> {
        let mut parsed = LanguageParser::parse(Rule::TEMPLATE, input)?;
        let expr = Self::from_pair(parsed.next().unwrap());
        Ok(expr)
    }

    fn from_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::TEMPLATE => {
                println!("TEMPLATE: {:?}", pair.as_str());
                let items = pair.into_inner().map(|p| TemplateItem::Interpolated(Self::from_pair(p))).collect();
                Expr::Template(items)
            },
            Rule::INTERPOLATED => {
                println!("INTERPOLATED: {:?}", pair.as_str());
                let inner = pair.into_inner().next().unwrap();
                Expr::Interpolated(Box::new(Self::from_pair(inner)))
            },
            Rule::NUMBER => {
                println!("NUMBER: {:?}", pair.as_str());
                let specific = Specific::Number(pair.as_str().parse::<f64>().unwrap());
                Expr::Literal(specific)
            },
            Rule::BOOL => {
                println!("BOOL: {:?}", pair.as_str());
                let specific = Specific::Bool(pair.as_str() == "true");
                Expr::Literal(specific)
            },
            Rule::STRING => {
                println!("STRING: {:?}", pair.as_str());
                let specific = Specific::String(pair.as_str().to_string());
                Expr::Literal(specific)
            },
            Rule::LOGIC | Rule::COMPARE | Rule::MATH | Rule::MULTIPLY | Rule::EXPONENT => {
                let mut inner = pair.into_inner();
                let mut left = Self::from_pair(inner.next().unwrap());
                while let Some(op) = inner.next() {
                    let right = Self::from_pair(inner.next().unwrap());
                    left = Expr::Binary(Box::new(left), BinaryOp::from_str(op.as_str()).unwrap(), Box::new(right));
                }
                left
            },
            Rule::TERM => {
                println!("TERM: {:?}", pair.as_str());
                let inner = pair.into_inner();
                let mut unary_op = None;
                let mut expr = None;
                for child in inner {
                    match child.as_rule() {
                        Rule::UNARY_OP => unary_op = Some(child.as_str().to_string()),
                        _ => expr = Some(Self::from_pair(child))
                    }
                }
                match unary_op {
                    Some(op) => Expr::Unary(UnaryOp::from_str(&op).unwrap(), Box::new(expr.unwrap())),
                    None => expr.unwrap()
                }
            },
            Rule::REFERENCE => {
                println!("REFERENCE: {:?}", pair.as_str());
                match pair.as_str() {
                    "true" => Expr::Literal(Specific::Bool(true)),
                    "false" => Expr::Literal(Specific::Bool(false)),
                    _ => Expr::Reference(pair.as_str().to_string()),
                }
            },
            Rule::GROUP => {
                println!("GROUP: {:?}", pair.as_str());
                let inner = pair.into_inner().next().unwrap();
                Expr::Group(Box::new(Self::from_pair(inner)))
            },
            _ => panic!("Unknown rule: {:?}", pair.as_rule()),
        }
    }    

    fn eval(&self, context: &crate::parser::parse::Context) -> Result<Specific, &'static str> {
        match self {
            Expr::Unary(op, expr) => {
                let val = expr.eval(context)?;
                Self::eval_unary_operation(op, val)
            }
            Expr::Binary(left, op, right) => {
                let left_val = left.eval(context)?;
                let right_val = right.eval(context)?;
                Self::eval_binary_operation(op, left_val, right_val)
            },
            Expr::Literal(val) => Ok(val.clone()),
            Expr::Reference(name) => {
                let expr = context.get(name).ok_or("Variable not found in context")?;
                expr.eval(context)
            },
            Expr::Template(items) => {
                if items.len() == 1 {
                    if let TemplateItem::Interpolated(expr) = &items[0] {
                        return expr.eval(context);
                    }
                }
                let mut result = String::new();
                for item in items {
                    match item {
                        TemplateItem::Literal(s) => result.push_str(&s.to_string()),
                        TemplateItem::Interpolated(expr) => {
                            let val = expr.eval(context)?;
                            result.push_str(&val.to_string());
                        }
                    }
                }
                Ok(Specific::String(result))
            },
            Expr::Group(expr) => expr.eval(context),
            Expr::Interpolated(expr) => expr.eval(context),
        }
    }

    fn eval_unary_operation(op: &UnaryOp, val: Specific) -> Result<Specific, &'static str> {
        match op {
            UnaryOp::LogicNot => match val {
                Specific::Bool(b) => Ok(Specific::Bool(!b)),
                _ => Err("Invalid type for NOT operation"),
            },
        }
    }

    fn eval_binary_operation(op: &BinaryOp, left_val: Specific, right_val: Specific) -> Result<Specific, &'static str> {
        match op {
            BinaryOp::MathAdd | BinaryOp::MathSubtract | BinaryOp::MathMultiply |
            BinaryOp::MathDivide | BinaryOp::MathModulus | BinaryOp::MathPower => {
                match (left_val, right_val) {
                    (Specific::Number(l), Specific::Number(r)) => {
                        match op {
                            BinaryOp::MathAdd => Ok(Specific::Number(l + r)),
                            BinaryOp::MathSubtract => Ok(Specific::Number(l - r)),
                            BinaryOp::MathMultiply => Ok(Specific::Number(l * r)),
                            BinaryOp::MathDivide => {
                                if r == 0.0 {
                                    Err("Division by zero")
                                } else {
                                    Ok(Specific::Number(l / r))
                                }
                            },
                            BinaryOp::MathModulus => Ok(Specific::Number(l % r)),
                            BinaryOp::MathPower => Ok(Specific::Number(l.powf(r))),
                            _ => unreachable!(),
                        }
                    },
                    _ => Err("Invalid operands for mathematical operation"),
                }
            },
            BinaryOp::LogicAnd | BinaryOp::LogicOr => {
                match (left_val, right_val) {
                    (Specific::Bool(l), Specific::Bool(r)) => {
                        match op {
                            BinaryOp::LogicAnd => Ok(Specific::Bool(l && r)),
                            BinaryOp::LogicOr => Ok(Specific::Bool(l || r)),
                            _ => unreachable!(),
                        }
                    },
                    _ => Err("Invalid operands for logical operation"),
                }
            },
            BinaryOp::LogicComparison(comp_op) => {
                match (left_val, right_val) {
                    (Specific::Number(l), Specific::Number(r)) => {
                        match comp_op {
                            ComparisonOp::LessThan => Ok(Specific::Bool(l < r)),
                            ComparisonOp::GreaterThan => Ok(Specific::Bool(l > r)),
                            ComparisonOp::LessThanOrEqual => Ok(Specific::Bool(l <= r)),
                            ComparisonOp::GreaterThanOrEqual => Ok(Specific::Bool(l >= r)),
                            ComparisonOp::Equals => Ok(Specific::Bool((l - r).abs() < f64::EPSILON)),
                        }
                    },
                    (Specific::Bool(l), Specific::Bool(r)) => {
                        match comp_op {
                            ComparisonOp::Equals => Ok(Specific::Bool(l == r)),
                            _ => Err("Invalid comparison for booleans"),
                        }
                    },
                    _ => Err("Invalid operands for comparison operation"),
                }
            },
        }
    }    
}

pub struct Context {
    tree: BTreeMap<String, Expr>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            tree: BTreeMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Expr> {
        self.tree.get(key)
    }

    pub fn insert(&mut self, key: String, value: &str) -> Result<(), &'static str> {
        let expr = Expr::from(value).unwrap();
        self.tree.insert(key, expr);
        Ok(())
    }

    pub fn eval(&self, value: &str) -> Result<Specific, &'static str> {
        let expr = Expr::from(value).unwrap();
        let result = expr.eval(&self);
        println!("Evaluated {} to {:?}", value, result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        let expression = "true";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Bool(true));
    }

    #[test]
    fn test_wrapped() {
        let expression = "${true}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Bool(true));
    }

    #[test]
    fn test_addition() {
        let expression = "${1 + 2}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Number(3.0));
    }

    #[test]
    fn test_addition_complex() {
        let expression = "${1 + 2 + 3}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Number(6.0));
    }

    #[test]
    fn test_subtraction() {
        let expression = "${1 - 2 - 3}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Number(-4.0));
    }

    #[test]
    fn test_multiplication() {
        let expression = "${1 * 2 * 3}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Number(6.0));
    }

    #[test]
    fn test_division() {
        let expression = "${1 / 2}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Number(0.5));
    }

    #[test]
    fn test_modulus() {
        let expression = "${5 % 2 % 3}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Number(1.0));
    }

    #[test]
    fn test_power() {
        let expression = "${1 ^ 2 ^ 3}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Number(1.0));
    }

    #[test]
    fn test_and() {
        let expression = "${true && false && true}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Bool(false));
    }

    #[test]
    fn test_or() {
        let expression = "${true || false || true}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Bool(true));
    }

    #[test]
    fn test_less_than() {
        let expression = "${1 < 2}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Bool(true));
    }

    #[test]
    fn test_greater_than() {
        let expression = "${2 > 3}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Bool(false));
    }

    #[test]
    fn test_less_than_or_equal() {
        let expression = "${3 <= 3}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Bool(true));
    }

    #[test]
    fn test_greater_than_or_equal() {
        let expression = "${2 >= 3}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Bool(false));
    }

    #[test]
    fn test_equals() {
        let expression = "${1 == 2}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Bool(false));

        let expression2 = "${2 == 2}";
        let result2 = context.eval(expression2).unwrap();
        assert_eq!(result2, Specific::Bool(true));
    }

    #[test]
    fn test_not() {
        let expression = "${!true}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Bool(false));
    }

    #[test]
    fn test_parentheses() {
        let expression = "${(1 + 2) * 3}";
        let context = Context::new();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Number(9.0));
    }

    #[test]
    fn test_variables() {
        let expression = "${a + b}";
        let mut context = Context::new();
        context.insert("a".to_owned(), "1.0").unwrap();
        context.insert("b".to_owned(), "2.0").unwrap();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Number(3.0));
    }
}
