mod core;
mod commands;
mod consts;

use clap::{App, Arg, SubCommand};

extern crate serde_json;

#[macro_use]
extern crate serde_derive;

fn main() {
    let matches = App::new("gopm")
        .version(consts::VERSION)
        .author(consts::AUTHOR)
        .about("A Go project manager and template generator")
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new Go application with a custom file structure template")
                .arg(Arg::with_name("app_name").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize the file structure of the current directory"),
        )
        .subcommand(SubCommand::with_name("run").about("Build and run the Go application"))
        .subcommand(SubCommand::with_name("build").about("Build the Go application"))
        .subcommand(SubCommand::with_name("get-author").about("Get the saved author"))
        .subcommand(
            SubCommand::with_name("set-author")
                .about("Save the author to the configuration\n\tExample: set-author Your Name")
                .arg(Arg::with_name("author").required(true).index(1)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("new") => {
            let sub_matches = matches.subcommand_matches("new").unwrap();
            let app_name = sub_matches.value_of("app_name").unwrap();
            commands::app::create_new_go_app(app_name);
        }
        Some("init") => commands::init_go_app(),
        Some("run") => commands::build_app::build_and_run_go_app(),
        Some("build") => commands::build_app::build_go_app(),
        Some("get-author") => commands::user::print_username(),
        Some("set-author") => {
            let sub_matches = matches.subcommand_matches("set-author").unwrap();
            let author = sub_matches.value_of("author").unwrap();
            commands::user::save_author_to_config(author);
        }
        _ => core::utils::logo(),
    }
}