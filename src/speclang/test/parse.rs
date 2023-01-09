#[cfg(test)]
mod cases {
  use crate::speclang::parse;
  use crate::speclang::test::shared;
  use serde_json::*;
  
  #[test]
  fn error_passthrough() {
    let input = "2 + 2 = 4";
    let message = shared::expect::error(input);
    assert!(message.contains("expected Identifier in line 1"));
  }

  mod assignment {
    use super::*;
    
    #[test]
    fn success_assignment_string() {
      let input = r#"
        key = "some value"
      "#;
      let expected = json!({
        "key": "some value"
      });
      let result = parse(input);
      assert_eq!(result, expected);
    }
    
    #[test]
    fn success_assignment_array() {
      let input = r#"
        key = ["some value", 1.23]
      "#;
      let expected = json!({
        "key": ["some value", 1.23]
      });
      let result = parse(input);
      assert_eq!(result, expected);
    }

    #[test]
    fn success_assignment_number() {
      let input = r#"
        key = 12345
      "#;
      let expected = json!({
        "key": 12345
      });
      let result = parse(input);
      assert_eq!(result, expected);
    }

    #[test]
    fn success_assignment_boolean() {
      let input = r#"
        key = true
      "#;
      let expected = json!({
        "key": true
      });
      let result = parse(input);
      assert_eq!(result, expected);
    }

    #[test]
    fn success_assignment_object() {
      let input = r#"
        key = {
          "inner_key": "inner_value"
        }
      "#;
      let expected = json!({
        "key": {
          "inner_key": "inner_value"
        }
      });
      let result = parse(input);
      assert_eq!(result, expected);
    }
  }

  mod definition {
    use super::*;

    #[test]
    fn success_hcl_empty_block() {
      let input = r#"
        foo {}
      "#;
      let expected = json!({
        "foo": {}
      });
      let result = parse(input);
      assert_eq!(result, expected);
    }

    #[test]
    fn success_hcl_single_assignment() {
      let input = r#"
        foo { bar = true }
      "#;
      let expected = json!({
        "foo": { "bar": true }
      });
      let result = parse(input);
      assert_eq!(result, expected);
    }

    #[test]
    fn success_hcl_multiple_assignments() {
      let input = r#"
        dog { name = "Rover" }
        cat { claws { sharp = true } }
      "#;
      let expected = json!({
        "dog": { "name": "Rover" },
        "cat": { "claws": { "sharp": true } }
      });
      let result = parse(input);
      assert_eq!(result, expected);
    }

    #[test]
    fn success_hcl_multiple_block_definitions() {
      let input = r#"
        foo {
          bar { baz = true }
          bar { baz = false }
        }
      "#;
      let expected = json!({
        "foo": {
          "bar": [
            { "baz": true },
            { "baz": false }
          ]
        }
      });
      let result = parse(input);
      assert_eq!(result, expected);
    }
  }
}
