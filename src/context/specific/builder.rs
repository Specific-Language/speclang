use hcl::Value;
use indexmap::IndexMap;

use super::Specific;

pub struct Builder {
    context: Specific,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            context: Specific::new(),
        }
    }

    pub fn build(self) -> Specific {
        self.context
    }

    pub fn apply_value(mut self, key: &str, value: Value) -> Self {
        println!("{} = {:?}", key, value);
        let mut merge_queue: Vec<(String, Value)> = Vec::new();
        match &value {
            Value::String(s) if s.starts_with("${") && s.ends_with("}") => {
                // todo : evaluate more thoroughly
                let reference = &s[2..s.len() - 1];
                let results = self.context.collect_prefix(reference);
                // if results.len() == 0 {
                //     panic!("reference {} not found", reference);
                // }
                for (k, v) in results {
                    let identifier = &k[reference.len()..].trim_start_matches('.');
                    let new_key = Self::compose_key(key, identifier);
                    merge_queue.push((new_key, v.clone()));
                }
            },
            _ => {
                self.context.tree.insert(key.to_string(), value);
            }
        }
        for (new_key, new_value) in merge_queue {
            self = self.apply_value(&new_key, new_value);
        }
        self
    }

    pub fn apply_extends(mut self, prefix: &str, value_obj: &IndexMap<String, Value>) -> Self {
        let mut merge_queue: Vec<(String, Value)> = Vec::new();
        for (reference, extends_obj) in value_obj.iter() {
            // Look up reference and hydrate
            let ref_then_dot = format!("{}.", reference);
            let ref_props = self.context.collect_prefix(ref_then_dot.as_str());
            for (ref_key, ref_value) in ref_props {
                let name = &ref_key[reference.len()..].trim_start_matches('.');
                let new_key = Self::compose_key(prefix, name);
                merge_queue.push((new_key, ref_value.clone()));
            }
            // Handle overrides in extends block
            if let Value::Object(extends_obj) = extends_obj {
                for (k, v) in extends_obj.iter() {
                    let new_key = Self::compose_key(prefix, k);
                    merge_queue.push((new_key, v.clone()));
                }
            } else {
                panic!("extends override value did not evaluate to an object");
            }
        }
        for (new_key, new_value) in merge_queue {
            self = self.apply_value(&new_key, new_value);
        }
        self
    }    
    
    pub fn merge(mut self, prefix: &str, obj: &IndexMap<String, Value>) -> Self {
        for (key, value) in obj.iter() {
            let new_key = Self::compose_key(prefix, key);
            match key.as_str() {
                "extends" => {
                    if let Value::Object(extends_obj) = value {
                        self = self.apply_extends(prefix, extends_obj);
                    } else {
                        panic!("extends value did not evaluate to an object");
                    }
                }
                _ => match value {
                    Value::Object(value_obj) => {
                        self = self.merge(&new_key, value_obj);
                    }
                    _ => {
                        self = self.apply_value(&new_key, value.clone());
                    }
                }
            }
        }
        self
    }    

    fn compose_key(prefix: &str, key: &str) -> String {
        match prefix.is_empty() {
            true => key.to_string(),
            false => format!("{}.{}", prefix, key),
        }
    }
}
