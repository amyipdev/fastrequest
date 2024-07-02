// SPDX-License-Identifier: AGPL-3.0-or-later

mod migrator;

use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Database::connect(concat!(
        "sqlite://",
        env!("CARGO_MANIFEST_DIR"),
        "/../../.entity-gen-migr.tmpdb?mode=rwc"
    ))
    .await?;
    migrator::Migrator::up(&conn, None).await?;
    Ok(())
}
