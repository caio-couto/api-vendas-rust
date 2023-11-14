use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum Customers
{
    Table,
    Id,
    Name,
    Email,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> 
    {
        let customers_table = Table::create().table(Customers::Table)
        .col
        (
            ColumnDef::new(Customers::Id)
            .uuid()
            .primary_key()
            .not_null()
            .extra("DEFAULT uuid_generate_v4()"),
        )
        .col
        (
            ColumnDef::new(Customers::Name)
            .string()
            .not_null()
        )
        .col 
        (
            ColumnDef::new(Customers::Email)
            .string()
            .not_null()
        )
        .col
        (
            ColumnDef::new(Customers::CreatedAt)
            .timestamp_with_time_zone()
            .not_null()
            .default(Expr::current_timestamp()),
        )
        .col
        (
            ColumnDef::new(Customers::UpdatedAt)
            .timestamp_with_time_zone()
            .not_null()
            .default(Expr::current_timestamp()),
        )
        .to_owned();

        manager.create_table(customers_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> 
    {
        let customers_table = Table::drop().table(Customers::Table).to_owned();

        manager.drop_table(customers_table).await
    }
}

