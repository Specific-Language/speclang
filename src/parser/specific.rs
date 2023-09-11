use serde_json::{Value, Number};

#[derive(Debug, Clone, PartialEq)]
pub enum Specific {
    Number(f64),
    Bool(bool),
}

impl Specific {
    // fn add(&self, other: &Self) -> Self {
    //     match (self, other) {
    //         (Specific::Number(l), Specific::Number(r)) => Specific::Number(l + r),
    //         _ => panic!("Invalid operation for given value types"),
    //     }
    // }

    pub fn deserialize(value: &Value) -> Self {
        match value {
            Value::Number(n) => Specific::Number(n.as_f64().expect("Failed to convert to f64")),
            Value::Bool(b) => Specific::Bool(*b),
            _ => panic!("Unexpected serde_json value type"),
        }
    }

    pub fn serialize(&self) -> Value {
        match self {
            Specific::Number(n) => Value::Number(Number::from_f64(*n).expect("Failed to convert to serde_json::Number")),
            Specific::Bool(b) => Value::Bool(*b),
            _ => panic!("Unexpected self value"),
        }
    }
}
