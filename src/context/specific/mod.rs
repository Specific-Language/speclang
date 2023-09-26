use std::collections::BTreeMap;
use hcl::{
    Value, 
    eval::{Context, Evaluate}, 
    expr::TemplateExpr
};
use self::builder::Builder;

pub mod builder;

pub struct Specific {
    pub tree: BTreeMap<String, Value>
}

impl Specific {
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
            Specific::builder()
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
    use crate::context::hcl;

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
            dird extends wuck {
                wings = bings
            }
        "#;
        let context = hcl::parse(input).unwrap();
        let specific = Specific::from(&context);
        println!("{:?}", specific.tree);
    }

    #[test]
    fn test_weekly() {
        let input = r#"
        weekly_tracker_grid {
            rows = list(weekly_tracker_row)
            rows {
                count = 10
            }
        }
        
        weekly_tracker_row {
            name = string
            days {
                Sunday = boolean
                Monday = boolean
                Tuesday = boolean
                Wednesday = boolean
                Thursday = boolean
                Friday = boolean
                Saturday = boolean
            }
            average = sum(days.each.value ? 1 : 0) / days.length
        }
        "#;
        let context = hcl::parse(input).unwrap();
        let specific = Specific::from(&context);
        println!("{:?}", specific.tree);
    }
}