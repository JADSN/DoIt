//* INTERNAL IMPORTS
// use serde;
use shared::todo_update::Update;

use rusqlite::{Connection, Result};

pub fn update(data_to_update: Update) -> Result<()> {
    let conn = Connection::open("db.sqlite3")?;

    match super::model::update(&conn, data_to_update) {
        Ok(data) => Ok(data),
        Err(error) => Err(error),
    }
}
