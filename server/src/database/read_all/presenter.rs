//* INTERNAL IMPORTS
use shared::todo::Todo;

use rusqlite::{Connection, Result};

pub fn read_all() -> Result<Todo> {
    let conn = Connection::open("db.sqlite3")?;

    match super::model::read_all(&conn) {
        Ok(data) => Ok(data),
        Err(error) => Err(error),
    }
}
