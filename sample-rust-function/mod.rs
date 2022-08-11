use std::collections::HashMap;
use serde_json::{Value, json};

pub fn perform(input: HashMap<String, Value>) -> Value {
    let numbers = input.get("numbers").unwrap();
    let numbers: &Vec<Value> = numbers.as_array().unwrap();

    let sum = numbers.iter().fold(0, |acc, x| acc + x.as_i64().unwrap());

    json!(sum)
}
