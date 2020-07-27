mod cli;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App as ClapApp, Arg, ArgMatches,
    SubCommand,
};

// * Atualiza a versão LOCALMENTE com base na escolha do Usuário através da CLI:
// * [server|all|insert_one|update_one|delete_one]
fn dispatcher(subcommand_param: (&str, Option<&ArgMatches>)) {
    match subcommand_param.0 {
        "server" => client::entrypoint().unwrap(),
        "all" => cli::read_all::presenter::handler(),
        "insert_one" => cli::insert_one::presenter::handler(subcommand_param.1),
        "update_one" => cli::update_one::presenter::handler(subcommand_param.1),
        "delete_one" => cli::delete_one::presenter::handler(subcommand_param.1),
        _ => std::process::exit(0),
    }
}

pub fn obtain_app() -> ArgMatches<'static> {
    let app = mount_cli();
    app
}

fn mount_cli() -> ArgMatches<'static> {
    ClapApp::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("doit").help("doit app").required(true))
        .subcommand(SubCommand::with_name("server").about("Start WebApp"))
        .subcommand(SubCommand::with_name("all").about("Read all itens in todos table"))
        .subcommand(
            SubCommand::with_name("insert_one")
                .about("Insert one item in todos tables")
                .arg(
                    Arg::with_name("description")
                        .short("d")
                        .min_values(1)
                        .max_values(1)
                        .value_name("STRING")
                        .required(true)
                        .help("Description of task"),
                )
                .arg(
                    Arg::with_name("done")
                        .short("o")
                        .min_values(1)
                        .max_values(1)
                        .value_name("BOOLEAN")
                        .required(true)
                        .help("Done of task"),
                ),
        )
        .subcommand(
            SubCommand::with_name("update_one")
                .about("Update one item in table todos")
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .min_values(1)
                        .max_values(1)
                        .value_name("INTEGER")
                        .required(true)
                        .help("Id of task"),
                )
                .arg(
                    Arg::with_name("description")
                        .short("d")
                        .min_values(1)
                        .max_values(1)
                        .value_name("STRING")
                        .required(true)
                        .help("Description of task"),
                )
                .arg(
                    Arg::with_name("done")
                        .short("o")
                        .min_values(1)
                        .max_values(1)
                        .value_name("BOOLEAN")
                        .required(true)
                        .help("Done of task"),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete_one")
                .about("Delete one item in table todos")
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .min_values(1)
                        .max_values(1)
                        .value_name("INTEGER")
                        .required(true)
                        .help("Id of task"),
                ),
        )
        .get_matches()
}

pub fn entrypoint() {
    let app = obtain_app();
    // TODO: Estudar tipagem: `let subcommand = app.subcommand.unwrap();`
    let subcommand = app.subcommand();
    dispatcher(subcommand);
}
