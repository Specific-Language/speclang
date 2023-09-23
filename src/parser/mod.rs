use crate::parser::context::Context;
use crate::parser::expression::Specific;
use serde_json::Value;

pub mod context;
pub mod expression;
pub mod operators;

pub fn parse_with_context(input: &str, context: &mut Context) -> Result<Specific, &'static str> {
    let result = hcl::from_str(input);
    let parsed_input: Value = match result {
        Ok(value) => value,
        Err(error) => panic!("parser::parse error {}", &error.to_string()),
    };
    context.eval(parsed_input.as_str().unwrap())
}

pub fn parse(input: &str) -> Result<Specific, &'static str> {
    let mut context = Context::new();
    parse_with_context(input, &mut context)
}
