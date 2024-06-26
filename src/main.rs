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
use rocket::{config::TlsConfig, data::ToByteUnit, fs::relative, Build, Rocket};

#[launch]
fn rocket_main() -> Rocket<Build> {
    pretty_env_logger::init();
    trace!("initialized logger");

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
            error!("could not find dist folder in source tree");
            erxit("did the svelte build run?");
        }
        p
    };
    debug!("located dist at {}", dist);

    let conf = config::load_config();

    // Set up database, verify credentials

    // CORS fairings (accept types, accurate clock)
    // Shared state for database
    rocket::build()
        .configure(rocket::Config {
            port: conf.settings.port,
            address: "::".parse::<std::net::IpAddr>().unwrap(),
            limits: rocket::data::Limits::new().limit("bytes", 32.kibibytes()),
            tls: Some(TlsConfig::from_paths(conf.ssl.cert, conf.ssl.key)),
            ..rocket::Config::release_default()
        })
}
