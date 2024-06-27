// SPDX-License-Identifier: AGPL-3.0-or-later

#[macro_use]
extern crate rocket;

mod config;
mod consts;
mod dbms;
mod utils;

use utils::*;

use std::env;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use rocket::{
    config::TlsConfig,
    fairing::{Fairing, Info, Kind},
    figment::Figment,
    fs::{relative, FileServer},
    http::Header,
    response::content::RawHtml,
    shield::{Hsts, Shield},
    time::Duration,
    Build, Request, Response, Rocket, State,
};

#[launch]
fn rocket_main() -> Rocket<Build> {
    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            if cfg!(debug_assertions) {
                "info"
            } else {
                "warn"
            },
        );
    }
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

    let conf = config::Config::load_config();
    trace!("configuration loaded");

    // Set up database, verify credentials

    // NOTE: for the future, versions of FastRequest
    // PE = People's Edition, for sending requests to lots of agencies
    // GE = Government Edition, for agencies
    info!("FastRequest PE v{}", env!("CARGO_PKG_VERSION"));
    info!("Copyright (c) 2024 Open Information Collective, licensed under AGPLv3");

    // CORS fairings (accept types, accurate clock)
    // Shared state for database
    info!(
        "Launching Rocket server on https://[::]:{}/",
        conf.settings.port
    );
    info!("Using protocols HTTP3/udp, HTTP2/tcp, HTTP1.1/tcp");
    rocket::custom(
        Figment::from(rocket::Config::release_default())
            .merge(("tls", TlsConfig::from_paths(conf.ssl.cert, conf.ssl.key)))
            .merge(("port", conf.settings.port))
            .merge(("address", "::".parse::<std::net::IpAddr>().unwrap())),
    )
    .attach(Shield::default().enable(Hsts::Preload(Duration::days(730))))
    .attach(CORS {
        url: conf.settings.url,
    })
    .mount("/", FileServer::from(dist))
    .mount("/", routes![index])
    .manage(dist.to_string() as DistHolder)
}

struct CORS {
    url: Option<String>,
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS/COEP",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        if let Some(ref url) = self.url {
            response.set_header(Header::new("Access-Control-Allow-Origin", url.clone()));
        }
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PUT, DELETE, HEAD, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "false"));
        response.set_header(Header::new("Cross-Origin-Opener-Policy", "same-origin"));
        response.set_header(Header::new("Cross-Origin-Embedder-Policy", "require-corp"));
    }
}

type DistHolder = String;

#[get("/")]
fn index(dist: &State<DistHolder>) -> RawHtml<Option<String>> {
    RawHtml(std::fs::read_to_string([&*dist, "index.html"].join("/")).ok())
}
