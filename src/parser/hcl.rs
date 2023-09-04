use std::collections::HashMap;

use serde_json::Value;

use crate::validator::validate;

pub fn parse_with_context(input: &str, context: &mut HashMap<String, Value>) -> serde_json::Value {
  let result = hcl::from_str(input);
  let parsed_input: serde_json::Value = match result {
    Ok(value) => value,
    Err(error) => panic!("parser::hcl::parse error {}", &error.to_string()),
  };
  validate(&parsed_input, context).expect("validator::validate error");
  parsed_input.to_owned()
}

pub fn parse(input: &str) -> serde_json::Value {
  let mut context: HashMap<String, Value> = HashMap::new();
  parse_with_context(input, &mut context)
}

#[cfg(test)]
mod tests {
  use crate::parser::hcl::parse;
  mod assignment {
    use std::collections::HashMap;

    use serde_json::{json, Value};

    #[test]
    fn string() {
      let input = r#"x = "string""#;
      let result = super::parse(input);
      assert_eq!(
        result["x"],
        serde_json::json!("string")
      );
    }

    #[test]
    fn number() {
      let input = r#"x = 3.14"#;
      let result = super::parse(input);
      assert_eq!(
        result["x"],
        serde_json::json!(3.14)
      );
    }

    #[test]
    fn boolean() {
      let input = r#"x = true"#;
      let result = super::parse(input);
      assert_eq!(
        result["x"],
        serde_json::json!(true)
      );
    }

    #[test]
    fn list() {
      let input = r#"x = [true, "two", 3]"#;
      let result = super::parse(input);
      assert_eq!(
        result["x"],
        serde_json::json!([true, "two", 3])
      );
    }

    #[test]
    fn map() {
      let input = r#"
      x = {
        a = true,
        b = "two",
        c = 3
      }
  "#;
      let result = super::parse(input);
      assert_eq!(
        result["x"],
        serde_json::json!({
          "a": true,
          "b": "two",
          "c": 3
        })
      );
    }

    #[test]
    fn primitive_reference() {
      let input = r#"x = number"#;
      let result = super::parse(input);
      assert_eq!(
        result["x"],
        serde_json::json!("${number}")
      );
    }

    #[test]
    fn dictionary_reference() {
      let input = r#"
      x = number
      y = x
"#;
      let result = super::parse(input);
      assert_eq!(
        result["x"],
        serde_json::json!("${number}")
      );
      assert_eq!(
        result["y"],
        serde_json::json!("${x}")
      );
    }

    #[test]
    fn expression() {
      let input = r#"x = y + 2"#;
      let mut context: HashMap<String, Value> = HashMap::new();
      context.insert("y".to_owned(), json!(5));
      let result = crate::parser::hcl::parse_with_context(input, &mut context);
      assert_eq!(
        result["x"],
        serde_json::json!("${y + 2}")
      );
    }

    #[test]
    fn overrides() {
      let input = r#"
      x = 1
      x = 2
  "#;
      let result = super::parse(input);
      assert_eq!(
        result["x"],
        serde_json::json!(2)
      );
    }

    #[test]
    fn union() {
      let input = r#"x = number || string"#;
      let result = super::parse(input);
      assert_eq!(
        result["x"],
        serde_json::json!("${number || string}")
      );
    }

    #[test]
    fn intersection() {
      let input = r#"x = number && string"#;
      let result = super::parse(input);
      assert_eq!(
        result["x"],
        serde_json::json!("${number && string}")
      );
    }
  }
}
