pub mod define {
  use crate::speclang::types::Trie;
  use serde_json::Value;

  pub fn spec(trie: &mut Trie, json: &Value, prefix: &str) {
    let name = prefix.to_owned();
    match json {
      Value::String(s) => {
        trie.insert(&name, Value::String(s.to_owned()));
      }
      Value::Number(n) => {
        trie.insert(&name, Value::Number(n.to_owned()));
      }
      Value::Bool(b) => {
        trie.insert(&name, Value::Bool(b.to_owned()));
      }
      Value::Object(map) => {
        for (key, value) in map {
          let new_prefix = format!("{}-{}", prefix, key);
          spec(trie, value, &new_prefix);
        }
      }
      Value::Array(arr) => {
        for (i, value) in arr.iter().enumerate() {
          let new_prefix = format!("{}-{}", prefix, i);
          spec(trie, value, &new_prefix);
        }
      }
      _ => panic!("{}", "Unknown JSON value type")
    }
  }
}
