mod directories;
mod templates_list;
mod templates;

use std::io::prelude::*;
use std::fs::{create_dir, DirBuilder, File};
use std::path::{Path, PathBuf};
use std::collections::hash_map::Iter;
use std::process::Command;

use new::Setting;
use self::templates_list::*;
use self::directories::DIRECTORIES;
use tera::Tera;
use tera::Context;

pub fn generate(setting: &Setting) -> Result<String, String> {
    println!("Generating Hangar Project {}", setting.name);
    create_project_directories(setting);
    let success = create_main_templates(setting);
    if success.is_err() {
        return Err(String::from("Failed generating Hangar Project."));
    }

    let success = create_var_templates(setting);
    if success.is_err() {
        return Err(String::from("Failed generating Hangar Project."));
    }

    // Check commands (diesel & git & npm)
    // Diesel init
    // Git init
    // Npm init
    println!(
        "Finish generating new Hangar Project.
Go to application folder and edit database url on `.env` and `Rocket.toml`.
Run migration `hangar db setup` and print the schema `hangar db print-schema > src/schema.rs`.
Start the application with `cargo run`."
    );
    Ok(String::from("Success"))
}

fn create_project_directories(setting: &Setting) {
    let folder_path = Path::new(&setting.name);
    create_dir(&folder_path).unwrap();
    for dir in DIRECTORIES.iter() {
        let mut path = PathBuf::new();
        path.push(&setting.name);
        path.push(dir);
        println!("Generating directory {}", path.to_str().unwrap());
        DirBuilder::new().recursive(true).create(path).unwrap();
    }
    if setting.db.as_str() == "postgresql" {
        let mut path = PathBuf::new();
        path.push(&setting.name);
        path.push("migrations/00000000000000_diesel_initial_setup");
        println!("Generating directory {}", path.to_str().unwrap());
        DirBuilder::new().recursive(true).create(path).unwrap();
    }
}

fn create_main_templates(setting: &Setting) -> Result<String, String> {
    let main_templates = write_to_file(MAIN_TEMPLATES.iter(), setting);
    if main_templates.is_err() {
        return main_templates;
    }

    let db_templates = match setting.db.as_ref() {
        "mysql" => write_to_file(MYSQL_TEMPLATES.iter(), setting),
        "sqlite" => write_to_file(SQLITE_TEMPLATES.iter(), setting),
        _ => write_to_file(POSTGRESQL_TEMPLATES.iter(), setting),
    };

    if db_templates.is_err() {
        return db_templates;
    }

    Ok(String::from("Successfully rendered main template."))
}

fn write_to_file(
    iter: Iter<&'static str, &'static str>,
    setting: &Setting,
) -> Result<String, String> {
    for (main_template_name, main_template_value) in iter {
        let mut path = PathBuf::new();
        path.push(&setting.name);
        path.push(main_template_name);
        let file_path = path.as_path();
        let display = file_path.display();

        let mut file = match File::create(&file_path) {
            Err(why) => {
                println!("{:?}", why);
                return Err(String::from("Cannot create template."));
            }
            Ok(file) => file,
        };

        match file.write_all(main_template_value.as_bytes()) {
            Err(why) => {
                println!("{:?}", why);
                return Err(String::from("Cannot write template."));
            }
            Ok(_) => println!("Generating {}", display),
        }
    }
    Ok(String::from("Successfully rendered main template."))
}

fn create_var_templates(setting: &Setting) -> Result<String, String> {
    let parsed_context = generate_context(setting);
    if parsed_context.is_err() {
        return Err(String::from("Cannot render template."));
    }
    let context = parsed_context.unwrap();

    for (var_template_name, var_template_value) in VAR_TEMPLATES.iter() {
        let mut path = PathBuf::new();
        path.push(&setting.name);
        path.push(var_template_name);
        let file_path = path.as_path();
        let display = file_path.display();

        let mut tera = Tera::default();
        tera.add_raw_template(var_template_name, var_template_value);
        let rendered = tera.render(var_template_name, &context);

        if rendered.is_err() {
            return Err(String::from("Cannot render template."));
        }

        let mut file = match File::create(&file_path) {
            Err(why) => {
                println!("{:?}", why);
                return Err(String::from("Cannot create template."));
            }
            Ok(file) => file,
        };

        match file.write_all(rendered.unwrap().as_bytes()) {
            Err(why) => {
                println!("{:?}", why);
                return Err(String::from("Cannot write template."));
            }
            Ok(_) => println!("Generating {}", display),
        }
    }

    Ok(String::from("Successfully rendered var template."))
}

fn generate_context(setting: &Setting) -> Result<Context, String> {
    let mut context = Context::new();
    context.add("database_url", &setting.db_url);
    context.add("authors_name_email", &setting.author);
    let cargo_db = match setting.db.as_ref() {
        "mysql" => "diesel_mysql",
        "sqlite" => "diesel_sqlite",
        _ => "diesel_postgres",
    };
    let db_connection_long = match setting.db.as_ref() {
        "mysql" => "diesel::mysql::MysqlConnection",
        "sqlite" => "diesel::sqlite::SqliteConnection",
        _ => "diesel::pg::PgConnection",
    };
    let db_connection_short = match setting.db.as_ref() {
        "mysql" => "MysqlConnection",
        "sqlite" => "SqliteConnection",
        _ => "PgConnection",
    };
    context.add("cargo_db", cargo_db);
    context.add("db_connection_long", db_connection_long);
    context.add("db_connection_short", db_connection_short);
    Ok(context)
}
