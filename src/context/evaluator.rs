use std::collections::HashMap;

use hcl::{
    eval::{Context, Evaluate},
    expr::{BinaryOperator, Operation, TemplateExpr, UnaryOperator},
    template::{Element, Template},
    Expression, BinaryOp, UnaryOp, Value, Object, Attribute, edit::expr::Null, Block, Body,
};
use indexmap::IndexMap;

// need to change this to work on a SpecificContext not just a Context

/// Evaluate a HCL template expression within the provided context
pub fn evaluate(expression: &TemplateExpr, context: &Context) -> Value {
    let template = Template::from_expr(expression).unwrap();
    let elements = template.elements().to_owned();
    evaluate_elements(&elements, context)
}

/// Evaluate a list of template elements
fn evaluate_elements(elements: &[Element], context: &Context) -> Value {
    if elements.len() == 1 {
        if let Element::Interpolation(e) = &elements[0] {
            return evaluate_expression(&e.expr, context);
        }
    }
    evaluate_template(elements, context)
}

/// Combine evaluated template elements into a single string value
fn evaluate_template(elements: &[Element], context: &Context) -> Value {
    elements.iter().fold(String::new(), |mut result, element| {
        match element {
            Element::Literal(s) => result.push_str(s),
            Element::Interpolation(e) => {
                let interpolated = evaluate_expression(&e.expr, context);
                let str_value = match interpolated {
                    Value::String(s) => s.to_string(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => panic!("unsupported interpolation type"),
                };
                result.push_str(&str_value);
            }
            Element::Directive(d) => {
                println!("directive: {:?}", d);
                // TODO: Handle this later
            }
        }
        result
    }).into()
}

/// Evaluate a HCL expression to produce a value
fn evaluate_expression(expression: &Expression, context: &Context) -> Value {
    match expression {
        Expression::Operation(op) => match &**op {
            Operation::Binary(binary) => {
                let lhs = evaluate_expression(&binary.lhs_expr, context);
                let rhs = evaluate_expression(&binary.rhs_expr, context);
                apply_binary_operation(binary, lhs, rhs)
            }
            Operation::Unary(unary) => {
                let value = evaluate_expression(&unary.expr, context);
                apply_unary_operation(unary, value)
            }
        },
        Expression::Variable(ref variable) => {
            let result = evaluate_reference(variable, context).unwrap();
            if let Value::String(s) = result {
                let recursive_expr = TemplateExpr::from(s);
                evaluate(&recursive_expr, context)
            } else {
                result
            } 
        },
        Expression::FuncCall(ref func_call) => {
            let func_name = func_call.name.as_str();
            let args = func_call.args.to_owned();
            match func_name {
                "sqrt" => {
                    let arg = args[0].evaluate(context).unwrap();
                    Value::from(arg.as_f64().unwrap().sqrt())
                },
                _ => panic!("Unhandled function call: {:?}", func_call)
            }
        },
        Expression::Number(ref number) => Value::from(number.as_f64().unwrap()),
        Expression::String(ref string) => Value::from(string.as_str()),
        Expression::Bool(ref boolean) => Value::from(boolean.to_owned()),
        _ => panic!("Unhandled expression: {:?}", expression)
    }
}

fn apply_binary_operation(binary: &BinaryOp, lhs: Value, rhs: Value) -> Value {
    match binary.operator {
        | BinaryOperator::Plus 
        | BinaryOperator::Minus 
        | BinaryOperator::Mul 
        | BinaryOperator::Div
        | BinaryOperator::Mod => {
            match (lhs, rhs) {
                (Value::Number(l), Value::Number(r)) => {
                    let l = l.as_f64().unwrap();
                    let r = r.as_f64().unwrap();
                    let result = match binary.operator {
                        BinaryOperator::Plus => l + r,
                        BinaryOperator::Minus => l - r,
                        BinaryOperator::Mul => l * r,
                        BinaryOperator::Div => l / r,
                        BinaryOperator::Mod => l % r,
                        _ => unreachable!(),
                    };
                    Value::from(result)
                }
                _ => panic!("Unsupported operands for arithmetic operation"),
            }
        },
        | BinaryOperator::And 
        | BinaryOperator::Or
        | BinaryOperator::Greater
        | BinaryOperator::GreaterEq
        | BinaryOperator::Less
        | BinaryOperator::LessEq
        | BinaryOperator::Eq
        | BinaryOperator::NotEq => {
            match (lhs, rhs) {
                (Value::Bool(l), Value::Bool(r)) => {
                    let result = match binary.operator {
                        BinaryOperator::And => l && r,
                        BinaryOperator::Or => l || r,
                        BinaryOperator::Greater => l > r,
                        BinaryOperator::GreaterEq => l >= r,
                        BinaryOperator::Less => l < r,
                        BinaryOperator::LessEq => l <= r,
                        BinaryOperator::Eq => l == r,
                        BinaryOperator::NotEq => l != r,
                        _ => unreachable!(),
                    };
                    Value::from(result)
                }
                _ => panic!("Unsupported operands for logical operation"),
            }
        },
    }
}


fn apply_unary_operation(unary: &UnaryOp, value: Value) -> Value {
    match unary.operator {
        UnaryOperator::Neg => Value::from(-value.as_f64().unwrap()),
        UnaryOperator::Not => Value::from(!value.as_bool().unwrap()),
    }
}

fn evaluate_reference(name: &str, context: &Context) -> Option<Value> {
    match name {
        "number" => {
            let number_value = [
                Expression::String("${value >= minimum}".to_string()),
                Expression::String("${value <= maximum}".to_string())
            ];
            let number_prototype = hcl::Body::builder()
                .add_attribute(Attribute::from(("value", Expression::Array(number_value.to_vec()))))
                .add_attribute(Attribute::from(("minimum", "${number}")))
                .add_attribute(Attribute::from(("maximum", "${number}")))
                .build();
            Some(Value::from(number_prototype))
        }
        _ => {
            let expression = TemplateExpr::from(format!("${{{}}}", name));
            expression.evaluate(context).ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() {
        let input = r#"
            a = 5
            z = 4
            b = sqrt(a + z) / 2
        "#;
        let context = crate::context::hcl::parse(input).unwrap();
        let expression = TemplateExpr::from("magic number is ${b + 2}!");
        let result = evaluate(&expression, &context);
        assert_eq!(result.as_str().unwrap(), "magic number is 3.5!");
    }

    #[test]
    fn test_modulo() {
        let input = r#"
            a = 5
            z = 4
            b = a % z
        "#;
        let context = crate::context::hcl::parse(input).unwrap();
        let expression = TemplateExpr::from("${b}");
        let result = evaluate(&expression, &context);
        assert_eq!(result.as_f64().unwrap(), 1.0);
    }
}