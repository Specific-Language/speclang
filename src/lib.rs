pub mod speclang {
  use serde_json::json;
  use wasm_bindgen::prelude::*;

  #[wasm_bindgen]
  pub fn parse(raw_input: &str) -> String {
    let input = format!("input {{{}}}", raw_input);

    // collapsed error handling because main fn must return string
    let hcl = match hcl::from_str(&input) {
      Ok(json) => json,
      Err(error) => json!({ 
        "error": error.to_string() 
      }),
    };

    hcl["input"].to_string()
  }
}
