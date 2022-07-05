/* TODOs
  - gracefully handle errors instead of unwrapping
  - transform in-place rather than copying (?)
  - review chosen solution with expert rustaceans
*/
pub mod speclang {
  use serde_json::{json, Value};
  use wasm_bindgen::prelude::*;

  #[wasm_bindgen]
  pub fn parse(raw_input: &str) -> String {
    let input = format!("input {{{}}}", raw_input);

    // collapsed error handling because main fn must return string
    let hcl = match hcl::from_str(&input) {
      Ok(json) => json,
      Err(error) => json!({ "error": error.to_string() }),
    };

    hcl.to_string()
  }

  // todo: Global Context { declare, modules, etc } Rust Type

  // fn hydrate(input: &Value, modules: &Value) -> Value {
  //   let mut output = json!({});
  //   let input_object = input.as_object().unwrap();
  //   if input_object.len() > 1 {
  //     for (key, value) in input_object {
  //       match key.as_str() {
  //         "declare" => declare(&mut output, value),
  //         _ => identity(&mut output, key, value),
  //       }
  //     }
  //   } else if input_object.len() == 1 {
  //     let single = input_object.keys().next().unwrap();
  //     output[single] = match single.as_str() {
  //       &_ => json!(input_object[single]), //lookup(single),
  //     }
  //   }
  //   output
  // }

  // fn identity(output: &mut Value, key: &str, value: &Value) {
  //   output[key] = hydrate(value);
  // }

  // fn lookup(input: &str) -> Value {
  //   // todo: actually lookup singles somewhere
  // }

  // fn extends(output: &mut Value, value: &Value) {
  //   for (child_key, child_value) in value.as_object().unwrap() {
  //     output[child_key] = hydrate(child_value);
  //   }
  // }

  // fn declare(output: &mut Value, value: &Value) {
  //   let props = value.as_object().unwrap().keys();
  //   if props.len() != 1 {
  //     println!("{}", props.last().unwrap());
  //     panic!("unhandled: 'declare' did not find single property")
  //   }
  //   let namespace = props.last().unwrap();
  //   output["modules"][namespace] = hydrate(&value[namespace]);
  // }

  // #[cfg(test)]
  // mod test {
  //   use super::*;

  //   #[test]
  //   fn assignment() {
  //     let input = "foo = \"bar\"";
  //     let output = parse(input);
  //     let expected_output = json!({
  //       "declare": {
  //         "foo": "bar"
  //       }
  //     })
  //     .to_string();
  //     assert_eq!(output, expected_output);
  //   }

  //   #[test]
  //   fn block() {
  //     let input = "foo {}";
  //     let output = parse(input);
  //     let expected_output = json!({
  //       "declare": {
  //         "foo": {}
  //       }
  //     })
  //     .to_string();
  //     assert_eq!(output, expected_output);
  //   }

  //   #[test]
  //   fn declare() {
  //     let input = "declare foo {}";
  //     let output = parse(input);
  //     let expected_output = json!({
  //       "modules": {
  //         "foo": {}
  //       }
  //     })
  //     .to_string();
  //     assert_eq!(output, expected_output);
  //   }
  // }
}
