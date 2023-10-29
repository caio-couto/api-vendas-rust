use std::sync::Arc;

use ::products::services::create_products_service::{  CreateProductServiceBuilder, CreateProductDtoBuilder };
use sea_orm::{ Database, prelude::Decimal };

#[tokio::main]
async fn main() -> anyhow::Result<()>
{
    dotenv::dotenv()?;

    let database_url: String = dotenv::var("DATABASE_URL")?;

    let connection = Database::connect(database_url).await?;

    let create_products_service = CreateProductServiceBuilder::default()
    .connection(Arc::new(connection))
    .build()
    .unwrap();

    let create_category_dto = CreateProductDtoBuilder::default()
    .name("Book 001".to_string())
    .price(Decimal::new(7300, 2))
    .quantity(5)
    .build()
    .unwrap();

    create_products_service.execute(create_category_dto).await?;

    Ok(())
}