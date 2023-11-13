use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let res = manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .date()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .date()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                     )
                    .to_owned(),
            )
            .await;
        
        let insert = Query::insert()
            .into_table(User::Table)
            .columns([User::Email, User::Password])
            .values_panic(["test@test.com".into(), "test@test.com".into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        res
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}
