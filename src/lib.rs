pub mod speclang {
  use serde_json::Value;
  
  #[cfg(target_arch = "wasm32")]
  #[wasm_bindgen::prelude::wasm_bindgen]
  pub fn parse(input: &str) -> String {
    let formatted = format!("input {{{}}}", input);
    print!("here {}", formatted);
    let parsed: Value = match hcl::from_str(&formatted) {
      Ok(value) => value,
      Err(error) => wasm_bindgen::throw_str(&error.to_string()),
    };
    parsed["input"].to_string()
  }
  
  #[cfg(not(target_arch = "wasm32"))]
  pub fn parse(input: &str) -> String {
    let formatted = format!("input {{{}}}", input);
    print!("here {}", formatted);
    let parsed: Value = match hcl::from_str(&formatted) {
      Ok(value) => value,
      Err(error) => panic!("{}", error)
    };
    parsed["input"].to_string()
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn success_definition_depth0() {
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

    #[test]
    fn error_hcl_parse_passthrough() {
      let result = std::panic::catch_unwind(|| {
        parse("invalid input");
      });
      let caught_error = match result {
        Ok(_) => panic!("Expected an error"),
        Err(error) => error,
      };
      let message = match caught_error.downcast_ref::<String>() {
        Some(value) => value,
        None => panic!("Expected a string error value"),
      };
      assert!(
        message.contains("expected BlockBody, Identifier, or StringLit in line 1")
      );
    }      
  }  
}
