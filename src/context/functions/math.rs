use hcl::eval::FuncArgs;

pub fn sqrt_func(args: FuncArgs) -> Result<hcl::Value, String> {
    if args.len() != 1 {
        return Err("Expected exactly one argument".to_string());
    }
    match &args[0] {
        hcl::Value::Number(n) => Ok(hcl::Value::from(n.as_f64().unwrap().sqrt())),
        _ => Err("Expected first argument to be a number".to_string()),
    }
}

pub fn pow_func(args: FuncArgs) -> Result<hcl::Value, String> {
    if args.len() != 2 {
        return Err("Expected exactly two arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (hcl::Value::Number(n1), hcl::Value::Number(n2)) => {
            Ok(hcl::Value::from(n1.as_f64().unwrap().powf(n2.as_f64().unwrap())))
        }
        _ => Err("Expected both arguments to be numbers".to_string()),
    }
}
