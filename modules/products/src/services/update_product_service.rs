use std::{sync::Arc, ops::Deref};
use derive_builder::Builder;
use sea_orm::{ DatabaseConnection, prelude::{Decimal, Uuid} , ActiveValue::Set, DbErr, ActiveModelTrait, EntityTrait};
use serde::Deserialize;
use crate::entity::products;

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct UpdateProductPath
{
    pub id: String 
}
#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct UpdateProductDto 
{
    pub name: Option<String>, 
    pub price: Option<Decimal>, 
    pub quantity: Option<i32>,
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct UpdateProductService 
{
    connection: Arc<DatabaseConnection>,
}

impl UpdateProductService 
{
    pub async fn execute(&self, update_product_path: UpdateProductPath ,update_product_dto: UpdateProductDto) -> Result<products::Model, DbErr>
    {
        let id = Uuid::parse_str(update_product_path.id.as_str()).unwrap();

        let product: Option<products::Model> = products::Entity::find_by_id(id)
        .one(self.connection.deref()).await?; 
    
        let mut product: products::ActiveModel = product.unwrap().into();

        match update_product_dto.name  
        {
            Some(n) => product.name = Set(n),
            None => ()
        }

        match update_product_dto.price  
        {
            Some(n) => product.price = Set(n),
            None => ()
        }

        match update_product_dto.quantity  
        {
            Some(n) => product.quantity = Set(n),
            None => ()
        }

        product.updated_at = Set(chrono::Utc::now().fixed_offset());

        let product: products::Model = product.update(self.connection.deref()).await?;
   
        Ok(product)
    }    
}
