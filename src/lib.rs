pub mod speclang {
  mod dictionary;
  mod shared;
  use serde_json::Value;

  fn parse_input(spec: &str) -> Result<Value, hcl::Error> {
    let input = format!("spec {{{}}}", spec);
    hcl::from_str(&input)
  }

  #[cfg(target_arch = "wasm32")]
  #[wasm_bindgen::prelude::wasm_bindgen]
  pub fn parse(spec: &str) -> String {
    let parsed_input = match parse_input(spec) {
      Ok(value) => value,
      Err(error) => wasm_bindgen::throw_str(&error.to_string()),
    };
    let spec_json = &parsed_input["spec"];
    spec_json.to_string()
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn parse(spec: &str) -> String {
    let parsed_input = match parse_input(spec) {
      Ok(value) => value,
      Err(error) => panic!("{}", &error.to_string()),
    };
    let spec_json = &parsed_input["spec"];
    let mut dict = dictionary::Dictionary::new();
    dictionary::define::spec_json(&mut dict, spec_json, "spec");
    panic!("{}", dict);
    spec_json.to_string()
  }
  
  #[cfg(test)]
  mod test {
    mod shared;
    mod cases;
  }
}
