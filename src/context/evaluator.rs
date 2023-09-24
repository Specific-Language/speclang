use hcl::template::Element;

pub fn evaluate(input: &str) { // -> String
    let expression = hcl::expr::TemplateExpr::from(input);
    let template = hcl::template::Template::from_expr(&expression).unwrap();
    let elements = template.elements().to_owned();
    evaluate_elements(elements)
}

fn evaluate_elements(elements: Vec<Element>) {
    for e in elements {
        match e {
            Element::Literal(s) => {
                println!("literal: {:?}", s);
            }
            Element::Interpolation(e) => {
                println!("interpolation:");
                evaluate_interpolation(e);
            }
            Element::Directive(d) => {
                println!("directive: {:?}", d);
            }
        }
    }
}

fn evaluate_interpolation(interp: hcl::template::Interpolation) {
    match interp.expr {
        hcl::Expression::Operation(op) => {
            println!("operation:");
            match *op {
                hcl::Operation::Binary(op2) => {
                    println!("binary operation");
                    match op2 {
                        hcl::BinaryOp { 
                            lhs_expr, 
                            operator, 
                            rhs_expr 
                        } => {
                            println!("lhs: {:?}", lhs_expr);
                            println!("operator: {:?}", operator);
                            println!("rhs: {:?}", rhs_expr);
                        }
                    }
                },
                hcl::Operation::Unary(op2) => {
                    println!("unary operation");
                    match op2 {
                        hcl::UnaryOp { 
                            operator, 
                            expr 
                        } => {
                            println!("operator: {:?}", operator);
                            println!("rhs: {:?}", expr);
                        }
                    }
                },
            }
        }
        _ => println!("interpolation: {:?}", interp),
    }
}

#[cfg(test)]
mod tests {
    use hcl::template::Template;
    use hcl::TemplateExpr;
    use super::*;

    #[test]
    fn test_sqrt() {
        let input = r#"
            z = 4
            a = 5
            b = sqrt(a + z) / 2
        "#;
        let context = crate::context::parser::parse(input).unwrap();
        // complex expressions dont work? need to pre-dissect?
        // let expression = TemplateExpr::from("${b}");
        let expression = TemplateExpr::from("hello ${b + 2}!");
        let template = Template::from_expr(&expression).unwrap();
        evaluate_elements(template.elements().to_owned());

        // let result = expression.evaluate(&context).unwrap();
        // // try to evaluate result as an expression until it can no longer be interpolated further
        // let final_result = fully_evaluate(result, &context);

        // assert_eq!(final_result, hcl::Value::from(9.0_f64.sqrt()/2.0));
    }
}