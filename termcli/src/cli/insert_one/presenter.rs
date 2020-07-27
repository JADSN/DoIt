use clap::ArgMatches;

pub fn handler(subcommand_param: Option<&ArgMatches>) {
    match super::model::insert_one(subcommand_param) {
        Ok(model) => {
            let view = super::view::insert_one(model);
            view.print_stdout().unwrap();
        }
        Err(error) => {
            let view = super::view::insert_one(error);
            view.print_stdout().unwrap();
        }
    }
}
