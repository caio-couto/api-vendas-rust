use std::{sync::Arc, ops::Deref};
use derive_builder::Builder;
use sea_orm::{ DatabaseConnection, DbErr, EntityTrait, prelude::Uuid, DeleteResult};
use serde::Deserialize;
use crate::entity::products::Entity;

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct DeleteProductPath
{
    pub id: String, 
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct DeleteProductService 
{
    connection: Arc<DatabaseConnection>,
}

impl DeleteProductService 
{
    pub async fn execute(&self, delete_product_path: DeleteProductPath) -> Result<DeleteResult, DbErr>
    {        
        let id = Uuid::parse_str(delete_product_path.id.as_str()).unwrap();
        let products: DeleteResult = Entity::delete_by_id(id).exec(self.connection.deref()).await?;

        Ok(products)
    }    
}
