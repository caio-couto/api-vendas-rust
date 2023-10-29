use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum Products 
{
    Table,
    Id,
    Name,    
    Price,
    Quantity,
    CreatedAt,
    UpdatedAt
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration 
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> 
    {
        let products_table = Table::create().table(Products::Table)
        .col
        (  
            ColumnDef::new(Products::Id)
            .uuid()
            .not_null()
            .primary_key()
            .extra("DEFAULT uuid_generate_v4()"),
        )
        .col
        (
            ColumnDef::new(Products::Name)
            .string()
            .not_null(),
        )
        .col
        (
            ColumnDef::new(Products::Price)
            .decimal_len(10, 2)
            .not_null(),
        )
        .col 
        (
            ColumnDef::new(Products::Quantity)
            .integer()
            .not_null(),
        )
        .col
        (
            ColumnDef::new(Products::CreatedAt)
            .timestamp_with_time_zone()
            .not_null()
            .extra("DEFAULT NOW()"),
        )
        .col
        (
            ColumnDef::new(Products::UpdatedAt)
            .timestamp_with_time_zone()
            .not_null()
            .extra("DEFAULT NOW()"),
        )
        .to_owned();

        manager.create_table(products_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> 
    {
        let category_table = Table::drop().table(Products::Table).to_owned();
        manager.drop_table(category_table).await
    }
}