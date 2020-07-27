use clap::ArgMatches;

pub fn delete_one(subcommand_param: Option<&ArgMatches>) -> Result<String, String> {
    match subcommand_param {
        Some(subcommand) => match subcommand.value_of("id") {
            Some(data) => {
                let id = data.parse::<u8>().unwrap_or(0);
                match server::database::delete_one::presenter::delete_one(id) {
                    Ok(_) => Err(String::from("DELETED")),
                    Err(error) => Err(error.to_string()),
                }
            }
            None => Ok(String::from("NOTHING HAPPENS")),
        },
        None => {
            let error = format!(r#"Verficar {}:{}"#, file!(), line!());
            todo!("{:?}", error);
        }
    }
}
