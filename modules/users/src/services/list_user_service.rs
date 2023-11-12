use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::{DatabaseConnection, EntityTrait, DbErr};
use crate::entity::users::{ Entity as Users, Model };

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct ListUserService 
{
    connection: Arc<DatabaseConnection>,
}

impl ListUserService 
{
    pub async fn execute(&self) -> Result<Vec<Model>, ApiError>
    {
        let users = Users::find()
        .all(self.connection.deref())
        .await
        .map_err(|_:DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        if users.len() <= 0
        {
            return  Err(ApiError { error_code: Errors::USER_NOT_FOUND, status_code: StatusCode::NOT_FOUND, custom_message: None});
        }

        Ok(users)
    }    
}