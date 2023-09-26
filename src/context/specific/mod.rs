use std::collections::BTreeMap;
use hcl::{
    Value, 
    eval::{Context, Evaluate}, 
    expr::TemplateExpr
};
use self::builder::Builder;

pub mod builder;

pub struct SpecificContext {
    pub tree: BTreeMap<String, Value>
}

impl SpecificContext {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new()
        }
    }

    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn from(input: &Context) -> Self {
        let root_expr = TemplateExpr::from("${context}");
        let result = root_expr.evaluate(input).unwrap();
        if let Value::Object(obj) = result {
            SpecificContext::builder()
                .merge("", &obj)
                .build()
        } else {
            panic!("context root did not evaluate to an object");
        }
    }

    pub fn collect_prefix(&self, prefix: &str) -> Vec<(&String, &Value)> {
        let mut end_bound = prefix.to_string();
        if let Some(last_char) = end_bound.pop() {
            end_bound.push((last_char as u8 + 1) as char);
        } else {
            end_bound.push('\0');
        }
        self.tree.range(prefix.to_string()..end_bound).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::parser;

    #[test]
    fn test_from() {
        let input = r#"
            bird {
                flying = true
                wings {
                    feathered = true
                    count = 2
                }
            }
            duck extends bird {
                quack = true
            }
            wuck extends duck {
                flying = false
            }
            bings extends "bird.wings" {
                count = 4
            }
        "#;
        let context = parser::parse(input).unwrap();
        let specific = SpecificContext::from(&context);
        println!("{:?}", specific.tree);
    }
}