use std::collections::BTreeMap;
use serde_json::Map;
use serde_json::Value;
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

    pub fn get(&self, name: &str) -> Option<&Computed> {
        self.tree.get(name)
    }

    pub fn set(&mut self, name: String, value: &str) -> Result<(), &'static str> {
        let expr = Computed::from(value).unwrap();
        self.tree.insert(name, expr);
        Ok(())
    }

    pub fn to_map(&self) -> Map<String, Value> {
        let mut map = Map::new();
        for (key, value) in &self.map {
            map.insert(key.to_owned(), value.to_owned());
        }
        map
    }
}

#[cfg(test)]
mod tests {
  use super::*;
    use crate::parser::expression::Specific;
    
    #[test]
    fn test_variables() {
        let expression = Computed::from("${a + b}").unwrap();
        let mut context = Context::new();
        context.set("a".to_owned(), "1.0").unwrap();
        context.set("b".to_owned(), "2.0").unwrap();
        let result = expression.eval(&context).unwrap();
        assert_eq!(result, Specific::Number(3.0));

        context.set("b".to_owned(), "3.0").unwrap();
        let result2 = expression.eval(&context).unwrap();
        assert_eq!(result2, Specific::Number(4.0));
    }
}
