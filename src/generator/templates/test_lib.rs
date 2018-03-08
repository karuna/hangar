pub static TEXT: &'static str = "#![feature(plugin, const_fn)]
#![feature(core_intrinsics)]
#![plugin(stainless)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate diesel;
extern crate parking_lot;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate chrono;

extern crate hangar;

mod libs;";
