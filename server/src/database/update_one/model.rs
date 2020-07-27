//* INTERNAL IMPORTS
use shared::todo_update::TodoUpdate;

use rusqlite::{params, Connection, Result};

pub fn update_one(conn: &Connection, data_to_update: TodoUpdate, id: u8) -> Result<()> {
    let sql = format!(r#"UPDATE "todos" SET "description"=?1, "done"=?2 WHERE id=?3;"#);

    conn.execute(
        &sql,
        params![data_to_update.description, data_to_update.done, id],
    )?;

    Ok(())
}
