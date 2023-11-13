use hcl::Value;
use hcl::template::Element;
use hcl::expr::Operation;
use hcl::expr::UnaryOperator;
use hcl::expr::BinaryOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum Unary {
    Not,
    Negative,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Binary {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Unary(Unary),
    Binary(Binary),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub left: Specific,
    pub op: Operator,
    pub right: Specific,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Specific {
    Never,
    Literal(hcl::Value),
    Reference(String),
    Expression(Box<Expression>),
    List(Vec<Specific>),
    StringTemplate(Vec<Specific>),
}

// this seems superfluous potentially
impl From<&Vec<hcl::Value>> for Specific {
    fn from(values: &Vec<Value>) -> Self {
        let mut result: Vec<Specific> = Vec::new();
        for value in values {
            result.push(value.into());
        }
        Self::List(result)
    }
}

impl From<&hcl::Value> for Specific {
    fn from(value: &Value) -> Self {
        match value {
            Value::Array(array_value) => {
                let mut result: Vec<Specific> = Vec::new();
                for item in array_value {
                    result.push(item.into());
                }
                Self::List(result)
            },
            Value::String(string_value) => {
                // todo : can we make this DRY
                let template_expr = hcl::TemplateExpr::from(string_value.as_str());
                let template = hcl::Template::from_expr(&template_expr).unwrap();
                let elements = template.elements();
                if elements.len() == 1 {
                    Self::from(&elements[0])
                } else {
                    let mut result: Vec<Specific> = Vec::new();
                    for element in elements {
                        result.push(element.into());
                    }
                    Self::StringTemplate(result)
                }
            },
            _ => Self::Literal(value.clone())
        }
    }
}

// is this rlly necessary?seems like could be handled directly as hcl
impl From<&hcl::template::Element> for Specific {
    fn from(element: &Element) -> Self {
        match element {
            Element::Literal(value) => Self::Literal(value.as_str().into()),
            Element::Interpolation(value) => value.expr.clone().into(),
            Element::Directive(value) => {
                panic!("Unsupported directive: {:?}", value);
            }
        }
    }
}

// is this rlly necessary? seems like 1:1
impl From<hcl::Expression> for Specific {
    fn from(value: hcl::Expression) -> Self {
        match value {
            hcl::Expression::Operation(op) => match *op {
                Operation::Unary(unary) => Specific::Expression(Box::new(Expression {
                    left: Specific::Never,
                    op: match unary.operator {
                        UnaryOperator::Not => Operator::Unary(Unary::Not),
                        UnaryOperator::Neg => Operator::Unary(Unary::Negative),
                    },
                    right: unary.expr.into(),
                })),
                Operation::Binary(binary) => Specific::Expression(Box::new(Expression {
                    left: binary.lhs_expr.into(),
                    op: match binary.operator {
                        BinaryOperator::Plus => Operator::Binary(Binary::Add),
                        BinaryOperator::Minus => Operator::Binary(Binary::Subtract),
                        BinaryOperator::Mul => Operator::Binary(Binary::Multiply),
                        BinaryOperator::Div => Operator::Binary(Binary::Divide),
                        BinaryOperator::Mod => Operator::Binary(Binary::Modulo),
                        BinaryOperator::Eq => Operator::Binary(Binary::Equal),
                        BinaryOperator::NotEq => Operator::Binary(Binary::NotEqual),
                        BinaryOperator::Less => Operator::Binary(Binary::Less),
                        BinaryOperator::LessEq => Operator::Binary(Binary::LessOrEqual),
                        BinaryOperator::Greater => Operator::Binary(Binary::Greater),
                        BinaryOperator::GreaterEq => Operator::Binary(Binary::GreaterOrEqual),
                        BinaryOperator::And => Operator::Binary(Binary::And),
                        BinaryOperator::Or => Operator::Binary(Binary::Or),
                    },
                    right: binary.rhs_expr.into(),
                })),
            }
            hcl::Expression::Variable(ref_value) => Self::Reference(ref_value.to_string()),
            hcl::Expression::Parenthesis(expr) => (*expr).into(),
            hcl::Expression::Number(value) => Self::Literal(value.into()),
            hcl::Expression::Bool(value) => Self::Literal(value.into()),
            hcl::Expression::String(value) => Self::Literal(value.into()),
            _ => panic!("Unsupported expression: {:?}", value),
        }
    }
}
