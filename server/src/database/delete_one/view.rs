use shared::todo::Todo;

use rusqlite::{Error, Result};

pub fn delete_one_ok(data: Todo) -> Result<Todo> {
    Ok(data)
}

pub fn delete_one_err(error: Error) -> Result<String> {
    Ok(error)
}
