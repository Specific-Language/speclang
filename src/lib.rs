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
    let hcl: Value = hcl::from_str(&input).unwrap();
    let spec: Value = hydrate(&hcl);
    spec.to_string()
  }

  fn hydrate(input_json: &Value) -> Value {
    let mut output = json!({});
    let input = input_json.as_object().unwrap();
    if input.len() > 1 {
      for (key, value) in input {
        match key.as_str() {
          // "extends" => extends(&mut output, value),
          _ => identity(&mut output, key, value),
        }
      }
    } else if input.len() == 1 {
      let single = input.keys().next().unwrap();
      output[single] = match single.as_str() {
        &_ => json!(input[single]), //lookup(single),
      }
    }
    output
  }

  fn identity(output: &mut Value, key: &str, value: &Value) {
    output[key] = hydrate(value);
  }

  // fn lookup(input: &str) -> Value {
  //   // todo: actually lookup singles somewhere
  // }

  // fn extends(output: &mut Value, value: &Value) {
  //   for (child_key, child_value) in value.as_object().unwrap() {
  //     output[child_key] = hydrate(child_value);
  //   }
  // }

  #[cfg(test)]
  mod test {
    use super::*;

    #[test]
    fn assignment() {
      let input = "foo = \"bar\"";
      let output = parse(input);
      let expected_output = json!({
        "input": {
          "foo": "bar"
        }
      })
      .to_string();
      assert_eq!(output, expected_output);
    }

    #[test]
    fn block() {
      let input = "foo {}";
      let output = parse(input);
      let expected_output = json!({
        "input": {
          "foo": {}
        }
      })
      .to_string();
      assert_eq!(output, expected_output);
    }
  }
}
