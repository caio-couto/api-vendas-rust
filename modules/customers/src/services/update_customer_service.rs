use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::{ DatabaseConnection, prelude::Uuid , ActiveValue::Set, DbErr, ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter};
use serde::Deserialize;
use crate::entity::customers::{self, Model, Entity as Customer};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct UpdateCustomerPath
{
    pub id: String 
}
#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct UpdateCustomerDto 
{
    pub name: Option<String>, 
    pub email: Option<String>, 
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct UpdateCustomerService 
{
    connection: Arc<DatabaseConnection>,
}

impl UpdateCustomerService 
{
    pub async fn execute(&self, update_customer_path: UpdateCustomerPath ,update_customer_dto: UpdateCustomerDto) -> Result<Model, ApiError>
    {
        let id = Uuid::parse_str(update_customer_path.id.as_str())
        .map_err(|_| ApiError { error_code: Errors::INVALID_UUID, status_code: StatusCode::BAD_REQUEST, custom_message: None })?;

        let customer: Option<customers::Model> = customers::Entity::find_by_id(id)
        .one(self.connection.deref())
        .await 
        .map_err(|_: DbErr| ApiError { error_code: Errors::SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;


        let customer = customer.ok_or(ApiError { error_code: Errors::USER_NOT_FOUND, status_code: StatusCode::NOT_FOUND, custom_message: None })?;
    
        let mut customer: customers::ActiveModel = customer.into();

        if let Some(e) = update_customer_dto.email
        {
            let customer_update_email = Customer::find()
            .filter(customers::Column::Email.eq(&e))
            .one(self.connection.deref())
            .await
            .map_err(|_:DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

            if let Some(u) = customer_update_email
            {
                if u.id != id
                {
                    return Err(ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::BAD_REQUEST, custom_message: None });
                }
            }

            customer.email = Set(e);
        }

        if let Some(n) = update_customer_dto.name
        {
            customer.name = Set(n);
        }

        customer.updated_at = Set(chrono::Utc::now().fixed_offset());

        let customer: customers::Model = customer.update(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError { error_code: Errors::SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;
   
        Ok(customer)
    }    
}
