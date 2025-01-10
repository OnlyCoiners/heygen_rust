use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Payload {
    test: bool,
    example: String,
    value: i32,
}

impl Payload {
    pub fn new(test: bool, example: String, value: i32) -> Self {
        Self {
            test,
            example,
            value,
        }
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
