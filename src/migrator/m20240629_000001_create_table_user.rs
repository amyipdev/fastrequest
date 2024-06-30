// SPDX-License-Identifier: AGPL-3.0-or-later

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240629_000001_create_table_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::Username).text().not_null())
                    .col(ColumnDef::new(User::FirstName).text().not_null())
                    .col(ColumnDef::new(User::LastName).text().not_null())
                    .col(ColumnDef::new(User::Email).text().not_null().unique_key())
                    .col(ColumnDef::new(User::Phone).text())
                    .col(ColumnDef::new(User::Organization).text())
                    .col(ColumnDef::new(User::Salt).text().not_null())
                    .col(ColumnDef::new(User::HashedPassword).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    FirstName,
    LastName,
    Email,
    Phone,
    Organization,
    Salt,
    HashedPassword,
}
