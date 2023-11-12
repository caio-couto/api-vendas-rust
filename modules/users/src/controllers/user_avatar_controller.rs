use std::sync::Arc;
use axum::{response::IntoResponse, extract::{State, Multipart, multipart::MultipartError}, Extension, http::StatusCode};
use bytes::Bytes;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::DatabaseConnection;
use middlewares::is_authenticated::IsAuthenticatedMiddleware;

use crate::services::update_user_avatar_service::{UpdateUserAvatarServiceBuilder, UpdateUserAvatarDtoBuilder};


pub struct UserAvatarController {}
impl UserAvatarController
{
    pub async fn update(State(connection): State<Arc<DatabaseConnection>>, Extension(auth_guard): Extension<IsAuthenticatedMiddleware>, mut multipart: Multipart) -> impl IntoResponse
    {
        let mut name: String = String::new();
        let mut data: Result<Bytes, MultipartError> = Ok(Bytes::new());
        while let Some(field) = multipart.next_field().await.unwrap() 
        {
            name = field.file_name().unwrap().to_string();
            data = field.bytes().await;
        }

        let data = match data 
        {
            Ok(d) => d,
            Err(e) => return ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::BAD_REQUEST, custom_message: Some(e.to_string())}.into_response()
        };

        let update_user_avatar_service = UpdateUserAvatarServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let update_user_avatar_dto = UpdateUserAvatarDtoBuilder::default()
        .id(auth_guard.access_token_uuid)
        .data(data)
        .name(name)
        .build()
        .unwrap();

        let user = update_user_avatar_service.execute(update_user_avatar_dto).await;

        match user 
        {
            Ok(u) => serde_json::to_string(&u).unwrap().into_response(),
            Err(e) => e.into_response()    
        }
    }
}
