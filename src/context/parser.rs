use hcl::eval::Context;
use hcl::Value;
use indexmap::IndexMap;

pub fn parse(input: &str) -> Result<Context, &'static str> {
    let result = hcl::from_str(format!("root {{{}}}", input).as_str());
    let parsed_input: hcl::Value = match result {
        Ok(value) => value,
        Err(error) => panic!("parser::parse error {}", &error.to_string()),
    };
    let map = match parsed_input {
        hcl::Value::Object(map) => map,
        _ => panic!("Unexpected value type in context map")
    };
    Ok(from(map))
}

pub fn from(input: IndexMap<String, Value>) -> Context<'static> {
    let mut context = Context::new();
    for (name, value) in input {
        context.declare_var(name, value);
    }
    context
}

#[cfg(test)]
mod tests {
    use hcl::expr::TemplateExpr;
    use crate::context::evaluator;
    use super::*;

    #[test]
    fn test_123() {
        let input = r#"
            a = 1
            b = 2
            c = a + b
            d = c + 2
        "#;

        let context = parse(input).unwrap();
        let expression = TemplateExpr::from("${d}");
        let result = evaluator::evaluate(&expression, &context);
        assert_eq!(result, hcl::Value::from(5.0));
    }

    #[test]
    fn test_extend() {
        let input = r#"
            x = number
        "#;

        let context = parse(input).unwrap();
        let expression = TemplateExpr::from("${x}");
        let result = evaluator::evaluate(&expression, &context);
        assert_eq!(result, hcl::Value::from(5.0));
    }

    // #[test]
    // fn test_abc() {
    //     // pow has to be built in because hcl2 doesnt support ^ operator
    //     let input = r#"
    //         function {
    //             input = list(unknown)
    //             output = unknown
    //         }

    //         sqrt {
    //             extend function {
    //                 input = [number]
    //                 output extend number {
    //                     value = pow(input, 0.5)
    //                 }
    //             }
    //         }

    //         point { 
    //             x = number
    //             y = number
    //         }

    //         line {
    //             start = point
    //             end = point
    //             length extend number {
    //                 value = sqrt(pow(end.x - start.x, 2) + pow(end.y - start.y, 2))
    //             }
    //         }
    //     "#;
    // // just feed it a context. register event hooks on value changes, etc. within the context. then just feed any and all events into the context. gpt tries to assign them when they fit, and returns a confidence vote. you can decide what to do with that.
    // // register events as they happen. and when a context is hydrated enough, it kicks into action some triggers, and emits some values. EVENT -> EVENT

    // let context = parse(input).unwrap();
    // let expression = TemplateExpr::from("the length is ${line.length}");
    // // let result = expression.evaluate(&context).unwrap();
    // let result = evaluator::evaluate(&expression, &context);
    // // println!("{:?}", result);
    // // let template = Template::from_expr(&expression).unwrap();

    // println!("{:?}", context);
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use hcl::{expr::TemplateExpr, eval::Evaluate};

//     #[test]
//     fn test_top_level_assignment() {
//         let input = r#"
//             a = 1
//             b = 2
//             c = a + b
//         "#;
//         let context = parse_to_context(input).unwrap();
//         let result = context.eval("c").unwrap();
//         assert_eq!(result, Literal::Number(3.0));
//     }

//     #[test]
//     fn test_nested_assignment() {
//         let input = r#"
//             a = 1
//             b = 2
//             c {
//                 d = a + b
//                 e = c.d + b
//             }
//             f = c.d + c.e + a
//         "#;
//         let context = parse_to_context(input).unwrap();
//         let result = context.eval("c.d").unwrap();
//         assert_eq!(result, Literal::Number(3.0));

//         let result2 = context.eval("c.e").unwrap();
//         assert_eq!(result2, Literal::Number(5.0));

//         let result3 = context.eval("f").unwrap();
//         assert_eq!(result3, Literal::Number(9.0));
//     }

//     #[test]
//     fn test_template_interpolation() {
//         let input = r#"
//             world {
//                 name = "World"
//             }
//             c = "Hello ${world.name}!"
//         "#;
//         let context = parse_to_context(input).unwrap();
//         let result = context.eval("c").unwrap();
//         assert_eq!(result, Literal::String("Hello World!".to_string()));
//     }
// }
