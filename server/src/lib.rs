//* INTERNAL MODULES
pub mod database;

use rusqlite::{Connection, Result, NO_PARAMS};

fn t0() -> Result<()> {
    let conn = Connection::open("db.sqlite3")?;

    let sql = format!(
        r#"CREATE TABLE IF NOT EXISTS "todos" (
	    "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    "description"	NAME NOT NULL,
	    "done"	BOOLEAN NOT NULL
        );"#
    );

    conn.execute(&sql, NO_PARAMS)?;

    Ok(())
}

pub fn entrypoint() -> Result<()> {
    t0()
}
