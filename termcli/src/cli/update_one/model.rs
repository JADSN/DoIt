// TODO: Error handling for Clap

// * IMPORTES INTERNOS
use shared::todo_update::TodoUpdate;

use clap::ArgMatches;

pub fn update_one(subcommand_param: Option<&ArgMatches>) -> Result<String, String> {
    match subcommand_param {
        Some(subcommand) => match subcommand.value_of("id") {
            Some(id) => match subcommand.value_of("description") {
                Some(description) => match subcommand.value_of("done") {
                    Some(done) => {
                        let data = TodoUpdate {
                            description: description.to_string(),
                            done: done.parse::<bool>().unwrap_or(false),
                        };

                        match server::database::update_one::presenter::update_one(
                            data,
                            id.parse::<u8>().unwrap_or(0),
                        ) {
                            Ok(_) => Ok(String::from("UPDATED")),
                            Err(error) => Err(error.to_string()),
                        }
                    }
                    None => Ok("Campo `id` não fornecido".to_string()),
                },
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
