// SPDX-License-Identifier: AGPL-3.0-or-later

#[macro_use]
extern crate rocket;

mod config;
mod consts;
mod dbms;
mod migrator;
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
use rocket_async_compression::CachedCompression;
use sea_orm_rocket::Database;

#[launch]
async fn rocket_main() -> Rocket<Build> {
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
    let secrets = config::Secrets::new(&conf).unwrap_or_else(|e| {
        debug!("{}", e);
        erxit("failed to load secrets")
    });
    trace!("secrets loaded");

    // NOTE: for the future, versions of FastRequest
    // PE = People's Edition, for sending requests to lots of agencies
    // GE = Government Edition, for agencies
    info!("FastRequest PE v{}", env!("CARGO_PKG_VERSION"));
    info!("Copyright (c) 2024 Open Information Collective, licensed under AGPLv3");

    let terminal_url_link: String = format!("https://[::]:{}", conf.settings.port);
    // CORS fairings (accept types, accurate clock)
    // Shared state for database
    info!(
        "Launching Rocket server on {}",
        terminal_link::Link::new(&terminal_url_link, &terminal_url_link)
    );
    info!("Using protocols HTTP3/udp, HTTP2/tcp, HTTP1.1/tcp");
    warn!("HTTP/3 support may throw benign errors; it is not yet stable");
    rocket::custom(
        Figment::from(rocket::Config::release_default())
            .merge(("tls", TlsConfig::from_paths(&conf.ssl.cert, &conf.ssl.key)))
            .merge(("port", conf.settings.port))
            .merge(("address", "::".parse::<std::net::IpAddr>().unwrap()))
            .merge(("databases.fastrequest.url", dbms::get_url(&conf, &secrets))),
    )
    .attach(dbms::Db::init())
    .attach(Shield::default().enable(Hsts::Preload(Duration::days(730))))
    .attach(CORS {
        url: conf.settings.url,
    })
    .attach(CachedCompression {
        cached_paths: vec!["".to_owned(), "/".to_owned(), "/index.html".to_owned()],
        cached_path_suffixes: vec![
            ".js".to_owned(),
            ".css".to_owned(),
            ".html".to_owned(),
            ".png".to_owned(),
            ".jpg".to_owned(),
            ".svg".to_owned(),
        ],
        excluded_path_prefixes: vec!["/api/".to_string()],
        ..Default::default()
    })
    // TODO: .register("/", catchers![not_found, ...])
    // TODO: .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
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
