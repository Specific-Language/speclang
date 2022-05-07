use serde_json::Value;

fn main() {
    let input = r#"
      some_block "label_1" {
        attr = "value"
      }

      some_block "label_2" {
        attr = "value"
      }
    "#;

    let value: Value = hcl::from_str(input).unwrap();

    println!("{}", value)
}
