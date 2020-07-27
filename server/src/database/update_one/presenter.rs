//* INTERNAL IMPORTS
use shared::todo_update::TodoUpdate;

use rusqlite::{Connection, Result};

pub fn update_one(data_to_update: TodoUpdate, id: u8) -> Result<()> {
    let conn = Connection::open("db.sqlite3")?;

    match super::model::update_one(&conn, data_to_update, id) {
        Ok(data) => Ok(data),
        Err(error) => Err(error),
    }
}
