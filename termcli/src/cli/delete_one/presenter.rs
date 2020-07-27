use clap::ArgMatches;

pub fn handler(subcommand_param: Option<&ArgMatches>) {
    match super::model::delete_one(subcommand_param) {
        Ok(model) => {
            let view = super::view::delete_one(model);
            view.print_stdout().unwrap();
        }
        Err(error) => {
            let view = super::view::delete_one(error);
            view.print_stdout().unwrap();
        }
    }
}
