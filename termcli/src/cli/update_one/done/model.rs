// TODO: Error handling for Clap

// * IMPORTES INTERNOS
use shared::todo_update::Update;

use clap::ArgMatches;
use serde_json::Value;

const COLUMN_NAME: &str = "done";

pub fn update_one(subcommand_param: Option<&ArgMatches>) -> Result<String, String> {
    match subcommand_param {
        Some(subcommand) => match subcommand.value_of("id") {
            Some(id) => match subcommand.value_of(COLUMN_NAME) {
                Some(done) => {
                    let done_parse = done.parse::<bool>().unwrap_or(false);

                    match server::database::update_one::done::presenter::update(Update::new(
                        id.parse::<u8>().unwrap_or(0),
                        String::from(COLUMN_NAME),
                        Value::from(done_parse),
                    )) {
                        Ok(_) => Ok(String::from("UPDATED")),
                        Err(error) => Err(error.to_string()),
                    }
                }
                None => Ok(String::from("Campo `id` não fornecido")),
            },
            None => Ok("Campo `done` não fornecido".to_string()),
        },
        None => {
            let error = format!(r#"Verficar {}:{}"#, file!(), line!());
            todo!("{:?}", error);
        }
    }
}
