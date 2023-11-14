use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::{DatabaseConnection, EntityTrait, DbErr, prelude::Uuid};
use serde::Deserialize;
use crate::entity::users::{ Entity as Users, Model };

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct ShowProfilePath
{
    user_id: Uuid
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct ShowProfileService 
{
    connection: Arc<DatabaseConnection>,
}

impl ShowProfileService 
{
    pub async fn execute(&self, show_profile_path: ShowProfilePath) -> Result<Model, ApiError>
    {
        let user = Users::find_by_id(show_profile_path.user_id)
        .one(self.connection.deref())
        .await
        .map_err(|_:DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        let user = user.ok_or(ApiError { error_code: Errors::USER_NOT_FOUND, status_code: StatusCode::NOT_FOUND, custom_message: None})?;

        Ok(user)
    }    
}