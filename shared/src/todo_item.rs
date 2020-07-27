use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TodoItem {
    #[serde(rename = "id")]
    pub id: u8,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "done")]
    pub done: bool,
}
