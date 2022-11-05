use crate::m20220101_000001_create_table_keyring::Keyring;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Randomness::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Randomness::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Randomness::Network).big_integer().not_null())
                    .col(ColumnDef::new(Randomness::KeyringId).integer().not_null())
                    .col(ColumnDef::new(Randomness::Epoch).integer().not_null())
                    .col(ColumnDef::new(Randomness::Alpha).string().not_null())
                    .col(ColumnDef::new(Randomness::Gamma).string().not_null())
                    .col(ColumnDef::new(Randomness::C).string().not_null())
                    .col(ColumnDef::new(Randomness::S).string().not_null())
                    .col(ColumnDef::new(Randomness::Y).string().not_null())
                    .col(
                        ColumnDef::new(Randomness::CreatedDate)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string())
                            .not_null(),
                    )
                    .foreign_key(
                        &mut ForeignKeyCreateStatement::new()
                            .name("link_randomness_to_keyring")
                            .from_tbl(Randomness::Table)
                            .from_col(Randomness::KeyringId)
                            .to_tbl(Keyring::Table)
                            .to_col(Keyring::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Randomness::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Randomness {
    Table,
    Id,
    Network,
    KeyringId,
    Epoch,
    Alpha,
    Gamma,
    C,
    S,
    Y,
    CreatedDate,
}