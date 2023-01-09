use std::collections::HashMap;
use std::fmt;
use serde_json::Value;

pub struct Trie {
  root: TrieNode,
}

struct TrieNode {
  children: HashMap<String, TrieNode>,
  value: Option<Value>,
}

impl TrieNode {
  fn new() -> Self {
    TrieNode {
      children: HashMap::new(),
      value: None,
    }
  }
}

impl Trie {
  pub fn new() -> Self {
    Trie { root: TrieNode::new() }
  }
  pub fn insert(&mut self, key: &str, value: Value) {
    let keys: Vec<&str> = key.split("-").collect();
    let mut current_node = &mut self.root;
    for key in keys {
      current_node = current_node.children.entry(key.to_owned()).or_insert(TrieNode::new());
    }
    current_node.value = Some(value);
  }
  pub fn get(&self, key: &str) -> Option<&Value> {
    let keys: Vec<&str> = key.split("-").collect();
    let mut current_node = &self.root;
    for key in keys {
      current_node = match current_node.children.get(key) {
        Some(child) => child,
        None => return None,
      };
    }
    current_node.value.as_ref()
  }
}

impl fmt::Display for Trie {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f)?;
    fn print_trie(trie: &TrieNode, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
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
    print_trie(&self.root, f, 0)
  }
}
