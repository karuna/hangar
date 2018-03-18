pub static TEXT: &'static str = "#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
#![recursion_limit = \"128\"]
// Clippy stuff
#![cfg_attr(feature = \"clippy\", feature(plugin))]
#![cfg_attr(feature = \"clippy\", plugin(clippy))]
// Stainless stuff
#![cfg_attr(test, plugin(stainless))]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate validator_derive;
extern crate validator;
extern crate argon2rs;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate ring;
extern crate uuid;
extern crate error_chain;
#[macro_use]
extern crate slog;
extern crate sloggers;
extern crate ammonia;
#[cfg(feature = \"multipart_form\")]
extern crate multipart;
#[cfg(feature = \"email\")]
extern crate email_format;
#[cfg(feature = \"email\")]
extern crate lettre;
#[cfg(feature = \"email\")]
extern crate native_tls;

pub mod apis;
pub mod controllers;
pub mod guards;
pub mod handlers;
pub mod libs;
pub mod models;
pub mod schema;
pub mod view_models;

use libs::init::init_rocket;
use handlers::init_catchers;

pub fn rocket_factory() -> Result<rocket::Rocket, String> {
    let initted_rocket = init_rocket(rocket::ignite());
    let catched_rocket = initted_rocket.catch(init_catchers());
    Ok(catched_rocket)
}
";
