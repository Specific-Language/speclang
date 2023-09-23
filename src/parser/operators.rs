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
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
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

#[derive(Debug, Clone)]
pub enum UnaryOp {
    LogicNot,
}

impl UnaryOp {
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "!" => Ok(UnaryOp::LogicNot),
            _ => Err("Unexpected unary operator"),
        }
    }
}
