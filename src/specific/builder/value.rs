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

impl From<&hcl::Value> for Specific {
    fn from(value: &hcl::Value) -> Self {
        match value {
            hcl::Value::Array(array_value) => {
                let mut result: Vec<Specific> = Vec::new();
                for item in array_value {
                    result.push(item.into());
                }
                Self::List(result)
            },
            hcl::Value::String(string_value) => {
                // todo : yuck! make this DRY
                let template_expr = hcl::TemplateExpr::from(string_value.as_str());
                let template = hcl::Template::from_expr(&template_expr).expect("Expected a template");
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

impl From<&hcl::template::Element> for Specific {
    fn from(element: &hcl::template::Element) -> Self {
        match element {
            hcl::template::Element::Literal(value) => Self::Literal(value.as_str().into()),
            hcl::template::Element::Interpolation(value) => value.expr.clone().into(),
            hcl::template::Element::Directive(value) => {
                panic!("Unsupported directive: {:?}", value);
            }
        }
    }
}

impl From<hcl::Expression> for Specific {
    fn from(value: hcl::Expression) -> Self {
        match value {
            hcl::Expression::Operation(op) => match *op {
                hcl::expr::Operation::Unary(unary) => Specific::Expression(Box::new(Expression {
                    left: Specific::Never,
                    op: match unary.operator {
                        hcl::expr::UnaryOperator::Not => Operator::Unary(Unary::Not),
                        hcl::expr::UnaryOperator::Neg => Operator::Unary(Unary::Negative),
                    },
                    right: unary.expr.into(),
                })),
                hcl::expr::Operation::Binary(binary) => Specific::Expression(Box::new(Expression {
                    left: binary.lhs_expr.into(),
                    op: match binary.operator {
                        hcl::expr::BinaryOperator::Plus => Operator::Binary(Binary::Add),
                        hcl::expr::BinaryOperator::Minus => Operator::Binary(Binary::Subtract),
                        hcl::expr::BinaryOperator::Mul => Operator::Binary(Binary::Multiply),
                        hcl::expr::BinaryOperator::Div => Operator::Binary(Binary::Divide),
                        hcl::expr::BinaryOperator::Mod => Operator::Binary(Binary::Modulo),
                        hcl::expr::BinaryOperator::Eq => Operator::Binary(Binary::Equal),
                        hcl::expr::BinaryOperator::NotEq => Operator::Binary(Binary::NotEqual),
                        hcl::expr::BinaryOperator::Less => Operator::Binary(Binary::Less),
                        hcl::expr::BinaryOperator::LessEq => Operator::Binary(Binary::LessOrEqual),
                        hcl::expr::BinaryOperator::Greater => Operator::Binary(Binary::Greater),
                        hcl::expr::BinaryOperator::GreaterEq => Operator::Binary(Binary::GreaterOrEqual),
                        hcl::expr::BinaryOperator::And => Operator::Binary(Binary::And),
                        hcl::expr::BinaryOperator::Or => Operator::Binary(Binary::Or),
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
