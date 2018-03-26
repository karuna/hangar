extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate tera;

use std::process::exit;
use std::env;

use clap::{App, AppSettings, Arg, SubCommand};

mod build;
mod db;
mod help;
mod new;
mod run;
mod generator;
mod watch;

fn main() {
    let args = generate_args();

    let build_subcommand = generate_build_subcommand();
    let db_subcommand = generate_db_subcommand();
    let new_subcommand = generate_new_subcommand();
    let run_subcommand = generate_run_subcommand();

    let matches = App::new("hangar")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Karuna Murti <karuna.murti@gmail.com>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(build_subcommand)
        .subcommand(db_subcommand)
        .subcommand(new_subcommand)
        .subcommand(run_subcommand)
        .get_matches_from(&args);

    let result: Result<String, String> = if let Some(matches) = matches.subcommand_matches("build")
    {
        build::execute(&args)
    } else if let Some(matches) = matches.subcommand_matches("db") {
        db::execute(&args)
    } else if let Some(matches) = matches.subcommand_matches("new") {
        new::execute(&matches)
    } else if let Some(matches) = matches.subcommand_matches("run") {
        run::execute(&args)
    } else {
        return;
    };

    match result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Something went wrong! Reason: {:?}", e);
            exit(101);
        }
    }
}

fn generate_args() -> Vec<String> {
    let mut args = env::args();
    let mut filtered_args = Vec::new();
    filtered_args.push(args.next().unwrap());

    match args.next() {
        None => {}
        Some(arg) => filtered_args.push(arg),
    }

    filtered_args.extend(args);
    filtered_args
}

fn generate_build_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("build")
        .about("Wrapper for cargo build")
        .help(help::BUILD_HELP)
        .setting(AppSettings::AllowExternalSubcommands)
}

fn generate_db_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("db")
        .about("Wrapper for diesel-cli")
        .help(help::DB_HELP)
        .setting(AppSettings::AllowExternalSubcommands)
}

fn generate_new_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("new")
        .about("Generate a new Hangar Project")
        .arg(
            Arg::with_name("name")
                .long("name")
                .help("name for the Hangar Project")
                .value_name("NAME")
                .required(true)
        )
        .arg(
            Arg::with_name("database")
                .long("database")
                .help("Diesel database connection you want to use for the Hangar Project.{n}Valid value: postgresql, mysql, or sqlite.{n}Default: postgresql")
                .value_name("DB")
                .required(false)
        )
        .arg(
            Arg::with_name("database-url")
                .long("database-url")
                .help("Url for your database{n}postgresql: postgres://user:password@host/db_name{n}mysql: mysql://user:password@host/db_name{n}sqlite: path/to/db.sqlite")
                .value_name("DB_URL")
                .required(false)
        )
        .arg(
            Arg::with_name("author")
                .long("author")
                .help("Author of the project: `John Doe <user@example.com>`")
                .value_name("AUTHOR")
                .required(false)
        )
}

fn generate_run_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("run")
        .about("Wrapper for cargo run")
        .help(help::RUN_HELP)
        .setting(AppSettings::AllowExternalSubcommands)
}
