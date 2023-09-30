use hcl::{Value, template::Element, Template, Expression};
use indexmap::IndexMap;

use super::Context;
use value::Specific;

pub mod value;

pub struct Builder {
    context: Context,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
        }
    }

    pub fn build(self) -> Context {
        self.context
    }

    pub fn apply_value(mut self, key: &str, value: Value) -> Self {
        let specific_value = Specific::Literal(value);
        self.context.tree.insert(key.to_string(), specific_value);
        self
    }

    pub fn apply_template(mut self, key: &str, template: Template) -> Self {    
        let elements = template.elements();
        if elements.len() == 1 {
            let single: &Element = &elements[0];
            self.context.tree.insert(key.to_string(), single.into());
        } else {
            let template_vec: Vec<Specific> = elements.into_iter()
                .map(Into::into)
                .collect();
            let value = Specific::Template(template_vec);
            self.context.tree.insert(key.to_string(), value.into());
        };
        self
    }
    
    pub fn apply_extends(mut self, prefix: &str, extends_obj: &IndexMap<String, Value>) -> Self {
        let mut temp_vec: Vec<(String, Specific)> = Vec::new();

        for (reference, overrides) in extends_obj.iter() {
            let new_props = self.context.collect_prefix(reference);
            for (ref_key, ref_value) in new_props {
                let name: String = Self::extract_property_name(ref_key, reference);
                let new_key = Self::compose_key(prefix, name.as_str());
                temp_vec.push((new_key, ref_value.clone()));
            }
            for (new_key, new_value) in &temp_vec {
                self.context.tree.insert(new_key.clone(), new_value.clone());
            }
            let overrides_obj = overrides.as_object().unwrap();
            self = self.apply_object(prefix, overrides_obj);

            temp_vec.clear();
        }
        self
    }
    
    pub fn apply_object(mut self, prefix: &str, obj: &IndexMap<String, Value>) -> Self {
        for (key, value) in obj.iter() {
            let new_key = Self::compose_key(prefix, key);
            match key.as_str() {
                "extends" => {
                    let extends_obj = value.as_object().unwrap();
                    self = self.apply_extends(prefix, extends_obj);
                }
                _ => match value {
                    Value::Object(value_obj) => {
                        self = self.apply_object(&new_key, value_obj);
                    }
                    Value::String(s) => {
                        let template_expr = hcl::TemplateExpr::from(s.as_str());
                        let template = hcl::Template::from_expr(&template_expr).unwrap();
                        self = self.apply_template(&new_key, template);
                    },
                    _ => {
                        self = self.apply_value(&new_key, value.clone());
                    }
                }
            }
        }
        self
    }    

    fn compose_key(prefix: &str, key: &str) -> String {
        match key.is_empty() {
            true => prefix.to_string(),
            false => match prefix.is_empty() {
                true => key.to_string(),
                false => format!("{}.{}", prefix, key),
            }
        }
    }

    fn extract_property_name(reference: &str, parent_name: &str) -> String {
        reference[parent_name.len()..].trim_start_matches('.').to_string()
    }
}
