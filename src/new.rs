use std::path::Path;
use clap;
use generator::generate;

const INVALID_NAME: &'static str = "Invalid name";
const INVALID_DB: &'static str = "Invalid database";
const PATH_EXIST: &'static str = "Path already exist";
const DEFAULT_DB: &'static str = "postgresql";

pub struct Setting {
    pub name: String,
    pub db: String,
    pub db_url: String,
    pub author: String,
}

pub fn execute<'a>(matches: &clap::ArgMatches<'a>) -> Result<String, String> {
    let mut setting = Setting {
        name: String::from(""),
        db: String::from("postgresql"),
        db_url: String::from(""),
        author: String::from("John Doe <user@example.com>"),
    };

    let path_from_name = matches.value_of("name").unwrap();

    if let Some(err) = check_path(path_from_name) {
        return Err(err);
    }

    setting.name = String::from(path_from_name);

    let database_setup = configure_database(matches.value_of("database"));
    if database_setup.is_err() {
        return Err(database_setup.unwrap());
    }

    setting.db = database_setup.unwrap();

    let option_db_url = matches.value_of("database-url");

    if let Some(db_url) = option_db_url {
        setting.db_url = String::from(db_url);
    } else {
        setting.db_url = match setting.db.as_ref() {
            "mysql" => String::from("postgres://user:password@host/db_name"),
            "sqlite" => String::from("db.sqlite"),
            _ => String::from("postgres://user:password@host/db_name"),
        }
    }

    let option_author = matches.value_of("author");

    if let Some(author) = option_author {
        setting.author = String::from(author);
    }

    generate(&setting)
}

fn check_path(path_name: &str) -> Option<String> {
    let try_path = Path::new(path_name);
    // Check `/`
    if try_path.is_absolute() {
        return Some(String::from(INVALID_NAME));
    }
    // Check '..', '.', and "~"
    if path_name.starts_with(".") || path_name.starts_with("~") {
        return Some(String::from(INVALID_NAME));
    }
    // Check only 1 single name
    let components = try_path.components();
    if components.count() > 1 {
        return Some(String::from(INVALID_NAME));
    }
    // Check existing path in order to avoid overwrite
    if try_path.exists() {
        return Some(String::from(PATH_EXIST));
    }
    None
}

fn configure_database(db_setting_str: Option<&str>) -> Result<String, String> {
    let db_setting = db_setting_str.unwrap_or(DEFAULT_DB);
    return match db_setting {
        DEFAULT_DB => Ok(String::from(DEFAULT_DB)),
        "mysql" => Ok(String::from("mysql")),
        "sqlite" => Ok(String::from("sqlite")),
        _ => Err(String::from(INVALID_DB)),
    };
}
