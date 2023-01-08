use std::fmt;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use serde_json::Value;

// Name
#[derive(Eq, Clone)]
pub struct Name(String);

impl Hash for Name {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.hash(state);
  }
}
impl PartialEq for Name {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}
impl PartialOrd for Name {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.0.partial_cmp(&other.0)
  }
}
impl Ord for Name {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.0.cmp(&other.0)
  }
}
impl fmt::Display for Name
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", &self.0)
  }
}

// Dictinoary
pub struct Dictionary(HashMap<Name, Value>);

impl Dictionary {
  pub fn new() -> Self {
    Dictionary(HashMap::new())
  }
  pub fn insert(&mut self, key: Name, value: Value) -> Option<Value> {
    self.0.insert(key, value)
  }
  // pub fn get(&self, k: &Name) -> Option<&Value> {
  //   self.0.get(k)
  // }
  // pub fn iter(&self) -> impl Iterator<Item = (&Name, &Value)> {
  //   self.0.iter()
  // }
}
impl fmt::Display for Dictionary
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut output = String::new();
    for (key, value) in &self.0 {
      output.push_str(&format!("{}: {}\n", key, value));
    }
    write!(f, "{}", output)
  }
}

pub mod define {
  use super::*;
  use serde_json::Value;

  pub fn spec_json(dict: &mut Dictionary, json: &Value, prefix: &str) {
    let name = Name(prefix.to_owned());
    match json {
      Value::Object(map) => {
        let mut references = serde_json::Map::new();
        for (key, value) in map {
          let new_prefix = format!("{}-{}", prefix, key);
          spec_json(dict, value, &new_prefix);
          references.insert(key.to_owned(), Value::String(new_prefix.to_owned()));
        }
        dict.insert(name.clone(), Value::Object(references));
      }
      Value::Bool(b) => {
        let value = Value::Bool(b.to_owned());
        dict.insert(name, value);
      }
      Value::Number(n) => {
        let value = Value::Number(n.to_owned());
        dict.insert(name, value);
      }
      Value::String(s) => {
        let value = Value::String(s.to_owned());
        dict.insert(name, value);
      }
      Value::Array(arr) => {
        let mut array = Vec::new();
        for (i, value) in arr.iter().enumerate() {
          let new_prefix = format!("{}-{}", prefix, i);
          spec_json(dict, value, &new_prefix);
          let reference = new_prefix.to_owned();
          array.push(Value::String(reference));
        }
        dict.insert(name.clone(), Value::Array(array));
      }    
      _ => panic!("{}", "Unknown JSON value type")
    }
  }
}
