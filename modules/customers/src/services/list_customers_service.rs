use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use derive_builder::Builder;
use sea_orm::{ DatabaseConnection, DbErr, EntityTrait};
use crate::entity::customers::{self, Entity};
use errors::{api_erro::ApiError, erros::Errors};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct ListCustomersService 
{
    connection: Arc<DatabaseConnection>,
}

impl ListCustomersService 
{
    pub async fn execute(&self) -> Result<Vec<customers::Model>, ApiError>
    {        
        let customers: Vec<customers::Model> = Entity::find()
        .all(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError { error_code: Errors::SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        Ok(customers)
    }    
}
