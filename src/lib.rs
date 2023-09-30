pub mod specific;

// #[wasm_bindgen::prelude::wasm_bindgen]
// pub fn parse(input: &str) -> String {
//   let result = parser::parse(input).unwrap();
//   result.to_string()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     mod parse {
//         use super::*;
//         #[test]
//         fn define() {
//             let input = r#"x = "string""#;
//             let serialized = parse(input);
//             let deserialized: Value = serde_json::from_str(&serialized).unwrap();
//             assert_eq!(
//                 deserialized,
//                 json!({"x": "string"})
//             );
//         }
//     }
    
//     mod get {
//         use super::*;
//         #[test]
//         fn evaluate() {
//             let input = r#"
//             x = 5
//             y = 3.14
//             z = x + y
//         "#;
//             let serialized = parse(input);
//             let context: Value = serde_json::from_str(&serialized).unwrap();
//             let result = get("z", &context.to_string());
//             println!("{}\n{}", context, result);
//         }
//     }
// }
