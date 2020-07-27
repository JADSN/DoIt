use shared::todo::Todo;
use tide::Request;

pub async fn read_all(_req: Request<()>) -> Result<Todo, String> {
    match server::database::read_all::presenter::read_all() {
        Ok(data) => Ok(data),
        Err(error) => Err(error.to_string()),
    }

    // let mut todos: Vec<TodoItem> = vec![];

    // let item = TodoItem {
    //     id: 1,
    //     description: String::from("Task A"),
    //     done: false,
    // };
    // todos.push(item);

    // let todo: Todo = Todo { todos };

    // Ok(todo)
}
