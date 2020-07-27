use rusqlite::{params, Connection, Result};

pub fn delete_one(conn: &Connection, id: u8) -> Result<()> {
    let sql = format!(r#"DELETE FROM "todos" WHERE id IN (?1);"#);
    conn.execute(&sql, params![id])?;
    Ok(())
}
