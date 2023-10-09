use hcl::{template::Element, Value};
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

    pub fn apply_string_template(mut self, key: &str, template: hcl::Template) -> Self {
        let elements = template.elements();
        match elements.len() {
            0 => {
                panic!("Oops! Found an empty template");
            }
            1 => {
                let element: &Element = &elements[0];
                self.context.tree.insert(key.to_string(), element.into());
            },
            _ => {
                let elements: Vec<Specific> = elements.into_iter().map(Into::into).collect();
                let template = Specific::StringTemplate(elements);
                self.context.tree.insert(key.to_string(), template.into());
            }
        }
        self
    }

    pub fn apply_object(mut self, prefix: &str, obj: &IndexMap<String, Value>) -> Self {
        for (key, value) in obj.iter() {
            let destination = Self::compose_key(prefix, key);
            match value {
                Value::Object(value_obj) if value_obj.is_empty() => {
                    self.context.tree.insert(destination, Specific::Reference(key.clone()));
                }
                Value::Object(value_obj) => {
                    self = self.apply_object(&destination, value_obj);
                }
                Value::String(s) => {
                    let template_expr = hcl::TemplateExpr::from(s.as_str());
                    let template = hcl::Template::from_expr(&template_expr).expect("Expected a template");
                    self = self.apply_string_template(&destination, template);
                }
                _ => {
                    self.context.tree.insert(destination, value.into());
                }
            }
        }
        self
    }

    fn compose_key(prefix: &str, name: &str) -> String {
        if prefix.len() == 0 {
            println!("~ {}", name);
        } else {
            println!("~ {}.{}", prefix, name);
        }
        match (name.is_empty(), prefix.is_empty()) {
            (true, _) => prefix.to_string(),
            (false, true) => name.to_string(),
            (false, false) => format!("{}.{}", prefix, name)
        }
    }
}
