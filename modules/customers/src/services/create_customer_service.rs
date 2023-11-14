use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::{ DatabaseConnection, ActiveValue::{ NotSet, Set }, DbErr, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait};
use serde::Deserialize;
use crate::entity::customers::{ActiveModel, Entity as Customer, self};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct CreateCustomertDto 
{
    pub name: String, 
    pub email: String, 
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct CreateCustomerService 
{
    connection: Arc<DatabaseConnection>,
}

impl CreateCustomerService 
{
    pub async fn execute(&self, create_customer_dto: CreateCustomertDto) -> Result<ActiveModel, ApiError>
    {
        let customer = Customer::find()
        .filter(customers::Column::Email.eq(&create_customer_dto.email))
        .one(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError { error_code: Errors::SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        if let Some(_) = customer
        {
            return  Err(ApiError { error_code: Errors::DATA_ALREDY_IN_USE, status_code: StatusCode::BAD_REQUEST, custom_message: None });
        }

        let new_customer = ActiveModel
        {
            id: NotSet,
            name: Set(create_customer_dto.name),
            email: Set(create_customer_dto.email),
            created_at: NotSet,
            updated_at: NotSet
        };
        
        let new_customer = new_customer.save(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError { error_code: Errors::SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;
        
        Ok(new_customer)
    }    
}
