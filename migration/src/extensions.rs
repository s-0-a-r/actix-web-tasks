use sea_orm_migration::prelude::*;

pub trait Timestamp {
    fn with_timestamps(&mut self, deletable: bool) -> &mut Self;
}

impl Timestamp for TableCreateStatement {
    fn with_timestamps(&mut self, deletable: bool) -> &mut Self {
        self.col(
            ColumnDef::new(Column::CreatedAt)
                .timestamp_with_time_zone()
                .not_null()
                .default(Expr::current_timestamp()),
        )
        .col(
            ColumnDef::new(Column::UpdatedAt)
                .timestamp_with_time_zone()
                .not_null()
                .default(Expr::current_timestamp()),
        );

        if deletable {
            self.col(
                ColumnDef::new(Column::DeletedAt)
                    .timestamp_with_time_zone()
                    .null(),
            );
        }

        self
    }
}

#[derive(DeriveIden)]
enum Column {
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
    #[sea_orm(iden = "deleted_at")]
    DeletedAt,
}
