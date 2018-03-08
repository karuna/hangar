pub static TEXT: &'static str = "use std::fs::File;
use std::io::Error;
use std::path::Path;

use slog::Logger;
use sloggers::Build;
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::file::FileLoggerBuilder;
use sloggers::types::Severity;

use rocket::config::Environment;

use super::settings::OurLogger;

pub struct CombinedLogger {
    pub terminal_logger: Option<Logger>,
    pub file_logger: Option<Logger>,
}

fn return_terminal_logger(log_level: &Severity) -> Logger {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(*log_level);
    builder.destination(Destination::Stdout);
    builder.build().unwrap()
}

fn return_file_logger(log_level: &Severity, path: &str) -> Result<Logger, Error> {
    let environment = Environment::active().expect(\"Unknown environment\");
    let filename = match environment {
        Environment::Development => \"development.log\",
        Environment::Staging => \"staging.log\",
        Environment::Production => \"production.log\",
    };
    let mut string_path = String::from(path);
    string_path.push_str(filename);
    if !Path::new(&string_path).exists() {
        File::create(&string_path)?;
    }
    let mut builder = FileLoggerBuilder::new(&string_path);
    builder.level(*log_level);
    Ok(builder.build().unwrap())
}

pub fn prepare_logger(our_logger: &OurLogger) -> CombinedLogger {
    let terminal_logger = if our_logger.terminal_logger {
        Some(return_terminal_logger(&our_logger.log_level))
    } else {
        None
    };
    let file_logger = if our_logger.file_logger {
        match return_file_logger(&our_logger.log_level, &our_logger.file_logger_dir_path) {
            Ok(logger) => Some(logger),
            Err(_) => None,
        }
    } else {
        None
    };
    CombinedLogger {
        terminal_logger,
        file_logger,
    }
}
";
