use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use derive_builder::Builder;
use sea_orm::{ DatabaseConnection, EntityTrait, prelude::Uuid, DeleteResult};
use serde::Deserialize;
use crate::entity::customers::Entity;
use errors::{api_erro::ApiError, erros::Errors};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct DeleteCustomerPath
{
    pub id: String, 
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct DeleteCustomerService 
{
    connection: Arc<DatabaseConnection>,
}

impl DeleteCustomerService 
{
    pub async fn execute(&self, delete_customer_path: DeleteCustomerPath) -> Result<DeleteResult, ApiError>
    {        
        let id = Uuid::parse_str(delete_customer_path.id.as_str())
        .map_err(|_| ApiError { error_code: Errors::INVALID_UUID, status_code: StatusCode::BAD_REQUEST, custom_message: None })?;

        let customer: DeleteResult = Entity::delete_by_id(id)
        .exec(self.connection.deref())
        .await
        .map_err(|_| ApiError { error_code: Errors::SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        if customer.rows_affected <= 0 
        {
            return Err(ApiError { error_code: Errors::USER_NOT_FOUND, status_code: StatusCode::NOT_FOUND, custom_message: None });
        }

        Ok(customer)
    }    
}
