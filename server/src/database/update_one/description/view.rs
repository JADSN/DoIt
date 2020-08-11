use shared::todo::Todo;

use rusqlite::{Error, Result};

pub fn update_ok(data: Todo) -> Result<Todo> {
    Ok(data)
}

pub fn updatee_err(error: Error) -> Result<String> {
    Ok(error)
}
