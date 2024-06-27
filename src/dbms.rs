// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::utils::*;

use std::time::Duration;

use sea_orm::{ConnectOptions, DatabaseConnection, DbErr};
use sea_orm_rocket::rocket::figment::Figment;

#[derive(sea_orm_rocket::Database, Debug)]
#[database("fastrequest")]
pub struct Db(SeaOrmPool);

#[derive(Debug)]
pub struct SeaOrmPool {
    pub conn: DatabaseConnection
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = DbErr;
    type Connection = DatabaseConnection;
    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        // is this unwrap safe?
        let config = figment.extract::<sea_orm_rocket::Config>().unwrap();
        let mut options: ConnectOptions = config.url.into();
        // consider doing this Our Own Way(TM) because this is kinda awful due to not using Rocket.toml
        // (why do Rocket.toml and figments exist-)
        options
            .max_connections(config.max_connections as u32)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .sqlx_logging_level(log::LevelFilter::Trace);
        if let Some(idle_timeout) = config.idle_timeout {
            options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        Ok(SeaOrmPool { conn: sea_orm::Database::connect(options).await? })
    }
}

pub fn get_url(
    conf: &crate::config::Config,
    secrets: &crate::config::Secrets,
) -> String {
    let mut url = String::from(&conf.db.dbms);
    url.push_str("://");
    if conf.db.dbms != "sqlite" {
        url.push_str(&conf.db.username);
        url.push(':');
        url.push_str(
            &secrets
                .db
                .as_ref()
                .unwrap_or_else(|| erxit("no db secrets set"))
                .password,
        );
        url.push('@');
        url.push_str(&conf.db.address);
        url.push(':');
        url.push_str(&conf.db.port.to_string());
        url.push('/');
        url.push_str(&conf.db.database);
    } else {
        url.push_str(&conf.db.database);
        url.push_str("?mode=rwc");
    }
    debug!("determined db url to be {}", url);
    url
}
