use std::collections::HashMap;
use std::fmt;
use serde_json::Value;

pub struct ContextNode {
  pub children: HashMap<String, ContextNode>,
  pub value: Option<Value>,
}

impl ContextNode {
  pub fn new() -> Self {
    ContextNode {
      children: HashMap::new(),
      value: None,
    }
  }
  pub fn from_value(value: Value) -> Self {
    ContextNode { 
      children: HashMap::new(),
      value: Some(value)
    }
  }
  pub fn insert(&mut self, key: &str, value: Value) {
    let node = ContextNode::from_value(value);
    self.children.insert(key.to_string(), node);
  }
  pub fn get(&self, key: &str) -> Option<&ContextNode> {
    self.children.get(key)
  }
  pub fn get_mut(&mut self, key: &str) -> Option<&mut ContextNode> {
    self.children.get_mut(key)
  }
}

impl fmt::Display for ContextNode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      print_trie(self, f, 0)
  }
}

fn print_trie(trie: &ContextNode, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
  let indent_string = " ".repeat(indent);
  for (key, child) in &trie.children {
      write!(f, "|{}{}: ", indent_string, key)?;
      match &child.value {
          Some(value) => write!(f, "{}", value)?,
          None => (),
      }
      writeln!(f)?;
      print_trie(child, f, indent + 2)?;
  }
  Ok(())
}
