#[cfg(test)]
pub mod expect {
  use crate::speclang;
  use serde_json::*;

  pub fn success(input: &str, expected: Value) {
    let result = speclang::parse(input);
    assert_eq!(result, expected.to_string());
  }

  pub fn error(input: &str, expected: &str) {
    let result = std::panic::catch_unwind(|| speclang::parse(input));
    let caught_error = match result {
      Ok(_) => panic!("Expected a fatal error, but none occurred"),
      Err(error) => error
    };
    let message = caught_error.downcast_ref::<String>().unwrap();
    assert!(message.contains(expected));
  }
}
