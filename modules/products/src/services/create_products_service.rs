use std::{sync::Arc, ops::Deref};
use derive_builder::Builder;
use sea_orm::{ DatabaseConnection, prelude::Decimal, ActiveValue::{ NotSet, Set }, DbErr, ActiveModelTrait};
use serde::Deserialize;
use crate::entity::products::ActiveModel;

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct CreateProductDto 
{
    pub name: String, 
    pub price: Decimal, 
    pub quantity: i32,
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct CreateProductService 
{
    connection: Arc<DatabaseConnection>,
}

impl CreateProductService 
{
    pub async fn execute(&self, create_product_dto: CreateProductDto) -> Result<ActiveModel, DbErr>
    {
        let new_product = ActiveModel
        {
            id: NotSet,
            name: Set(create_product_dto.name),
            price: Set(create_product_dto.price),
            quantity: Set(create_product_dto.quantity),
            created_at: NotSet,
            updated_at: NotSet
        };
        
        let new_product = new_product.save(self.connection.deref()).await?;
        
        Ok(new_product)
    }    
}
