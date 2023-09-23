use crate::parser::expression::Expression;
use serde_json::Map;
use serde_json::Value;
use std::collections::BTreeMap;

pub struct Context {
    tree: BTreeMap<String, Expression>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            tree: BTreeMap::new(),
        }
    }

    pub fn from(map: Map<String, Value>) -> Self {
        // flattens the map into a string->expression tree
        // todo : objects and arrays
        let mut tree = BTreeMap::new();
        for (key, value) in map {
            match value {
                Value::String(s) => {
                    let expr = Expression::from(s.as_str()).unwrap();
                    tree.insert(key, expr);
                },
                Value::Number(n) => {
                    let expr = Expression::from(n.to_string().as_str()).unwrap();
                    tree.insert(key, expr);
                },
                Value::Bool(b) => {
                    let expr = Expression::from(b.to_string().as_str()).unwrap();
                    tree.insert(key, expr);
                },
                _ => panic!("Unexpected value type in context map")
            }
        }
        Context { tree }
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
        assert_eq!(context.tree.len(), 0);
    }

    #[test]
    fn test_context_from_map() {
        let mut map = Map::new();
        map.insert("a".to_owned(), serde_json::json!(1));
        map.insert("b".to_owned(), serde_json::json!(2));
        let context = Context::from(map);
        assert_eq!(context.tree.len(), 2);
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
