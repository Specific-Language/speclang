pub mod context;
mod lexicon;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn define_str(name: &str, spec: &str) {
  let mut node = context::ContextNode::new();
  define(&mut node, name, spec);
  println!("{}", node);
}

pub fn define(node: &mut context::ContextNode, name: &str, spec: &str) {
  let spec_json = parse(spec);
  lexicon::define::spec(node, name, &spec_json);
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn parse_str(spec: &str) -> String {
  let output = parse(spec);
  serde_json::to_string_pretty(&output).unwrap()
}

pub fn parse(spec: &str) -> serde_json::Value {
  let input = format!("spec {{{}}}", spec);
  let result = hcl::from_str(&input);
  let parsed_input: serde_json::Value = match result {
    Ok(value) => value,
    Err(error) => panic!("Parse error! {}", &error.to_string()),
  };
  parsed_input["spec"].to_owned()
}

#[cfg(test)]
mod test {
  mod shared;
  mod define;
  mod parse;
}

// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen::prelude::wasm_bindgen]

/* next steps : */

// events
// need an "evaluate" event to get the current value? ex: cat-hungry
// eval ${} statements 
// or can this be baked into get()? getDefinition() versus getValue(context) 
// also need a "recognize" event to recognize an unknown value as some definition or partial
// combine define / parse fns here?
