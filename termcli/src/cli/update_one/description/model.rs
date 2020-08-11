// TODO: Error handling for Clap

// * IMPORTES INTERNOS
use shared::todo_update::Update;

use clap::ArgMatches;
use serde_json::Value;

const COLUMN_NAME: &str = "description";

pub fn update_one(subcommand_param: Option<&ArgMatches>) -> Result<String, String> {
    match subcommand_param {
        Some(subcommand) => match subcommand.value_of("id") {
            Some(id) => match subcommand.value_of(COLUMN_NAME) {
                Some(descrption) => {
                    match server::database::update_one::description::presenter::update(Update::new(
                        id.parse::<u8>().unwrap_or(0),
                        String::from(COLUMN_NAME),
                        Value::from(descrption),
                    )) {
                        Ok(_) => Ok(String::from("UPDATED")),
                        Err(error) => Err(error.to_string()),
                    }
                }
                None => Ok(String::from("Campo `id` não fornecido")),
            },
            None => Ok("Campo `description` não fornecido".to_string()),
        },
        None => {
            let error = format!(r#"Verficar {}:{}"#, file!(), line!());
            todo!("{:?}", error);
        }
    }
}
