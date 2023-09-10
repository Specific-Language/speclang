use serde_json::{Value, Map};

use crate::validator::validate;

pub fn parse_with_context(input: &str, context: &mut Map<String, Value>) -> Value {
  let result = hcl::from_str(input);
  let parsed_input: Value = match result {
    Ok(value) => value,
    Err(error) => panic!("parser::parse error {}", &error.to_string()),
  };
  validate(&parsed_input, context).expect("validator::validate error");
  parsed_input.to_owned()
}

pub fn parse(input: &str) -> Value {
  let mut context: Map<String, Value> = Map::new();
  parse_with_context(input, &mut context)
}

#[cfg(test)]
mod tests {
  use super::*;
  mod assignment {
    use super::*;
    use serde_json::json;

    #[test]
    fn string() {
      let input = r#"x = "string""#;
      let result = parse(input);
      assert_eq!(
        result["x"],
        json!("string")
      );
    }

    #[test]
    fn number() {
      let input = r#"x = 3.14"#;
      let result = parse(input);
      assert_eq!(
        result["x"],
        json!(3.14)
      );
    }

    #[test]
    fn boolean() {
      let input = r#"x = true"#;
      let result = parse(input);
      assert_eq!(
        result["x"],
        json!(true)
      );
    }

    #[test]
    fn list() {
      let input = r#"x = [true, "two", 3]"#;
      let result = parse(input);
      assert_eq!(
        result["x"],
        json!([true, "two", 3])
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
      let result = parse(input);
      assert_eq!(
        result["x"],
        json!({
          "a": true,
          "b": "two",
          "c": 3
        })
      );
    }

    #[test]
    fn primitive_reference() {
      let input = r#"x = number"#;
      let result = parse(input);
      assert_eq!(
        result["x"],
        json!("${number}")
      );
    }

    #[test]
    fn dictionary_reference() {
      let input = r#"
      x = number
      y = x
"#;
      let result = parse(input);
      assert_eq!(
        result["x"],
        json!("${number}")
      );
      assert_eq!(
        result["y"],
        json!("${x}")
      );
    }

    #[test]
    fn expression() {
      let input = r#"x = y + 2"#;
      let mut context: Map<String, Value> = Map::new();
      context.insert("y".to_owned(), json!(5));
      let result = parse_with_context(input, &mut context);
      assert_eq!(
        result["x"],
        json!("${y + 2}")
      );
    }

  // todo: test that an error is thrown
  //   #[test]
  //   fn overrides() {
  //     let input = r#"
  //     x = 1
  //     x = 2
  // "#;
  //     let result = parse(input);
  //     assert_eq!(
  //       result["x"],
  //       json!(2)
  //     );
  //   }

    #[test]
    fn union() {
      let input = r#"x = number || string"#;
      let result = parse(input);
      assert_eq!(
        result["x"],
        json!("${number || string}")
      );
    }

    #[test]
    fn intersection() {
      let input = r#"x = number && string"#;
      let result = parse(input);
      assert_eq!(
        result["x"],
        json!("${number && string}")
      );
    }
  }
}
