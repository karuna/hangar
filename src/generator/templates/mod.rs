pub mod application_css;
pub mod application_js;
pub mod blank;
pub mod dotenv;
pub mod favicon_svg;
pub mod gitignore;

pub mod controller_assets;
pub mod controller_mod;
pub mod controller_pages;
pub mod controller_users;
pub mod guards_mod;
pub mod guards_sanitized_str;
pub mod guards_user;
pub mod libs_conn;
pub mod libs_const;
pub mod libs_db;
pub mod libs_email;
pub mod libs_init;
pub mod libs_log_format;
pub mod libs_logger;
pub mod libs_mod;
pub mod libs_routes;
pub mod libs_settings;
pub mod model_mod;
pub mod model_user;
pub mod view_models_mod;
pub mod view_models_users;
pub mod html_404;
pub mod html_base_layout;
pub mod html_index;
pub mod html_macros;

pub mod handlers;
pub mod lib;
pub mod main;

pub mod test_lib;

pub mod cargo_toml;
pub mod package_json;
pub mod readme_md;
pub mod rocket_toml;
pub mod rustfmt_toml;

pub mod db_sqlite;
pub mod db_mysql;
pub mod db_postgresql;
