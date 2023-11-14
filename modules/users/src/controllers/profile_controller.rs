use std::sync::Arc;

use axum::{response::IntoResponse, extract::State, Json, Extension};
use sea_orm::DatabaseConnection;
use middlewares::is_authenticated::IsAuthenticatedMiddleware;

use crate::services::{show_profile_service::{ShowProfileServiceBuilder, ShowProfilePathBuilder}, update_profile_service::{UpdateProfileServiceBuilder, UpdateProfileDtoBuilder, UpdateProfileDto, UpdateProfileAuthBuilder}};

pub struct ProfileController {}
impl ProfileController 
{
    pub async fn show(State(connection): State<Arc<DatabaseConnection>>, Extension(auth): Extension<IsAuthenticatedMiddleware>) -> impl IntoResponse
    {
        let show_profile_service = ShowProfileServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let show_profile_path = ShowProfilePathBuilder::default()
        .user_id(auth.access_token_uuid)
        .build()
        .unwrap();

        let user = show_profile_service.execute(show_profile_path).await;

        match user
        {
            Ok(u) => serde_json::to_string(&u).unwrap().into_response(),
            Err(e) => e.into_response()
        }
    }
    pub async fn update(State(connection): State<Arc<DatabaseConnection>>, Extension(auth): Extension<IsAuthenticatedMiddleware>, Json(update_profile_dto): Json<UpdateProfileDto>) -> impl IntoResponse
    {    
        let update_profile_service = UpdateProfileServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let update_profile_auth = UpdateProfileAuthBuilder::default()
        .user_id(auth.access_token_uuid)
        .build()
        .unwrap();

        let update_profile_path = UpdateProfileDtoBuilder::default()
        .name(update_profile_dto.name)
        .email(update_profile_dto.email)
        .password(update_profile_dto.password)
        .old_password(update_profile_dto.old_password)
        .build()
        .unwrap();

        let user = update_profile_service.execute(update_profile_path, update_profile_auth).await;

        match user
        {
            Ok(u) => serde_json::to_string(&u).unwrap().into_response(),
            Err(e) => e.into_response()
        }
    }
}