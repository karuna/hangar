pub static TEXT: &'static str = "use r2d2_diesel::ConnectionManager;

use rocket::fairing::AdHoc;
use rocket::{Config, Rocket, State};
use super::conn::Conn;
use rocket_contrib::Template;

use super::db::DbConnection;
#[cfg(feature = \"email\")]
use super::email::MailerConfig;
use super::logger::{prepare_logger, CombinedLogger};
use super::log_format::Common;
use super::routes::routes;
use super::settings::Settings;

use controllers::assets_controller;

pub fn init_rocket(rocket: Rocket) -> Rocket {
    let attached_rocket = rocket
        .attach(AdHoc::on_request(|req, _| {
            let new_uri = req.uri().as_str().to_lowercase();
            req.set_uri(new_uri);
        }))
        .attach(AdHoc::on_attach(|rocket| {
            let generated_rocket = setup_state(rocket);
            Ok(generated_rocket)
        }))
        .attach(Template::fairing())
        .attach(AdHoc::on_response(|req, res| {
            let logger = req.guard::<State<CombinedLogger>>().unwrap();
            let log = Common::log(req, res);
            if logger.terminal_logger.is_some() {
                let terminal_logger = logger.terminal_logger.as_ref().unwrap();
                info!(terminal_logger, \"{}\", log);
            }
            if logger.file_logger.is_some() {
                let file_logger = logger.file_logger.as_ref().unwrap();
                info!(file_logger, \"{}\", log);
            }
        }));
    routes(attached_rocket)
}

/// put your state and middleware here
fn setup_state(rocket: Rocket) -> Rocket {
    let local_config = rocket.config().clone();
    let local_settings = Settings::new_from_config(&local_config);

    // Database
    let database_url = local_settings.database.database_url.clone();
    let database_pool = local_settings.database.database_pool;
    let database_manager = ConnectionManager::<DbConnection>::new(database_url);
    let diesel_pool = Conn::init_pool(database_manager, database_pool).unwrap();
    // Assets
    let manage_asset = local_settings.assets.serve_assets;
    // Logger
    let combined_logger = prepare_logger(&local_settings.our_logger);

    let managed_rocket = rocket.manage(diesel_pool).manage(local_settings);
    let assetable_rocket = get_assetable_rocket(managed_rocket, manage_asset);
    let loggable_rocket = assetable_rocket.manage(combined_logger);

    if cfg!(feature = \"email\") {
        let rocket_with_email = match set_mailer_config(&local_config) {
            Ok(mailer) => loggable_rocket.manage(mailer),
            Err(_) => loggable_rocket,
        };

        rocket_with_email
    } else {
        loggable_rocket
    }
}

fn get_assetable_rocket(managed_rocket: Rocket, manage_asset: bool) -> Rocket {
    if manage_asset {
        managed_rocket.mount(\"/assets\", routes![assets_controller::files])
    } else {
        managed_rocket
    }
}

#[cfg(feature = \"email\")]
fn set_mailer_config(config: &Config) -> Result<MailerConfig, String> {
    match config.get_table(\"mailer\") {
        Err(_) => Err(String::from(\"No mailer configuration\")),
        Ok(mailer_config) => Ok(MailerConfig(mailer_config.to_owned())),
    }
}
";
