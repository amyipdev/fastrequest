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
    pub db: DbConfig,
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
    pub port: u16,
    // Used for CORS
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DbConfig {
    pub dbms: String,
    pub username: String,
    pub database: String,
    pub address: String,
    pub port: u16,
}

// Loads and verifies configuration
impl Config {
    pub fn load_config() -> Self {
        // Get configuration path
        let local_target = [&uw_nofile(env::var("HOME")), ".config/fastrequest.toml"].join("/");
        let rel_target = relative!("fastrequest.toml");
        let env_target = env::var("FRQ_CONFIG_PATH");
        trace!(
            "checking {:?}, {}, {}, {}",
            env_target,
            ETC_CONFIG_TARGET,
            local_target,
            rel_target
        );
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
                erxits(format! {"declared file {} does not exist", ptr});
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

        config
    }
}

#[derive(Deserialize)]
pub struct Secrets {
    // option because sqlite doesn't need secrets
    pub db: Option<DbSecrets>,
}

#[derive(Deserialize)]
pub struct DbSecrets {
    pub password: String,
}

impl Secrets {
    pub fn new(conf: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        if conf.settings.use_env_secrets {
            erxit("use_env_secrets functionality is not available yet")
        } else if let Some(ref p) = conf.settings.secrets_file {
            Ok(toml::from_str(&std::fs::read_to_string(p)?)?)
        } else {
            erxit("specified secrets file does not exist!")
        }
    }
}
