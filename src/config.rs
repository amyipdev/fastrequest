// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::consts::*;
use crate::utils::*;

use std::env;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use rocket::fs::relative;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ssl: SslConfig,
    pub settings: GeneralConfig,
}

#[derive(Serialize, Deserialize)]
pub struct SslConfig {
    pub cert: String,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    pub secrets_file: Option<String>,
    pub use_env_secrets: bool,
}

// Loads and verifies configuration
pub fn load_config() -> Config {
    // Get configuration path
    let local_target = [&uw_nofile(env::var("HOME")), ".config/fastrequest.toml"].join("/");
    let rel_target = relative!("fastrequest.toml");
    let env_target = env::var("FRQ_CONFIG_PATH");
    let conf_p = if let Ok(ref p) = env_target {
        if !pexi(p) {
            erxit("FRQ_CONFIG_PATH file does not exist");
        }
        p
    } else if pexi(&local_target) {
        &local_target
    } else if pexi(ETC_CONFIG_TARGET) {
        ETC_CONFIG_TARGET
    } else if pexi(rel_target) {
        rel_target
    } else {
        erxit("no configuration file found or specified");
    };

    // Load as TOML
    let mut config: Config = toml::from_str(
        &std::fs::read_to_string(conf_p)
            .unwrap_or_else(|_| erxit("unable to read configuration file")),
    )
    .unwrap_or_else(|_| erxit("configuration file does not match schema"));

    // Check legitimacy and replace relative paths
    let base = relative!("");
    // Run through all non-Option paths
    for ptr in [&mut config.ssl.cert, &mut config.ssl.key] {
        if ptr.chars().next() == Some('.') {
            *ptr = [base, ptr].join("");
        }
        if !pexi(ptr) {
            erxits(format!{"declared file {} does not exist", ptr});
        }
    }
    for optr in [&mut config.settings.secrets_file] {
        *optr = Some(
            [
                base,
                if let Some(v) = optr {
                    if v.chars().next() == Some('.') {
                        v
                    } else {
                        continue;
                    }
                } else {
                    continue;
                },
            ]
            .join(""),
        );
        if let Some(p) = optr {
            if !pexi(p) {
                erxits(format!("declared file {} does not exist", p));
            }
        }
    }

    // Verify credentials exist for all enabled modes

    config
}
