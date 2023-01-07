pub mod speclang {
  #[cfg(target_arch = "wasm32")]
  #[wasm_bindgen::prelude::wasm_bindgen]
  pub fn parse(input: &str) -> String {
    let formatted = format!("input {{{}}}", input);
    let parsed: serde_json::Value = match hcl::from_str(&formatted) {
      Ok(value) => value,
      Err(error) => wasm_bindgen::throw_str(&error.to_string()),
    };
    parsed["input"].to_string()
  }
  
  #[cfg(not(target_arch = "wasm32"))]
  pub fn parse(input: &str) -> String {
    let formatted = format!("input {{{}}}", input);
    let parsed: serde_json::Value = match hcl::from_str(&formatted) {
      Ok(value) => value,
      Err(error) => panic!("{}", error)
    };
    parsed["input"].to_string()
  }

  #[cfg(test)]
  mod test {
    mod helper;
    use serde_json::*;

    #[test]
    fn success_definition_depth0() {
      let input = r#"
        key = "value"
      "#;
      let expected = json!({
        "key": "value"
      });
      helper::expect::success(input, expected);
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
      helper::expect::success(input, expected);
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
      helper::expect::success(input, expected);
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
      helper::expect::success(input, expected);
    }

    #[test]
    fn error_hcl_parse_passthrough() {
      let input = "invalid input";
      let expected = "expected BlockBody, Identifier, or StringLit in line 1";
      helper::expect::error(input, expected)
    }
  }  
}
