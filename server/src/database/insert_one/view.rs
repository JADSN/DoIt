use shared::todo::Todo;

use rusqlite::{Error, Result};

pub fn insert_one_ok(data: Todo) -> Result<Todo> {
    Ok(data)
}

pub fn insert_one_err(error: Error) -> Result<String> {
    Ok(error)
}
