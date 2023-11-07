use std::sync::Arc;

use axum::{response::IntoResponse, extract::State, Json};
use sea_orm::DatabaseConnection;

use crate::services::create_user_service::{CreateUserDto, CreateUserServiceBuilder, CreateUserDtoBuilder};

pub struct UserController {}
impl UserController 
{
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

        let _user = create_user_service.execute(create_user_dto).await.unwrap();

        "Usuário Criado"
    }
}
