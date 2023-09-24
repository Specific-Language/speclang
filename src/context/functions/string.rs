use hcl::eval::FuncArgs;

pub fn length_func(args: FuncArgs) -> Result<hcl::Value, String> {
    if args.len() != 1 {
        return Err("Expected exactly one argument".to_string());
    }
    match &args[0] {
        hcl::Value::String(s) => Ok(hcl::Value::from(s.len())),
        _ => Err("Expected first argument to be a string".to_string()),
    }
}
