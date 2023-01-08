#[cfg(test)]
pub mod definition {
  use serde_json::*;
  use crate::speclang::parse;
  use crate::speclang::test::helper;
  
  #[test]
  fn success_hcl2parse() {
    let input = r#"
      key = "value"
    "#;
    let expected = json!({
      "key": "value"
    });
    let result = parse(input);
    assert_eq!(result, expected.to_string());
  }

  #[test]
  fn error_hcl2parse_passthrough() {
    let input = "2 + 2 = 4";
    let message = helper::expect::error(input);
    assert!(message.contains("expected Identifier in line 1"));
  }

  #[test]
  fn success_definition_depth1() {
    let input = r#"
      foo {
        hello = "world"
        bar = true
      }
    "#;
    let expected = json!({
      "foo": {
        "hello": "world",
        "bar": true
      }
    });
    let result = parse(input);
    assert_eq!(result, expected.to_string());
  }

  #[test]
  fn success_definition_depth2() {
    let input = r#"
      foo {
        hello = "world"
        bar {
          baz = true
        }
      }
    "#;
    let expected = json!({
      "foo": {
        "hello": "world",
        "bar": {
          "baz": true
        }
      }
    });
    let result = parse(input);
    assert_eq!(result, expected.to_string());
  }

  #[test]
  fn success_definition_siblings() {
    let input = r#"
      foo {
        hello = "world"
        bar = true
      }
      foo {
        hello = "goodbye"
        bar = false
      }
    "#;
    let expected = json!({
      "foo": [
        {
          "hello": "world",
          "bar": true
        },
        {
          "hello": "goodbye",
          "bar": false
        }
      ]
    });
    let result = parse(input);
    assert_eq!(result, expected.to_string());
  }
}