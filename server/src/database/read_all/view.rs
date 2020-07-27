use shared::todo::Todo;

use rusqlite::{Error, Result};

pub fn read_all_ok(data: Todo) -> Result<Todo> {
    Ok(data)
}

pub fn read_all_err(error: Error) -> Result<String> {
    Ok(error)
}
