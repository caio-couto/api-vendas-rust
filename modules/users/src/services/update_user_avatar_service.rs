use std::{sync::Arc, ops::Deref, io};
use axum::http::StatusCode;
use bytes::Bytes;
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::{ DatabaseConnection, prelude::Uuid, EntityTrait, DbErr, ActiveValue::Set, ActiveModelTrait};
use tokio::{fs::File, io::AsyncWriteExt};
use crate::entity::users::{ActiveModel, Entity as Users, Model};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct UpdateUserAvatarDto
{
    pub id: Uuid,
    pub data: Bytes,
    pub name: String
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct UpdateUserAvatarService 
{
    connection: Arc<DatabaseConnection>,
}

impl UpdateUserAvatarService 
{
    pub async fn execute(&self, update_user_avatar_dto: UpdateUserAvatarDto) -> Result<Model, ApiError>
    {
        let user = Users::find_by_id(update_user_avatar_dto.id)
        .one(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError { error_code: Errors::SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        let user = user.ok_or(ApiError { error_code: Errors::USER_NOT_FOUND, status_code: StatusCode::NOT_FOUND, custom_message: None })?;

        if user.avatar.is_some()
        {
            tokio::fs::remove_file(format!("./uploads/{}", user.avatar.clone().unwrap()))
            .await
            .map_err(|e: io::Error| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: Some(e.to_string())})?;
        }

        let name = format!("{}{}", Uuid::new_v4(), update_user_avatar_dto.name);

        if update_user_avatar_dto.data.is_empty()
        {
            return Err(ApiError { error_code: Errors::NOT_FOUND, status_code: StatusCode::BAD_REQUEST, custom_message: Some("Image not valid.".to_string())})
        }

        let mut file = File::create(format!("./uploads/{}", name))
        .await
        .map_err(|e: io::Error| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: Some(e.to_string())})?;

        file.write(&update_user_avatar_dto.data)
        .await
        .map_err(|e: io::Error| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: Some(e.to_string())})?;

        

        let mut user: ActiveModel = user.into();

        user.avatar = Set(Some(name));

        user.updated_at = Set(chrono::Utc::now().fixed_offset());

        let user: Model = user.update(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError { error_code: Errors::SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        Ok(user)
    }    
}
