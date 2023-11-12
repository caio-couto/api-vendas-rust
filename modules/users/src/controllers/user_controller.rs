use std::sync::Arc;

use axum::{response::IntoResponse, extract::State, Json};
use sea_orm::DatabaseConnection;

use crate::services::{create_user_service::{CreateUserDto, CreateUserServiceBuilder, CreateUserDtoBuilder}, list_user_service::ListUserServiceBuilder};

pub struct UserController {}
impl UserController 
{
    pub async fn list(State(connection): State<Arc<DatabaseConnection>>) -> impl IntoResponse
    {
        let list_user_service = ListUserServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let users = list_user_service.execute().await;

        match users 
        {
            Ok(u) => serde_json::to_string(&u).unwrap().into_response(),
            Err(e) => e.into_response()
        }
    }
    pub async fn create(State(connection): State<Arc<DatabaseConnection>>, Json(create_user_dto): Json<CreateUserDto>) -> impl IntoResponse
    {
        let create_user_service = CreateUserServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let create_user_dto = CreateUserDtoBuilder::default()
        .name(create_user_dto.name)
        .email(create_user_dto.email)
        .password(create_user_dto.password)
        .build()
        .unwrap();

        let user = create_user_service.execute(create_user_dto).await;

        match user 
        {
            Ok(_) => "Usuário Criado".into_response(),
            Err(e) => e.into_response()
        }        
    }
}
