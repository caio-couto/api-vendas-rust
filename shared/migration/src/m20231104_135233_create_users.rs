use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum Users
{
    Table,
    Id,
    Name,
    Email,
    Password,
    Avatar,
    CreatedAt,
    UpdatedAt,
}
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration 
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> 
    {
        let users_table = Table::create().table(Users::Table)
        .col
        (
            ColumnDef::new(Users::Id)
            .uuid()
            .primary_key()
            .not_null()
            .extra("DEFAULT uuid_generate_v4()"),
        )
        .col
        (
            ColumnDef::new(Users::Name)
            .string()
            .not_null()
        )
        .col 
        (
            ColumnDef::new(Users::Email)
            .string()
            .not_null()
            .unique_key()
        )
        .col
        (
            ColumnDef::new(Users::Password)
            .string()
            .not_null()
        ).
        col
        (
            ColumnDef::new(Users::Avatar)
            .string()
        )
        .col
        (
            ColumnDef::new(Users::CreatedAt)
            .timestamp_with_time_zone()
            .not_null()
            .extra("DEFAULT NOW()"),
        )
        .col
        (
            ColumnDef::new(Users::UpdatedAt)
            .timestamp_with_time_zone()
            .not_null()
            .extra("DEFAULT NOW()"),
        )
        .to_owned();

        manager.create_table(users_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> 
    {
        let users_table = Table::drop().table(Users::Table).to_owned();

        manager.drop_table(users_table).await
    }
}

