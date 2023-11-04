use std::{sync::Arc, ops::Deref};
use derive_builder::Builder;
use sea_orm::{ DatabaseConnection, DbErr, EntityTrait};
use crate::entity::products::{self, Entity};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct ListProductService 
{
    connection: Arc<DatabaseConnection>,
}

impl ListProductService 
{
    pub async fn execute(&self) -> Result<Vec<products::Model>, DbErr>
    {        
        let products: Vec<products::Model> = Entity::find().all(self.connection.deref()).await?;

        Ok(products)
    }    
}
