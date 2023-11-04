use std::{sync::Arc, ops::Deref};
use derive_builder::Builder;
use sea_orm::{ DatabaseConnection, DbErr, EntityTrait, prelude::Uuid};
use serde::Deserialize;
use crate::entity::products::{self, Entity};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct ShowProductPath
{
    pub id: String, 
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct ShowProductService 
{
    connection: Arc<DatabaseConnection>,
}

impl ShowProductService 
{
    pub async fn execute(&self, show_product_path: ShowProductPath) -> Result<Option<products::Model>, DbErr>
    {        
        let id = Uuid::parse_str(show_product_path.id.as_str()).unwrap();
        let products: Option<products::Model> = Entity::find_by_id(id).one(self.connection.deref()).await?;

        Ok(products)
    }    
}
