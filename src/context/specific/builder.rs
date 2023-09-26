use hcl::Value;
use indexmap::IndexMap;

use super::SpecificContext;

pub struct Builder {
    context: SpecificContext,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            context: SpecificContext::new(),
        }
    }

    pub fn build(self) -> SpecificContext {
        self.context
    }

    pub fn apply_value(mut self, key: &str, value: Value) -> Self {
        println!("{} = {:?}", key, value);
        self.context.tree.insert(key.to_string(), value);
        self
    }

    pub fn apply_extends(mut self, prefix: &str, value_obj: &IndexMap<String, Value>) -> Self {
        let mut merge_queue: Vec<(String, Value)> = Vec::new();
        for (reference, extends_obj) in value_obj.iter() {
            // Look up reference and hydrate
            let ref_props = self.context.collect_prefix(reference);
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
