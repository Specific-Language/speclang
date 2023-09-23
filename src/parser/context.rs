use std::collections::BTreeMap;
use serde_json::Map;
use serde_json::Value;
use crate::parser::expression::Expression;

pub struct Context {
    map: Map<String, Value>,
    tree: BTreeMap<String, Expression>,
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

    pub fn get(&self, name: &str) -> Option<&Expression> {
        self.tree.get(name)
    }

    pub fn set(&mut self, name: String, value: &str) -> Result<(), &'static str> {
        let expr = Expression::from(value).unwrap();
        self.tree.insert(name, expr);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
  use super::*;
    use crate::parser::expression::Static;
    
    #[test]
    fn test_variables() {
        let expression = Expression::from("${a + b}").unwrap();
        let mut context = Context::new();
        context.set("a".to_owned(), "1.0").unwrap();
        context.set("b".to_owned(), "2.0").unwrap();
        let result = expression.eval(&context).unwrap();
        assert_eq!(result, Static::Number(3.0));

        context.set("b".to_owned(), "3.0").unwrap();
        let result2 = expression.eval(&context).unwrap();
        assert_eq!(result2, Static::Number(4.0));
    }

    #[test]
    fn test_new_context() {
        let context = Context::new();
        assert_eq!(context.map.len(), 0);
        assert_eq!(context.tree.len(), 0);
    }

    #[test]
    fn test_context_from_map() {
        let mut map = Map::new();
        map.insert("a".to_owned(), serde_json::json!(1));
        map.insert("b".to_owned(), serde_json::json!(2));
        let context = Context::from(map);
        assert_eq!(context.map.len(), 2);
        assert_eq!(context.tree.len(), 0);
    }

    #[test]
    fn test_context_set() {
        let mut context = Context::new();
        context.set("a".to_owned(), "1.0").unwrap();
        context.set("b".to_owned(), "2.0").unwrap();
        assert_eq!(context.tree.len(), 2);
    }

    #[test]
    fn test_context_get() {
        let mut context = Context::new();
        context.set("a".to_owned(), "1.0").unwrap();
        context.set("b".to_owned(), "2.0").unwrap();
        let result = context.get("a").unwrap().eval(&context).unwrap();
        assert_eq!(result, Static::Number(1.0));
    }

    #[test]
    fn test_context_complex_get() {
        let mut context = Context::new();
        context.set("a".to_owned(), "${b+c}").unwrap();
        context.set("b".to_owned(), "2.0").unwrap();
        context.set("c".to_owned(), "${-z}").unwrap();
        context.set("z".to_owned(), "5.0").unwrap();
        let result = context.get("a").unwrap().eval(&context).unwrap();
        assert_eq!(result, Static::Number(-3.0));
    }
}
