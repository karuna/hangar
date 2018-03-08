use std::collections::HashMap;
use super::templates;

lazy_static! {
    pub static ref MAIN_TEMPLATES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("client/assets/.gitkeep", templates::blank::TEXT);
        m.insert("client/assets/application.css", templates::application_css::TEXT);
        m.insert("client/assets/application.js", templates::application_js::TEXT);
        m.insert("log/.gitkeep", templates::blank::TEXT);
        m.insert("migrations/.gitkeep", templates::blank::TEXT);
        m.insert("src/api/mod.rs", templates::blank::TEXT);
        m.insert("src/assets/favicon.svg", templates::favicon_svg::TEXT);
        m.insert("src/controllers/assets_controller.rs", templates::controller_assets::TEXT);
        m.insert("src/controllers/mod.rs", templates::controller_mod::TEXT);
        m.insert("src/controllers/pages_controller.rs", templates::controller_pages::TEXT);
        m.insert("src/controllers/users_controller.rs", templates::controller_users::TEXT);
        m.insert("src/guards/mod.rs", templates::guards_mod::TEXT);
        m.insert("src/guards/user.rs", templates::guards_user::TEXT);
        m.insert("src/guards/sanitized_str.rs", templates::guards_sanitized_str::TEXT);
        m.insert("src/libs/conn.rs", templates::libs_conn::TEXT);
        m.insert("src/libs/lib_const.rs", templates::libs_const::TEXT);
        m.insert("src/libs/email.rs", templates::libs_email::TEXT);
        m.insert("src/libs/init.rs", templates::libs_init::TEXT);
        m.insert("src/libs/log_format.rs", templates::libs_log_format::TEXT);
        m.insert("src/libs/logger.rs", templates::libs_logger::TEXT);
        m.insert("src/libs/mod.rs", templates::libs_mod::TEXT);
        m.insert("src/libs/routes.rs", templates::libs_routes::TEXT);
        m.insert("src/libs/settings.rs", templates::libs_settings::TEXT);
        m.insert("src/models/mod.rs", templates::model_mod::TEXT);
        m.insert("src/models/user.rs", templates::model_user::TEXT);
        m.insert("src/view_models/mod.rs", templates::view_models_mod::TEXT);
        m.insert("src/view_models/users.rs", templates::view_models_users::TEXT);
        m.insert("src/views/pages/404.html.tera", templates::html_404::TEXT);
        m.insert("src/views/pages/index.html.tera", templates::html_index::TEXT);
        m.insert("src/views/base_layout.html.tera", templates::html_base_layout::TEXT);
        m.insert("src/views/macros.html.tera", templates::html_macros::TEXT);
        m.insert("src/handlers.rs", templates::handlers::TEXT);
        m.insert("src/lib.rs", templates::lib::TEXT);
        m.insert("src/main.rs", templates::main::TEXT);
        m.insert("tests/lib.rs", templates::test_lib::TEXT);
        m.insert(".gitignore", templates::gitignore::TEXT);
        m.insert("README.md", templates::readme_md::TEXT);
        m.insert("rustfmt.toml", templates::rustfmt_toml::TEXT);
        m
    };

    pub static ref VAR_TEMPLATES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(".env", templates::dotenv::TEXT);
        m.insert("Cargo.toml", templates::cargo_toml::TEXT);
        m.insert("Rocket.toml", templates::rocket_toml::TEXT);
        m.insert("package.json", templates::package_json::TEXT);
        m.insert("src/libs/db.rs", templates::libs_db::TEXT);
        m
    };

    pub static ref SQLITE_TEMPLATES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("migrations/00000000000001_create_users/down.sql", templates::db_sqlite::DOWN);
        m.insert("migrations/00000000000001_create_users/up.sql", templates::db_sqlite::UP);
        m
    };

    pub static ref MYSQL_TEMPLATES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("migrations/00000000000001_create_users/down.sql", templates::db_mysql::DOWN);
        m.insert("migrations/00000000000001_create_users/up.sql", templates::db_mysql::UP);
        m
    };

    pub static ref POSTGRESQL_TEMPLATES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("migrations/00000000000000_diesel_initial_setup/down.sql", templates::db_postgresql::DOWN_00);
        m.insert("migrations/00000000000000_diesel_initial_setup/up.sql", templates::db_postgresql::UP_00);
        m.insert("migrations/00000000000001_create_users/down.sql", templates::db_postgresql::DOWN_01);
        m.insert("migrations/00000000000001_create_users/up.sql", templates::db_postgresql::UP_01);
        m
    };
}
