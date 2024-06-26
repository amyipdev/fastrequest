// SPDX-License-Identifier: AGPL-3.0-or-later

#[macro_use]
extern crate rocket;

mod config;
mod consts;
mod utils;

use utils::*;

use std::env;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use rocket::{Build, Rocket, fs::relative};

#[launch]
fn rocket_main() -> Rocket<Build> {
    pretty_env_logger::init();

    // NOTE: Nix builds will need to pass FRQ_BUILD_DIST
    // Find dist/ folder with Svelte compilation results
    let runtime_dist = env::var("FRQ_RUNTIME_DIST");
    let dist = if let Ok(ref p) = runtime_dist {
        if !pexi(p) {
            erxit("no dist folder at FRQ_RUNTIME_DIST");
        }
        p
    } else if let Some(p) = option_env!("FRQ_BUILD_DIST") {
        if !pexi(p) {
            erxit("no dist folder at compile-time FRQ_BUILD_DIST");
        }
        p
    } else {
        let p = relative!("dist");
        if !pexi(p) {
            erxit("could not find dist folder in source tree\ndid the svelte build run?");
        }
        p
    };

    let conf = config::load_config();

    // Set up database, verify credentials

    // CORS fairings (accept types, accurate clock)
    // Shared state for database
    rocket::build()
}
