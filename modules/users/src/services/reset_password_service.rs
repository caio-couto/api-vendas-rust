use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use bcrypt::{hash, BcryptError};
use chrono::Duration;
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Uuid;
use sea_orm::{ DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, DbErr, ActiveModelTrait};
use serde::Deserialize;
use crate::entity::users::{ Entity as Users, self };
use crate::entity::user_tokens::{ Entity as UserTokens, self };

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct ResetPasswordDto 
{
    pub token: String, 
    pub password: String,
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct ResetPasswordService 
{
    connection: Arc<DatabaseConnection>,
}

impl ResetPasswordService 
{
    pub async fn execute(&self, resert_password_dto: ResetPasswordDto) -> Result<(), ApiError>
    {
        let token = Uuid::parse_str(resert_password_dto.token.as_str())
        .map_err(|_| ApiError { error_code: Errors::INVALID_UUID, status_code: StatusCode::BAD_REQUEST, custom_message: None })?;

        let user_token = UserTokens::find()
        .filter(user_tokens::Column::Token.eq(token))
        .one(self.connection.deref())
        .await
        .map_err(|e: DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: Some(e.to_string()) })?;

        let user_token = user_token.ok_or(ApiError { error_code: Errors::USER_NOT_FOUND, status_code: StatusCode::NOT_FOUND, custom_message: Some("Token not found".to_string()) })?;

        let user = Users::find()
        .filter(users::Column::Id.eq(user_token.user_id))
        .one(self.connection.deref())
        .await
        .map_err(|e: DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: Some(e.to_string()) })?;

        let user = user.ok_or(ApiError { error_code: Errors::USER_NOT_FOUND, status_code: StatusCode::NOT_FOUND, custom_message: Some("User not found".to_string()) })?;

        let token_created_at = user_token.created_at;
        let compare_date = token_created_at.checked_add_signed(Duration::hours(2)).unwrap();

        if chrono::Utc::now().fixed_offset() > compare_date
        {
            return Err(ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::BAD_REQUEST, custom_message: Some("Token expirado".to_string()) });
        }

        let password = hash(resert_password_dto.password, 7)
        .map_err(|_: BcryptError| ApiError { error_code: Errors::INVALID_PASSWORD, status_code: StatusCode::BAD_REQUEST, custom_message: None })?;

        let mut user: users::ActiveModel = user.into();

        user.password = Set(password);

        let _user: users::Model = user.update(self.connection.deref())
        .await
        .map_err(|e: DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: Some(e.to_string()) })?;

        Ok(())
    }    
}
