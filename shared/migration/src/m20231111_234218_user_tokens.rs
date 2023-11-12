use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum UserTokens {
    Table,
    Id,
    Token,
    UserId,
    CreatedAt,
    UpdatedAt
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> 
    {
        let user_tokens_table = Table::create().table(UserTokens::Table)
        .col
        (
            ColumnDef::new(UserTokens::Id)
            .uuid()
            .primary_key()
            .not_null()
            .extra("DEFAULT uuid_generate_v4()"),
        )
        .col
        (
            ColumnDef::new(UserTokens::Token)
            .uuid()
            .not_null()
            .extra("DEFAULT uuid_generate_v4()"),
        )
        .col
        (
            ColumnDef::new(UserTokens::UserId)
            .uuid()
            .not_null()
        )
        .col
        (
            ColumnDef::new(UserTokens::CreatedAt)
            .timestamp_with_time_zone()
            .not_null()
            .default(Expr::current_timestamp()),
        )
        .col
        (
            ColumnDef::new(UserTokens::UpdatedAt)
            .timestamp_with_time_zone()
            .not_null()
            .default(Expr::current_timestamp()),
        )
        .to_owned();

        manager.create_table(user_tokens_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> 
    {
        let user_tokens_table = Table::drop().table(UserTokens::Table).to_owned();

        manager.drop_table(user_tokens_table).await
    }
}