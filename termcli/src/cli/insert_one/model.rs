// TODO: Error handling for Clap

// * IMPORTES INTERNOS
use shared::todo_insert::TodoInsert;

use clap::ArgMatches;

pub fn insert_one(subcommand_param: Option<&ArgMatches>) -> Result<String, String> {
    match subcommand_param {
        Some(subcommand) => match subcommand.value_of("description") {
            Some(description) => match subcommand.value_of("done") {
                Some(done) => {
                    let data = TodoInsert {
                        description: description.to_string(),
                        done: done.parse::<bool>().unwrap_or(false),
                    };
                    match server::database::insert_one::presenter::insert_one(data) {
                        Ok(_) => Ok(String::from("CREATED")),
                        Err(error) => Err(error.to_string()),
                    }
                }
                None => Ok("Campo `description` não fornecido".to_string()),
            },
            None => Ok("Campo `done` não fornecido".to_string()),
        },
        None => {
            let error = format!(r#"Verficar {}:{}"#, file!(), line!());
            todo!("{:?}", error);
        }
    }
}
