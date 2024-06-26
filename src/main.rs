// SPDX-License-Identifier: AGPL-3.0-or-later

#[macro_use]
extern crate rocket;

mod config;
mod consts;
mod utils;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use rocket::{Build, Rocket};

#[launch]
fn rocket_main() -> Rocket<Build> {
    pretty_env_logger::init();

    let conf = config::load_config();

    // Set up database, verify credentials

    // CORS fairings (accept types, accurate clock)
    // Shared state for database
    rocket::build()
}
