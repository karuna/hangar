pub static TEXT: &'static str = "pub mod conn;
pub mod db;
pub mod init;
pub mod lib_const;
pub mod log_format;
pub mod logger;
pub mod routes;
pub mod settings;
#[cfg(feature = \"email\")]
pub mod email;";
