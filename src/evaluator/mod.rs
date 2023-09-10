// use serde_json::*;

// use crate::validator::expression;

// pub fn evaluate(value: Value, context: Map<String, Value>) -> Value {

// }

// pub fn get(name: &str, context: Map<String, Value>) -> &Value {
//     context.get(name).unwrap()
// }


// // Value::String(string) => {
// //     let expressions = expression::find(string);
// //     if !expressions.is_empty() {
// //         println!("\tExpressions: {:?}", expressions);
// //         expressions.iter().for_each(|expression| {
// //             expression::validate(expression, &context).unwrap();
// //         });
// //     }
// // },
// // Value::Number(..) => {},
// // Value::Bool(..) => {},
// // Value::Array(list) => {
// //     list.iter().for_each(|value| {
// //         validate(value, context).unwrap();
// //     });
// // },
// // Value::Object(map) => {
// //     for (key, value) in map.iter() {
// //         validate(value, context).unwrap();
// //         context.insert(key.to_string(), value.clone());
// //     }
// // },
// // _ => return Err(ValidationError::InvalidType),