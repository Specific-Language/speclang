pub mod parser;
pub mod validator;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn parse(input: &str) -> String {
  let result = parser::parse(input);
  result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;
    
    #[test]
    fn assignment() {
        let input = r#"x = "string""#;
        let serialized = parse(input);
        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            deserialized,
            json!({"x": "string"})
        );
    }
}
