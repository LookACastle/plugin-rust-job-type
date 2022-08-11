// This is just a stub for IDE.
// It gets replaced by user's Fatman code in fatman-template.Dockerfile
use std::collections::HashMap;
use serde_json::{Value, json};

pub fn perform(input: HashMap<String, Value>) -> Value {
    json!(input)
}
