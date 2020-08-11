use cli_table::{
    format::{CellFormat, Justify},
    Cell, Row, Table,
};

pub fn update_one(status: String) -> Table {
    let mut table_rows = vec![];

    let bold = CellFormat::builder()
        .justify(Justify::Center)
        .bold(true)
        .build();

    let center = CellFormat::builder().justify(Justify::Center).build();

    let row_header = Row::new(vec![Cell::new("STATUS", bold)]);
    let row_count = Row::new(vec![Cell::new(&status.to_string(), center)]);

    table_rows.push(row_header);
    table_rows.push(row_count);

    Table::new(table_rows, Default::default()).unwrap()
}
