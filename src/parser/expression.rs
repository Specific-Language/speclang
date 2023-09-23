use crate::parser::operators::*;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "parser/expression.pest"]
pub struct ExpressionParser;

#[derive(Debug, Clone)]
pub enum TemplateItem {
    Interpolated(Expression),
    Literal(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Static {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Static {
    pub fn to_string(&self) -> String {
        match self {
            Static::String(s) => s.clone(),
            Static::Bool(b) => b.to_string(),
            Static::Number(n) => n.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Unary(UnaryOp, Box<Expression>),
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
    Literal(Static),
    Reference(String),
    Group(Box<Expression>),
    Template(Vec<TemplateItem>),
    Interpolated(Box<Expression>),
}

impl Expression {
    pub fn from(input: &str) -> Result<Self, pest::error::Error<Rule>> {
        let mut parsed = ExpressionParser::parse(Rule::TEMPLATE, input)?;
        Ok(Self::from_pair(parsed.next().unwrap()))
    }

    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::TEMPLATE => {
                let items = pair.into_inner().map(|p| TemplateItem::Interpolated(Self::from_pair(p))).collect();
                Expression::Template(items)
            }
            Rule::INTERPOLATED => {
                let inner = pair.into_inner().next().unwrap();
                Expression::Interpolated(Box::new(Self::from_pair(inner)))
            }
            Rule::NUMBER => Expression::Literal(Static::Number(pair.as_str().parse().unwrap())),
            Rule::BOOL => Expression::Literal(Static::Bool(pair.as_str() == "true")),
            Rule::STRING => Expression::Literal(Static::String(pair.as_str().to_string())),
            Rule::LOGIC | Rule::COMPARE | Rule::MATH | Rule::MULTIPLY | Rule::EXPONENT => {
                let mut inner = pair.into_inner();
                let mut left = Self::from_pair(inner.next().unwrap());
                while let Some(op) = inner.next() {
                    let right = Self::from_pair(inner.next().unwrap());
                    left = Expression::Binary(
                        Box::new(left),
                        BinaryOp::from_str(op.as_str()).unwrap(),
                        Box::new(right),
                    );
                }
                left
            }
            Rule::TERM => {
                let inner = pair.into_inner();
                let mut unary_op = None;
                let mut expr = None;
                for child in inner {
                    if let Rule::UNARY_OP = child.as_rule() {
                        unary_op = Some(child.as_str().to_string());
                    } else {
                        expr = Some(Self::from_pair(child));
                    }
                }            
                match unary_op {
                    Some(op) => Expression::Unary(
                        UnaryOp::from_str(&op).unwrap(),
                        Box::new(expr.unwrap())
                    ),
                    None => expr.unwrap(),
                }
            }
            Rule::REFERENCE => {
                match pair.as_str() {
                    "true" => Expression::Literal(Static::Bool(true)),
                    "false" => Expression::Literal(Static::Bool(false)),
                    _ => Expression::Reference(pair.as_str().to_string()),
                }
            }
            Rule::GROUP => {
                let inner = pair.into_inner().next().unwrap();
                Expression::Group(Box::new(Self::from_pair(inner)))
            }
            _ => panic!("Unknown rule: {:?}", pair.as_rule()),
        }
    }

    pub fn eval(&self, context: &crate::parser::Context) -> Result<Static, &'static str> {
        match self {
            Expression::Unary(op, expr) => {
                let val = expr.eval(context)?;
                Self::eval_unary_operation(op, val)
            }
            Expression::Binary(left, op, right) => {
                let left_val = left.eval(context)?;
                let right_val = right.eval(context)?;
                Self::eval_binary_operation(op, left_val, right_val)
            }
            Expression::Literal(val) => Ok(val.clone()),
            Expression::Reference(name) => {
                let expr = context.get(name).ok_or("Variable not found in context")?;
                expr.eval(context)
            }
            Expression::Template(items) => {
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
                Ok(Static::String(result))
            },
            _ => unreachable!(),
        }
    }

    fn eval_unary_operation(op: &UnaryOp, val: Static) -> Result<Static, &'static str> {
        match op {
            UnaryOp::LogicNot => match val {
                Static::Bool(b) => Ok(Static::Bool(!b)),
                _ => Err("Invalid type for NOT operation"),
            },
            UnaryOp::MathNegative => match val {
                Static::Number(n) => Ok(Static::Number(-n)),
                _ => Err("Invalid type for negative operation"),
            },
        }
    }

    fn eval_binary_operation(
        op: &BinaryOp,
        left_val: Static,
        right_val: Static,
    ) -> Result<Static, &'static str> {
        match op {
            | BinaryOp::MathAdd
            | BinaryOp::MathSubtract
            | BinaryOp::MathMultiply
            | BinaryOp::MathDivide
            | BinaryOp::MathModulus
            | BinaryOp::MathPower => match (left_val, right_val) {
                (Static::Number(l), Static::Number(r)) => match op {
                    BinaryOp::MathAdd => Ok(Static::Number(l + r)),
                    BinaryOp::MathSubtract => Ok(Static::Number(l - r)),
                    BinaryOp::MathMultiply => Ok(Static::Number(l * r)),
                    BinaryOp::MathDivide => {
                        if r == 0.0 {
                            Err("Division by zero")
                        } else {
                            Ok(Static::Number(l / r))
                        }
                    }
                    BinaryOp::MathModulus => Ok(Static::Number(l % r)),
                    BinaryOp::MathPower => Ok(Static::Number(l.powf(r))),
                    _ => unreachable!(),
                },
                _ => Err("Invalid operands for mathematical operation"),
            },
            BinaryOp::LogicAnd | BinaryOp::LogicOr => match (left_val, right_val) {
                (Static::Bool(l), Static::Bool(r)) => match op {
                    BinaryOp::LogicAnd => Ok(Static::Bool(l && r)),
                    BinaryOp::LogicOr => Ok(Static::Bool(l || r)),
                    _ => unreachable!(),
                },
                _ => Err("Invalid operands for logical operation"),
            },
            BinaryOp::LogicComparison(comp_op) => match (left_val, right_val) {
                (Static::Number(l), Static::Number(r)) => match comp_op {
                    ComparisonOp::LessThan => Ok(Static::Bool(l < r)),
                    ComparisonOp::GreaterThan => Ok(Static::Bool(l > r)),
                    ComparisonOp::LessThanOrEqual => Ok(Static::Bool(l <= r)),
                    ComparisonOp::GreaterThanOrEqual => Ok(Static::Bool(l >= r)),
                    ComparisonOp::Equals => Ok(Static::Bool((l - r).abs() < f64::EPSILON)),
                },
                (Static::Bool(l), Static::Bool(r)) => match comp_op {
                    ComparisonOp::Equals => Ok(Static::Bool(l == r)),
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
        let expression = Expression::from("true").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Bool(true));
    }

    #[test]
    fn test_wrapped() {
        let expression = Expression::from("${true}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Bool(true));
    }

    #[test]
    fn test_addition() {
        let expression = Expression::from("${1 + 2}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Number(3.0));
    }

    #[test]
    fn test_addition_complex() {
        let expression = Expression::from("${1 + 2 + 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Number(6.0));
    }

    #[test]
    fn test_subtraction() {
        let expression = Expression::from("${1 - 2 - 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Number(-4.0));
    }

    #[test]
    fn test_multiplication() {
        let expression = Expression::from("${1 * 2 * 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Number(6.0));
    }

    #[test]
    fn test_division() {
        let expression = Expression::from("${1 / 2}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Number(0.5));
    }

    #[test]
    fn test_modulus() {
        let expression = Expression::from("${5 % 2 % 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Number(1.0));
    }

    #[test]
    fn test_power() {
        let expression = Expression::from("${1 ^ 2 ^ 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Number(1.0));
    }

    #[test]
    fn test_and() {
        let expression = Expression::from("${true && false && true}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Bool(false));
    }

    #[test]
    fn test_or() {
        let expression = Expression::from("${true || false || true}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Bool(true));
    }

    #[test]
    fn test_less_than() {
        let expression = Expression::from("${1 < 2}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Bool(true));
    }

    #[test]
    fn test_greater_than() {
        let expression = Expression::from("${2 > 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Bool(false));
    }

    #[test]
    fn test_less_than_or_equal() {
        let expression = Expression::from("${3 <= 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Bool(true));
    }

    #[test]
    fn test_greater_than_or_equal() {
        let expression = Expression::from("${2 >= 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Bool(false));
    }

    #[test]
    fn test_equals() {
        let expression = Expression::from("${1 == 2}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Bool(false));

        let expression2 = Expression::from("${2 == 2}").unwrap();
        let result2 = expression2.eval(&CONTEXT).unwrap();
        assert_eq!(result2, Static::Bool(true));
    }

    #[test]
    fn test_not() {
        let expression = Expression::from("${!true}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Bool(false));
    }

    #[test]
    fn test_parentheses() {
        let expression = Expression::from("${(1 + 2) * 3}").unwrap();
        let result = expression.eval(&CONTEXT).unwrap();
        assert_eq!(result, Static::Number(9.0));
    }
}
