pub fn handler() {
    match super::model::read_all() {
        Ok(model) => {
            let view = super::view::read_all_ok(model);
            view.print_stdout().unwrap();
        }
        Err(error) => println!("Error: {}", error),
    };
}
