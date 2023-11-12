use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::{ DatabaseConnection, ActiveValue::{ NotSet, Set }, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, DbErr};
use serde::Deserialize;
use crate::entity::users::{ActiveModel, Entity as Users, self};
use bcrypt::{ hash, BcryptError};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct CreateUserDto 
{
    pub name: String, 
    pub email: String, 
    pub password: String,
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct CreateUserService 
{
    connection: Arc<DatabaseConnection>,
}

impl CreateUserService 
{
    pub async fn execute(&self, create_user_dto: CreateUserDto) -> Result<ActiveModel, ApiError>
    {
        let user = Users::find()
        .filter(users::Column::Email.eq(&create_user_dto.email))
        .one(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError {error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None})?;

        let password = hash(create_user_dto.password, 7)
        .map_err(|_: BcryptError| ApiError { error_code: Errors::INVALID_PASSWORD, status_code: StatusCode::BAD_REQUEST, custom_message: None })?; 

        match user 
        {
            Some(_) => return Err(ApiError { error_code: Errors::DATA_ALREDY_IN_USE, status_code: StatusCode::BAD_REQUEST, custom_message: Some("Email addres already used.".to_string()) }),
            None => ()
        }
            
        let new_user = ActiveModel
        {
            id: NotSet,
            name: Set(create_user_dto.name),
            email: Set(create_user_dto.email),
            password: Set(password),
            avatar: NotSet,
            created_at: NotSet,
            updated_at: NotSet
        };
        
        let new_user = new_user.save(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError {error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None})?;
        
        Ok(new_user)
    }    
}
