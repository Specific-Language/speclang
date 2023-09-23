use std::collections::BTreeMap;
use serde_json::Map;
use serde_json::Value;
use crate::parser::expression::Specific;
use crate::parser::expression::Computed;

pub struct Context {
    map: Map<String, Value>,
    tree: BTreeMap<String, Computed>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            map: Map::new(),
            tree: BTreeMap::new(),
        }
    }

    pub fn from(map: Map<String, Value>) -> Self {
      Context {
          map,
          tree: BTreeMap::new(), // construct tree too?
      }
    }

    pub fn get(&self, key: &str) -> Option<&Computed> {
        self.tree.get(key)
    }

    pub fn insert(&mut self, key: String, value: &str) -> Result<(), &'static str> {
        let expr = Computed::from(value).unwrap();
        self.tree.insert(key, expr);
        Ok(())
    }

    pub fn eval(&self, value: &str) -> Result<Specific, &'static str> {
        let expr = Computed::from(value).unwrap();
        let result = expr.eval(&self);
        println!("Evaluated {} to {:?}", value, result);
        result
    }
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn test_variables() {
        let expression = "${a + b}";
        let mut context = Context::new();
        context.insert("a".to_owned(), "1.0").unwrap();
        context.insert("b".to_owned(), "2.0").unwrap();
        let result = context.eval(expression).unwrap();
        assert_eq!(result, Specific::Number(3.0));
    }
}
