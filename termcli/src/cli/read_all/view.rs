use cli_table::{
    format::{CellFormat, Justify},
    Cell, Row, Table,
};

use shared::todo::Todo;

// TODO: Make a `convert` or `utils` lib for doit
fn conv_bool_to_str(input: bool, value_true: &str, value_false: &str) -> String {
    if input {
        value_true.to_string()
    } else {
        value_false.to_string()
    }
}

pub fn read_all_ok(model: Todo) -> Table {
    let mut table_rows = vec![];

    let bold = CellFormat::builder()
        .justify(Justify::Center)
        .bold(true)
        .build();

    let center = CellFormat::builder().justify(Justify::Center).build();

    let right = CellFormat::builder()
        .justify(Justify::Right)
        .bold(true)
        .build();

    let row_header = Row::new(vec![
        Cell::new("ID", bold),
        Cell::new("Description", bold),
        Cell::new("Done", bold),
    ]);

    table_rows.push(row_header);
    for todo in model.todos {
        let todo_done_string = conv_bool_to_str(todo.done, "DONE", "UNDONE");

        let row = Row::new(vec![
            Cell::new(&todo.id, right),
            Cell::new(&todo.description, Default::default()),
            Cell::new(&todo_done_string, center),
        ]);

        table_rows.push(row);
    }

    Table::new(table_rows, Default::default()).unwrap()
}
