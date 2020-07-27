use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TodoUpdate {
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "done")]
    pub done: bool,
}
