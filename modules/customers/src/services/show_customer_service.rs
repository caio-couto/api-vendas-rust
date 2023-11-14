use std::{sync::Arc, ops::Deref};
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::{ DatabaseConnection, DbErr, EntityTrait, prelude::Uuid};
use serde::Deserialize;
use crate::entity::customers::{self, Entity, Model};
use axum::http::StatusCode;

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct ShowCustomerPath
{
    pub id: String, 
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct ShowCustomerService 
{
    connection: Arc<DatabaseConnection>,
}

impl ShowCustomerService 
{
    pub async fn execute(&self, show_customer_path: ShowCustomerPath) -> Result<Model, ApiError>
    {        
        let id = Uuid::parse_str(show_customer_path.id.as_str())
        .map_err(|_| ApiError { error_code: Errors::INVALID_UUID, status_code: StatusCode::BAD_REQUEST, custom_message: None })?;

        let customer: Option<customers::Model> = Entity::find_by_id(id)
        .one(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError { error_code: Errors::SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;
        
        match customer 
        {
            Some(p) => Ok(p),
            None => Err(ApiError { error_code: Errors::USER_NOT_FOUND, status_code: StatusCode::NOT_FOUND, custom_message: None })
        }
    }    
}
