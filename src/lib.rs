pub mod speclang {
  fn parse_input(spec: &str) -> Result<serde_json::Value, hcl::Error> {
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
    parsed_input["spec"].to_string()
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn parse(spec: &str) -> String {
    let parsed_input = match parse_input(spec) {
      Ok(value) => value,
      Err(error) => panic!("{}", &error.to_string()),
    };
    parsed_input["spec"].to_string()
  }

  #[cfg(test)]
  mod test {
    mod helper;
    mod cases;
  }
}
