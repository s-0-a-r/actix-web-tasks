use sea_orm_migration::{prelude::*, schema::*};

use crate::extensions::Timestamp;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_auto(Column::Id))
                    .col(string(Column::Title))
                    .col(string(Column::Description))
                    .col(string(Column::Status))
                    .with_timestamps(true)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Task {
    #[sea_orm(iden = "tasks")]
    Table,
}

#[derive(DeriveIden)]
enum Column {
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "title")]
    Title,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "status")]
    Status,
}
