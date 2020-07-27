use shared::todo::Todo;

pub fn read_all() -> Result<Todo, String> {
    match server::database::read_all::presenter::read_all() {
        Ok(data) => Ok(data),
        Err(error) => Err(error.to_string()),
    }
}
