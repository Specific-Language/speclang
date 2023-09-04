// pub mod types;
pub mod parser;
pub mod validator;
// pub mod generator;
// pub mod utils;

// #[wasm_bindgen::prelude::wasm_bindgen]
// pub fn define_str(name: &str, spec: &str) {
// //   define(&mut node, name, spec);
// //   println!("{}", node);
// }

// #[wasm_bindgen::prelude::wasm_bindgen]
// pub fn parse_str(spec: &str) -> String {
// //   let output = parse(spec);
// //   serde_json::to_string_pretty(&output).unwrap()
// }

// pub fn define(node: &mut context::ContextNode, name: &str, spec: &str) {
//   let spec_json = parse(spec);
//   lexicon::define::spec(node, name, &spec_json);
// }

// pub fn parse(spec: &str) -> serde_json::Value {
//   let input = format!("spec {{{}}}", spec);
//   let result = hcl::from_str(&input);
//   let parsed_input: serde_json::Value = match result {
//     Ok(value) => value,
//     Err(error) => panic!("Parse error! {}", &error.to_string()),
//   };
//   parsed_input["spec"].to_owned()
// }
