use crate::parser::operators::*;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "parser/expression.pest"]
pub struct ExpressionParser;

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::String(s) => s.clone(),
            Literal::Bool(b) => b.to_string(),
            Literal::Number(n) => n.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TemplateItem {
    Interpolated(Computed),
    Literal(String),
}

#[derive(Debug, Clone)]
pub enum Computed {
    Unary(UnaryOp, Box<Computed>),
    Binary(Box<Computed>, BinaryOp, Box<Computed>),
    Literal(Literal),
    Reference(String),
    Group(Box<Computed>),
    Template(Vec<TemplateItem>),
    Interpolated(Box<Computed>),
}

impl Computed {
    pub fn from(input: &str) -> Result<Self, pest::error::Error<Rule>> {
        let mut parsed = ExpressionParser::parse(Rule::TEMPLATE, input)?;
        Ok(Self::from_pair(parsed.next().unwrap()))
    }

    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::TEMPLATE => {
                let items = pair.into_inner().map(|p| TemplateItem::Interpolated(Self::from_pair(p))).collect();
                Computed::Template(items)
            }
            Rule::INTERPOLATED => {
                let inner = pair.into_inner().next().unwrap();
                Computed::Interpolated(Box::new(Self::from_pair(inner)))
            }
            Rule::GROUP => {
                let inner = pair.into_inner().next().unwrap();
                Computed::Group(Box::new(Self::from_pair(inner)))
            }
            Rule::NUMBER => Computed::Literal(Literal::Number(pair.as_str().parse().unwrap())),
            Rule::BOOL => Computed::Literal(Literal::Bool(pair.as_str() == "true")),
            Rule::STRING => Computed::Literal(Literal::String(pair.as_str().to_string())),
            Rule::REFERENCE => {
                match pair.as_str() {
                    "true" => Computed::Literal(Literal::Bool(true)),
                    "false" => Computed::Literal(Literal::Bool(false)),
                    _ => Computed::Reference(pair.as_str().to_string()),
                }
            }
            Rule::LOGIC | Rule::COMPARE | Rule::MATH | Rule::MULTIPLY | Rule::EXPONENT => {
                let mut inner = pair.into_inner();
                let mut left = Self::from_pair(inner.next().unwrap());
                while let Some(op) = inner.next() {
                    let right = Self::from_pair(inner.next().unwrap());
                    left = Computed::Binary(
                        Box::new(left),
                        BinaryOp::from_str(op.as_str()).unwrap(),
                        Box::new(right),
                    );
                }
                left
            }
            Rule::TERM => {
                let mut inner = pair.into_inner();
                let child = inner.next().expect("Expected at least one child");
                if let Rule::UNARY_OP = child.as_rule() {
                    let op = UnaryOp::from_str(child.as_str()).unwrap();
                    let next_child = inner.next().expect("Expected expression after unary op");
                    let expr = Self::from_pair(next_child);
                    Computed::Unary(op, Box::new(expr))
                } else {
                    Self::from_pair(child)
                }
            }
            _ => panic!("Unknown rule: {:?}", pair.as_rule()),
        }
    }

    pub fn eval(&self, context: &crate::parser::Context) -> Result<Literal, &'static str> {
        match self {
            Computed::Literal(val) => Ok(val.clone()),
            Computed::Reference(name) => context.eval(name),
            Computed::Unary(op, expr) => {
                let val = expr.eval(context)?;
                Self::eval_unary_operation(op, val)
            }
            Computed::Binary(left, op, right) => {
                let left_val = left.eval(context)?;
                let right_val = right.eval(context)?;
                Self::eval_binary_operation(op, left_val, right_val)
            }
            Computed::Template(items) => {
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
                Ok(Literal::String(result))
            },
            _ => unreachable!(),
        }
    }

    fn eval_unary_operation(op: &UnaryOp, val: Literal) -> Result<Literal, &'static str> {
        match op {
            UnaryOp::LogicNot => match val {
                Literal::Bool(b) => Ok(Literal::Bool(!b)),
                _ => Err("Invalid type for NOT operation"),
            },
            UnaryOp::MathNegative => match val {
                Literal::Number(n) => Ok(Literal::Number(-n)),
                _ => Err("Invalid type for negative operation"),
            },
        }
    }

    fn eval_binary_operation(
        op: &BinaryOp,
        left_val: Literal,
        right_val: Literal,
    ) -> Result<Literal, &'static str> {
        match op {
            | BinaryOp::MathAdd
            | BinaryOp::MathSubtract
            | BinaryOp::MathMultiply
            | BinaryOp::MathDivide
            | BinaryOp::MathModulus
            | BinaryOp::MathPower => match (left_val, right_val) {
                (Literal::Number(l), Literal::Number(r)) => match op {
                    BinaryOp::MathAdd => Ok(Literal::Number(l + r)),
                    BinaryOp::MathSubtract => Ok(Literal::Number(l - r)),
                    BinaryOp::MathMultiply => Ok(Literal::Number(l * r)),
                    BinaryOp::MathDivide => {
                        if r == 0.0 {
                            Err("Division by zero")
                        } else {
                            Ok(Literal::Number(l / r))
                        }
                    }
                    BinaryOp::MathModulus => Ok(Literal::Number(l % r)),
                    BinaryOp::MathPower => Ok(Literal::Number(l.powf(r))),
                    _ => unreachable!(),
                },
                _ => Err("Invalid operands for mathematical operation"),
            },
            BinaryOp::LogicAnd | BinaryOp::LogicOr => match (left_val, right_val) {
                (Literal::Bool(l), Literal::Bool(r)) => match op {
                    BinaryOp::LogicAnd => Ok(Literal::Bool(l && r)),
                    BinaryOp::LogicOr => Ok(Literal::Bool(l || r)),
                    _ => unreachable!(),
                },
                _ => Err("Invalid operands for logical operation"),
            },
            BinaryOp::LogicComparison(comp_op) => match (left_val, right_val) {
                (Literal::Number(l), Literal::Number(r)) => match comp_op {
                    ComparisonOp::LessThan => Ok(Literal::Bool(l < r)),
                    ComparisonOp::GreaterThan => Ok(Literal::Bool(l > r)),
                    ComparisonOp::LessThanOrEqual => Ok(Literal::Bool(l <= r)),
                    ComparisonOp::GreaterThanOrEqual => Ok(Literal::Bool(l >= r)),
                    ComparisonOp::Equals => Ok(Literal::Bool((l - r).abs() < f64::EPSILON)),
                },
                (Literal::Bool(l), Literal::Bool(r)) => match comp_op {
                    ComparisonOp::Equals => Ok(Literal::Bool(l == r)),
                    _ => Err("Invalid comparison for booleans"),
                },
                _ => Err("Invalid operands for comparison operation"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use crate::parser::context::Context;

    lazy_static! {
        static ref CONTEXT: Context = Context::new();
    }

    #[test]
    fn test_bool() {
        let expression = Computed::from("true").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Bool(true));
    }

    #[test]
    fn test_wrapped() {
        let expression = Computed::from("${true}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Bool(true));
    }

    #[test]
    fn test_addition() {
        let expression = Computed::from("${1 + 2}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Number(3.0));
    }

    #[test]
    fn test_addition_complex() {
        let expression = Computed::from("${1 + 2 + 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Number(6.0));
    }

    #[test]
    fn test_subtraction() {
        let expression = Computed::from("${1 - 2 - 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Number(-4.0));
    }

    #[test]
    fn test_multiplication() {
        let expression = Computed::from("${1 * 2 * 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Number(6.0));
    }

    #[test]
    fn test_division() {
        let expression = Computed::from("${1 / 2}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Number(0.5));
    }

    #[test]
    fn test_modulus() {
        let expression = Computed::from("${5 % 2 % 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Number(1.0));
    }

    #[test]
    fn test_power() {
        let expression = Computed::from("${1 ^ 2 ^ 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Number(1.0));
    }

    #[test]
    fn test_and() {
        let expression = Computed::from("${true && false && true}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Bool(false));
    }

    #[test]
    fn test_or() {
        let expression = Computed::from("${true || false || true}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Bool(true));
    }

    #[test]
    fn test_less_than() {
        let expression = Computed::from("${1 < 2}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Bool(true));
    }

    #[test]
    fn test_greater_than() {
        let expression = Computed::from("${2 > 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Bool(false));
    }

    #[test]
    fn test_less_than_or_equal() {
        let expression = Computed::from("${3 <= 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Bool(true));
    }

    #[test]
    fn test_greater_than_or_equal() {
        let expression = Computed::from("${2 >= 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Bool(false));
    }

    #[test]
    fn test_equals() {
        let expression = Computed::from("${1 == 2}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Bool(false));

        let expression2 = Computed::from("${2 == 2}").unwrap();
        let result2 = expression2.eval(&CONTEXT).unwrap();
        assert_eq!(result2, Literal::Bool(true));
    }

    #[test]
    fn test_not() {
        let expression = Computed::from("${!true}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Bool(false));
    }

    #[test]
    fn test_parentheses() {
        let expression = Computed::from("${(1 + 2) * 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Literal::Number(9.0));
    }
}
