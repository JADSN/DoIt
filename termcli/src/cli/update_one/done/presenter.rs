use clap::ArgMatches;

pub fn handler(subcommand_param: Option<&ArgMatches>) {
    match super::model::update_one(subcommand_param) {
        Ok(model) => {
            let view = super::view::update_one(model);
            view.print_stdout().unwrap();
        }
        Err(error) => {
            let view = super::view::update_one(error);
            view.print_stdout().unwrap();
        }
    }
}
