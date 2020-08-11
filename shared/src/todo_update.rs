use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TodoUpdate {
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "done")]
    pub done: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Update {
    pub id: u8,
    pub col_name: String,
    pub new_value: Value,
}

impl Update {
    pub fn new(id: u8, col_name: String, new_value: Value) -> Self {
        Self {
            id,
            col_name,
            new_value,
        }
    }
}
