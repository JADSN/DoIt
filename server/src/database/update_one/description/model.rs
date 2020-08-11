//* INTERNAL IMPORTS
use shared::todo_update::Update;

use rusqlite::{params, Connection, Result};

pub fn update(conn: &Connection, data_to_update: Update) -> Result<()> {
    // let sql = format!(r#"UPDATE "todos" SET "done"=?1 WHERE id=?2;"#);
    let sql = format!(r#"UPDATE "todos" SET "description"=?1 WHERE id=?2;"#);

    // TODO: ERROR HANDLING
    let done_description = data_to_update.new_value.as_str().unwrap_or("EMPTY");

    conn.execute(&sql, params![done_description, data_to_update.id])?;

    Ok(())
}
