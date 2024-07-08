// SPDX-License-Identifier: AGPL-3.0-or-later

use sea_orm_migration::prelude::*;

use crate::migrator::m20240629_000001_create_table_user::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240708_000001_create_table_cookie"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cookie::Table)
                    .col(ColumnDef::new(Cookie::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Cookie::UserId).uuid().not_null())
                    .col(ColumnDef::new(Cookie::ExpiryDatetime).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_userid_cookie")
                            .from(Cookie::Table, Cookie::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned()
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
pub enum Cookie {
    Table,
    Id,
    UserId,
    ExpiryDatetime
}
