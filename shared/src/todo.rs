use serde::{Deserialize, Serialize};

use super::todo_item::TodoItem;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Todo {
    pub todos: Vec<TodoItem>,
}

impl Todo {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn set_items(&mut self, new_value: Vec<TodoItem>) {
        self.todos = new_value
    }
}
