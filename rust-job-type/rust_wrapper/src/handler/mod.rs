// This is just a stub for IDE.
// It gets replaced by user's Job code in job-template.Dockerfile
use std::collections::HashMap;
use serde_json::{Value, json};

pub fn perform(input: HashMap<String, Value>) -> Value {
    json!(input)
}
