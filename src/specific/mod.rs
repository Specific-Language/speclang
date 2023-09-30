use std::collections::BTreeMap;
use hcl::Value;
use self::builder::{
    Builder, 
    value::Specific
};

pub mod builder;
pub mod evaluator;

pub struct Context {
    pub tree: BTreeMap<String, Specific>
}

impl Context {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new()
        }
    }

    pub fn builder() -> Builder {
        Builder::new()
    }
    
    pub fn from_str(input: &str) -> Result<Self, String> {
        let parsed_value: Value = hcl::from_str(input)
            .map_err(|err| err.to_string())?;

        let input_map = parsed_value.as_object()
            .ok_or("Expected parsed value to be a Value::Object".to_string())?;
        
        Ok(Context::builder()
            .apply_object("", &input_map)
            .build())
    }    

    pub fn collect_prefix(&self, prefix: &str) -> Vec<(&String, &Specific)> {
        let mut end_bound = prefix.to_string();
        end_bound.push('.');
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
    use crate::specific::builder::value::{Expression, Operator, Binary};

    use super::*;

    #[test]
    fn test_basic() {
        let input = r#"
            a = 1
            b = 2
            c = a + b
        "#;
        let specific = Context::from_str(input).unwrap();
        println!("{:?}", specific.tree);
        assert_eq!(
            specific.tree.get("a").unwrap(),
            &Specific::Literal(Value::Number(1.into()))
        );
        assert_eq!(
            specific.tree.get("b").unwrap(),
            &Specific::Literal(Value::Number(2.into()))
        );
        assert_eq!(
            specific.tree.get("c").unwrap(),
            &Specific::Expression(Box::new(Expression {
                left: Specific::Reference("a".to_string()),
                op: Operator::Binary(Binary::Add),
                right: Specific::Reference("b".to_string()),
            }))
        );
    }
    
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
            cat {
                meow = true
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
            dird {
                name = string
                extends wuck {
                    wings = bings
                }
                extends cat {
                    meow = false
                }
            }
        "#;
        let specific = Context::from_str(input).unwrap();
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
            mantra = string
            days {
                Sunday = boolean
                Monday = boolean
                Tuesday = boolean
                Wednesday = boolean
                Thursday = boolean
                Friday = boolean
                Saturday = boolean
            }
            // each specific is a snapshot in time. expressions should be evaluated as part of construction
            average = days.each[value ? 1 : 0].sum / days.length
        }
        "#;
        let specific = Context::from_str(input).unwrap();
        println!("tree: {:?}", specific.tree);
    }
}