// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::utils::*;

use sea_orm::{Database, DatabaseConnection};

pub async fn new(conf: &crate::config::Config, secrets: &crate::config::Secrets) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let mut url = String::from(&conf.db.dbms);
    url.push_str("://");
    if conf.db.dbms != "sqlite" {
        url.push_str(&conf.db.username);
        url.push(':');
        url.push_str(&secrets.db.as_ref().unwrap_or_else(|| erxit("no db secrets set")).password);
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
    Ok(Database::connect(url).await?)
}
