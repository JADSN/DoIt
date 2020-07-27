//* INTERNAL IMPORTS
use shared::todo_insert::TodoInsert;

use rusqlite::{params, Connection, Result};

pub fn insert_one(conn: &Connection, data_to_insert: TodoInsert) -> Result<()> {
    let sql = format!(r#"INSERT INTO "todos"("description","done") VALUES (?1,?2);"#);

    conn.execute(
        &sql,
        params![data_to_insert.description, data_to_insert.done],
    )?;

    Ok(())
}
