use std::{sync::Arc, ops::Deref};
use derive_builder::Builder;
use sea_orm::{ DatabaseConnection, prelude::Decimal, ActiveValue::{ NotSet, Set }, ActiveModelTrait, DbErr };

use crate::entity::products;

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct CreateProductDto 
{
    name: String, 
    price: Decimal, 
    quantity: i32,
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct CreateProductService 
{
    connection: Arc<DatabaseConnection>,
}

impl CreateProductService 
{
    pub async fn execute(&self, create_category_dto: CreateProductDto) -> Result<(), DbErr>
    {
        let _new_product = products::ActiveModel
        {
            id: NotSet,
            name: Set(create_category_dto.name),
            price: Set(create_category_dto.price),
            quantity: Set(create_category_dto.quantity),
            created_at: NotSet,
            updated_at: NotSet
        }.save(self.connection.deref()).await?;
        
        Ok(())
    }    
}