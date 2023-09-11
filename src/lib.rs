use serde_json::*;

pub mod parser;
// pub mod validator;
// pub mod evaluator;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn parse(input: &str) -> String {
  let result = parser::parse(input);
  result.to_string()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn get(name: &str, context: &str) -> String {
  let deserialized: Map<String, Value> = serde_json::from_str(context).unwrap();
//   let result = evaluator::get(name, deserialized);
//   result.to_string()
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    mod parse {
        use super::*;
        #[test]
        fn define() {
            let input = r#"x = "string""#;
            let serialized = parse(input);
            let deserialized: Value = serde_json::from_str(&serialized).unwrap();
            assert_eq!(
                deserialized,
                json!({"x": "string"})
            );
        }
    }
    
    mod get {
        use super::*;
        #[test]
        fn evaluate() {
            let input = r#"
            x = 5
            y = 3.14
            z = x + y
        "#;
            let serialized = parse(input);
            let context: Value = serde_json::from_str(&serialized).unwrap();
            let result = get("z", &context.to_string());
            println!("{}\n{}", context, result);
        }
    }
}
