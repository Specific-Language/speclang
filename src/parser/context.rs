use crate::parser::expression::Computed;
use crate::parser::expression::Literal;

use serde_json::Map;
use serde_json::Value;
use std::collections::BTreeMap;

pub struct Context {
    tree: BTreeMap<String, Computed>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            tree: BTreeMap::new(),
        }
    }

    pub fn from(map: Map<String, Value>) -> Self {
        let mut context = Self::new();
        for (key, value) in map {
            match value {
                Value::String(s) => {
                    let expr = Computed::from(s.as_str()).unwrap();
                    context.set(&key, expr);
                },
                Value::Number(n) => {
                    let expr = Computed::from(n.to_string().as_str()).unwrap();
                    context.set(&key, expr);
                },
                Value::Bool(b) => {
                    let expr = Computed::from(b.to_string().as_str()).unwrap();
                    context.set(&key, expr);
                },
                Value::Object(o) => {
                    let subcontext = Context::from(o);
                    for (subkey, subvalue) in subcontext.tree {
                        let name = format!("{}.{}", key, subkey);
                        context.set(&name, subvalue);
                    }
                }
                _ => panic!("Unexpected value type in context map")
            }
        }
        context
    }

    pub fn get(&self, name: &str) -> Option<&Computed> {
        self.tree.get(name)
    }

    pub fn set(&mut self, name: &str, expr: Computed) {
        self.tree.insert(name.to_owned(), expr);
    }

    pub fn set_parsed(&mut self, name: &str, input: &str) -> Result<(), &'static str> {
        let expr = Computed::from(input).unwrap();
        self.set(name, expr);
        Ok(())
    }

    pub fn eval(&self, name: &str) -> Result<Literal, &'static str> {
        self.get(name).ok_or("Variable not found in context")?.eval(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variables() {
        let expression = Computed::from("${a + b}").unwrap();
        let mut context = Context::new();
        context.set_parsed("a", "1.0").unwrap();
        context.set_parsed("b", "2.0").unwrap();
        let result = expression.eval(&context).unwrap();
        assert_eq!(result, Literal::Number(3.0));

        context.set_parsed("b", "3.0").unwrap();
        let result2 = expression.eval(&context).unwrap();
        assert_eq!(result2, Literal::Number(4.0));
    }

    #[test]
    fn test_new_context() {
        let context = Context::new();
        assert_eq!(context.tree.len(), 0);
    }

    #[test]
    fn test_context_set() {
        let mut context = Context::new();
        context.set_parsed("a", "1.0").unwrap();
        context.set_parsed("b", "2.0").unwrap();
        assert_eq!(context.tree.len(), 2);
    }

    #[test]
    fn test_context_literal_eval() {
        let mut context = Context::new();
        context.set_parsed("a", "1.0").unwrap();
        context.set_parsed("b", "2.0").unwrap();
        let result = context.eval("a").unwrap();
        assert_eq!(result, Literal::Number(1.0));
    }

    #[test]
    fn test_context_computed_eval() {
        let mut context = Context::new();
        context.set_parsed("a", "${b+c}").unwrap();
        context.set_parsed("b", "2.0").unwrap();
        context.set_parsed("c", "${-z}").unwrap();
        context.set_parsed("z", "5.0").unwrap();
        let result = context.eval("a").unwrap();
        assert_eq!(result, Literal::Number(-3.0));
    }

    #[test]
    fn test_context_from_map() {
        let mut map = Map::new();
        map.insert("a".to_owned(), serde_json::json!(1));
        map.insert("b".to_owned(), serde_json::json!(2));
        let context = Context::from(map);
        assert_eq!(context.tree.len(), 2);
        let result = context.eval("a").unwrap();
        assert_eq!(result, Literal::Number(1.0));
    }

    #[test]
    fn test_context_from_nested_object() {
        let mut map = Map::new();
        map.insert("a".to_owned(), serde_json::json!(1));
        map.insert("foo".to_owned(), serde_json::json!({"bar": 2}));
        let context = Context::from(map);
        let result = context.eval("foo.bar").unwrap();
        assert_eq!(result, Literal::Number(2.0));
    }
}
