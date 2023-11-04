use ::products::services::create_products_service::{  CreateProductServiceBuilder, CreateProductDtoBuilder };
use sea_orm::prelude::Decimal;
use database::connection;

#[tokio::main]
async fn main() -> anyhow::Result<()>
{

    let create_products_service = CreateProductServiceBuilder::default()
    .connection(connection().await?)
    .build()
    .unwrap();

    let create_category_dto = CreateProductDtoBuilder::default()
    .name("Book 002".to_string())
    .price(Decimal::new(8000, 2))
    .quantity(9)
    .build()
    .unwrap();

    create_products_service.execute(create_category_dto).await?;

    Ok(())
}