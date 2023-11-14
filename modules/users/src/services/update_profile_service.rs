use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::{DatabaseConnection, EntityTrait, DbErr, prelude::Uuid, QueryFilter, ColumnTrait, ActiveValue::Set, ActiveModelTrait};
use serde::Deserialize;
use crate::entity::users::{ Entity as Users, Model, self };
use bcrypt::{hash, verify, BcryptError};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct UpdateProfileAuth
{
    pub user_id: Uuid,
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct UpdateProfileDto
{
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub old_password: Option<String>
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct UpdateProfileService 
{
    connection: Arc<DatabaseConnection>,
}

impl UpdateProfileService 
{
    pub async fn execute(&self, update_profile_path: UpdateProfileDto, update_profile_auth: UpdateProfileAuth) -> Result<Model, ApiError>
    {
        let user_id = update_profile_auth.user_id.clone();

        let user = Users::find_by_id(user_id)
        .one(self.connection.deref())
        .await
        .map_err(|_:DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        let user = user.ok_or(ApiError { error_code: Errors::USER_NOT_FOUND, status_code: StatusCode::NOT_FOUND, custom_message: None})?;

        let user_password = user.password.clone();

        let mut update_user: users::ActiveModel = user.into();

        if let Some(e) = update_profile_path.email
        {
            let user_update_email = Users::find()
            .filter(users::Column::Email.eq(&e))
            .one(self.connection.deref())
            .await
            .map_err(|_:DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

            if let Some(u) = user_update_email
            {
                if u.id != update_profile_auth.user_id
                {
                    return Err(ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::BAD_REQUEST, custom_message: None });
                }
            }

            update_user.email = Set(e);
        }

        else if let Some(p) = &update_profile_path.password
        {
            if update_profile_path.password.is_some() && update_profile_path.old_password.is_none()
            {
                return Err(ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::BAD_REQUEST, custom_message: Some("Old password is required".to_string()) });
            }

            let check_old_password = verify(&p, &user_password.as_str())
            .map_err(|_: BcryptError| ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::BAD_REQUEST, custom_message: None })?;

            if !check_old_password
            {
                return Err(ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::BAD_REQUEST, custom_message: None });
            }

            let hased_new_password = hash(p, 7)
            .map_err(|_: BcryptError| ApiError { error_code: Errors::INVALID_PASSWORD, status_code: StatusCode::BAD_REQUEST, custom_message: None })?;

            update_user.password = Set(hased_new_password);
        }

        if let Some(n) = update_profile_path.name
        {
            update_user.name = Set(n);
        }

        update_user.updated_at = Set(chrono::Utc::now().fixed_offset());

        let user: Model = update_user.update(self.connection.deref())
        .await
        .map_err(|_:DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        Ok(user)
    }    
}