//* INTERNAL IMPORTS
use shared::todo_update::Update;

use rusqlite::{params, Connection, Result};

pub fn update(conn: &Connection, data_to_update: Update) -> Result<()> {
    // let sql = format!(r#"UPDATE "todos" SET "done"=?1 WHERE id=?2;"#);
    let sql = format!(r#"UPDATE "todos" SET "done"=?1 WHERE id=?2;"#);

    // TODO: ERROR HANDLING
    let done_parse = data_to_update.new_value.as_bool().unwrap_or(false);

    conn.execute(&sql, params![done_parse, data_to_update.id])?;

    Ok(())
}
