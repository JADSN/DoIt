//* INTERNAL IMPORTS
use shared::todo_insert::TodoInsert;

use rusqlite::{Connection, Result};

pub fn insert_one(data_to_insert: TodoInsert) -> Result<()> {
    let conn = Connection::open("db.sqlite3")?;

    match super::model::insert_one(&conn, data_to_insert) {
        Ok(data) => Ok(data),
        Err(error) => Err(error),
    }
}
