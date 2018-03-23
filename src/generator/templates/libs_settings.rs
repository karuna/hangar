pub static TEXT: &'static str = "use std::collections::BTreeMap;

use chrono::Duration;
use sloggers::types::Severity;

use rocket::config::{Config, Value};

pub struct ApplicationSecurity {
    pub access_token_timeout_days: u32,
    pub password_salt: String,
}

pub struct Assets {
    pub assets_dir: String,
    pub assets_host: String,
    pub serve_assets: bool,
}

pub struct Database {
    pub database_url: String,
    pub database_pool: u32,
}

pub struct OurLogger {
    pub terminal_logger: bool,
    pub file_logger: bool,
    pub file_logger_dir_path: String,
    pub log_level: Severity,
}

pub struct Settings {
    pub application_security: ApplicationSecurity,
    pub assets: Assets,
    pub database: Database,
    pub our_logger: OurLogger,
}

impl ApplicationSecurity {
    pub fn access_token_duration(&self) -> Duration {
        Duration::days(i64::from(self.access_token_timeout_days))
    }
}

impl Settings {
    pub fn new_from_config(config: &Config) -> Self {
        let default_access_token_timeout_days = 30;
        let default_password_salt = \"somerandomsalt\";
        let default_assets_dir = \"src/assets\";
        let default_assets_host = \"localhost:8000\";
        let default_serve_assets = true;
        let default_database_url = \"\";
        let default_database_pool = 5;
        let default_terminal_logger = true;
        let default_file_logger = true;
        let default_file_logger_dir_path = \"log/\";
        let default_log_level = \"debug\";

        let access_token_timeout_days = config
            .get_int(\"access_token_timeout_days\")
            .unwrap_or(default_access_token_timeout_days)
            as u32;
        let password_salt = config
            .get_string(\"password_salt\")
            .unwrap_or_else(|_| default_password_salt.to_string());
        let assets_dir = config
            .get_string(\"assets_dir\")
            .unwrap_or_else(|_| default_assets_dir.to_string());
        let assets_host = config
            .get_string(\"assets_host\")
            .unwrap_or_else(|_| default_assets_host.to_string());
        let serve_assets = config
            .get_bool(\"serve_assets\")
            .unwrap_or(default_serve_assets);
        let database_url = config
            .get_string(\"database_url\")
            .unwrap_or_else(|_| default_database_url.to_string());
        let database_pool = config
            .get_int(\"database_pool\")
            .unwrap_or(default_database_pool) as u32;
        let terminal_logger = config
            .get_bool(\"terminal_logger\")
            .unwrap_or(default_terminal_logger);
        let file_logger = config
            .get_bool(\"file_logger\")
            .unwrap_or(default_file_logger);
        let file_logger_dir_path = config
            .get_string(\"file_logger_dir_path\")
            .unwrap_or_else(|_| default_file_logger_dir_path.to_string());
        let log_level = config.get_str(\"log_level\").unwrap_or(default_log_level);

        let application_security = ApplicationSecurity {
            access_token_timeout_days,
            password_salt,
        };

        let assets = Assets {
            assets_dir,
            assets_host,
            serve_assets,
        };

        let database = Database {
            database_url,
            database_pool,
        };

        let level = parse_log_level(log_level);

        let our_logger = OurLogger {
            terminal_logger,
            file_logger,
            file_logger_dir_path,
            log_level: level,
        };

        Settings {
            application_security,
            assets,
            database,
            our_logger,
        }
    }
}

pub fn unwrap_str<'a>(config: &'a BTreeMap<String, Value>, key: &'a str) -> &'a str {
    config.get(key).unwrap().as_str().unwrap()
}

pub fn unwrap_int(config: &BTreeMap<String, Value>, key: &str) -> i64 {
    config.get(key).unwrap().as_integer().unwrap()
}

// https://docs.rs/slog/2.1.1/slog/enum.Level.html
fn parse_log_level(log_level: &str) -> Severity {
    match log_level {
        \"Critical\" => Severity::Critical,
        \"Error\" => Severity::Error,
        \"Warning\" => Severity::Warning,
        \"Info\" => Severity::Info,
        \"Trace\" => Severity::Trace,
        _ => Severity::Debug,
    }
}
";
