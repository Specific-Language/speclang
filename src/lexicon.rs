pub mod define {
  use crate::context::ContextNode;
  use serde_json::Value;

  pub fn spec(node: &mut ContextNode, key: &str, json: &Value) {
    match json {
      Value::String(s) => {
        node.insert(key, Value::String(s.to_owned()));
      }
      Value::Number(n) => {
        node.insert(key, Value::Number(n.to_owned()));
      }
      Value::Bool(b) => {
        node.insert(key, Value::Bool(b.to_owned()));
      }
      Value::Object(map) => {
        if map.len() == 0 {
          node.insert(key, Value::Object(serde_json::Map::new()));
          println!("inserting empty object at {} with key {}", node, key);
        }
        for (child_key, value) in map {
          // let new_prefix = format!("{}.{}", key, child_key);
          spec(node, child_key, value);
        }
      }
      Value::Array(arr) => {
        println!("node {} {}", node, key);

        node.insert(key, Value::String("${list}".to_owned()));

        for (i, value) in arr.iter().enumerate() {
          if let Some(reference) = value.as_str() {
            if reference.starts_with("${") && reference.ends_with("}") {
              let name = &reference[2..reference.len() - 1];
              if node.get(name).is_none() {
                panic!("Reference not found: {}", name);
              }
            }
          }
          let new_prefix = format!("{}.{}", key, i);
          let new_node = node.get_mut(key).unwrap();
          spec(new_node, &new_prefix, value);
        }
        let new_node = node.get(key).unwrap();
        println!("new_node {}", new_node);
      }
      _ => panic!("{}", "Unknown JSON value type")
    }
  }
}
