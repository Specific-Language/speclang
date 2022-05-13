use serde_json::{Error, Value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
  let hcl: Value = hcl::from_str(input).unwrap();
  // todo: extend HCL into Specific Language
  let ser: Result<String, Error> = serde_json::to_string(&hcl);
  match ser {
    Ok(o) => return o,
    Err(_e) => panic!("failed to serialize"), // todo: handle error
  }
}
