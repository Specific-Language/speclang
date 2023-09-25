use std::collections::BTreeMap;
use hcl::{
    Value, 
    eval::{Context, Evaluate}, 
    expr::TemplateExpr
};
use indexmap::IndexMap;

pub struct SpecificContext {
    tree: BTreeMap<String, Value>
}

impl SpecificContext {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new()
        }
    }
    pub fn from(input: &Context) -> Self {
        let root_expr = TemplateExpr::from("${root}");
        let result = root_expr.evaluate(input).unwrap();
        if let Value::Object(obj) = result {
            let mut context = Self::new();
            Self::merge(&mut context, &obj, "".to_string());
            context
        } else {
            panic!("root expression did not evaluate to an object");
        }
    }
    pub fn merge(&mut self, obj: &IndexMap<String, Value>, prefix: String) {
        let mut to_merge: Vec<(String, Value)> = Vec::new();
        for (key, value) in obj.iter() {
            let new_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };
            match value {
                Value::Object(value_obj) => {
                    if key == "extend" {
                        let mut reference_to_merge: Vec<(String, Value)> = Vec::new(); // Temporary vector for reference properties
                        let mut overrides_to_merge: Vec<(String, Value)> = Vec::new(); // Temporary vector for overridden values
                        
                        for (extend_key, extend_value) in value_obj.iter() {
                            let reference_props = self.collect_prefix(extend_key);
                            println!("reference props: {:?}", reference_props);
                            for (reference_key, reference_value) in reference_props {
                                let reference_name = &reference_key[extend_key.len() + 1..];
                                let new_reference_key = format!("{}.{}", prefix, reference_name);
                                reference_to_merge.push((new_reference_key, reference_value.clone())); 
                            }
                            
                            if let Value::Object(extend_obj) = extend_value {
                                // Use a temporary vector to store overridden values
                                overrides_to_merge.extend(self.merge_and_return(extend_obj, prefix.clone()));
                            } else {
                                panic!("extend override value did not evaluate to an object");
                            }
                        }
                        to_merge.extend(reference_to_merge.into_iter()); // Append reference properties to to_merge
                        to_merge.extend(overrides_to_merge.into_iter()); // Append overridden values to to_merge
                    } else {
                        self.merge(value_obj, new_key);
                    }
                },
                other_value => {
                    to_merge.push((new_key, other_value.clone()));
                }
            }
        }
        for (key, value) in to_merge {
            println!("inserting: {} = {:?}", key, value);
            self.tree.insert(key, value);
        }
    }
    
    // New helper method that performs the merge and returns the collected values
    pub fn merge_and_return(&mut self, obj: &IndexMap<String, Value>, prefix: String) -> Vec<(String, Value)> {
        let mut result: Vec<(String, Value)> = Vec::new();
        for (key, value) in obj.iter() {
            let new_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };
            match value {
                Value::Object(value_obj) => {
                    self.merge(value_obj, new_key);
                },
                other_value => {
                    result.push((new_key, other_value.clone()));
                }
            }
        }
        result
    }
                      
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.tree.get(key)
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
    use crate::context::parser::parse;

    use super::*;

    #[test]
    fn test_tree() {
        let input = r#"
            a = 1
            b = 2
            c = a + b
            d = c + 2
            e {
                f = true
                g = false
            }
        "#;

        let context = parse(input).unwrap();
        let specific_context = SpecificContext::from(&context);
        println!("{:?}", specific_context.tree);
    }

    #[test]
    fn test_extend() {
        let input = r#"
            number {
                minimum = number
                maximum = number
            }
            circle {
                center = point
                radius extend number {
                    minimum = 0
                }
            }
        "#;

        let context = parse(input).unwrap();
        let specific_context = SpecificContext::from(&context);
        println!("{:?}", specific_context.tree);
    }
}