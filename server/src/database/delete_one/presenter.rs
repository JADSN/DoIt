use rusqlite::{Connection, Result};

pub fn delete_one(id: u8) -> Result<()> {
    let conn = Connection::open("db.sqlite3")?;

    match super::model::delete_one(&conn, id) {
        Ok(data) => Ok(data),
        Err(error) => Err(error),
    }
}
