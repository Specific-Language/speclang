#[cfg(test)]
pub mod expect {
  use crate::parse;

  pub fn error(input: &str) -> String {
    let result = std::panic::catch_unwind(|| parse(input));
    let caught_error = match result {
      Ok(_) => panic!("Expected a fatal error, but none occurred"),
      Err(error) => error
    };
    caught_error.downcast_ref::<String>().unwrap().to_owned()
  }
}
