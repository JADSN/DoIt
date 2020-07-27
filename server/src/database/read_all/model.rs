//* INTERNAL IMPORTS
use shared::{todo::Todo, todo_item::TodoItem};

use rusqlite::{Connection, Result, NO_PARAMS};

pub fn read_all(conn: &Connection) -> Result<Todo> {
    let mut stmt = conn.prepare("SELECT * FROM todos")?;
    let todo_item_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            description: row.get(1)?,
            done: row.get(2)?,
        })
    })?;

    let mut todo = Todo::empty();

    for todo_item in todo_item_iter {
        todo.todos.push(todo_item?);
    }

    Ok(todo)
}
