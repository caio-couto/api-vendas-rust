use std::sync::Arc;

use axum::{response::IntoResponse, extract::State, Json};
use sea_orm::DatabaseConnection;

use crate::services::create_sessions_service::{CreateSessionsServiceBuilder, CreateSessionDto, CreateSessionDtoBuilder};

pub struct SessionsController;
impl SessionsController 
{
    pub async fn create(State(connection): State<Arc<DatabaseConnection>>, Json(create_sessions_dto): Json<CreateSessionDto>) -> impl IntoResponse
    {
        let create_sessions_service = CreateSessionsServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let create_sessions_dto = CreateSessionDtoBuilder::default()
        .email(create_sessions_dto.email)
        .password(create_sessions_dto.password)
        .build()
        .unwrap();

        let session = create_sessions_service.execute(create_sessions_dto).await;

        match session 
        {
            Ok(s) => serde_json::to_string(&s).unwrap().into_response(),
            Err(e) => e.into_response()    
        }
    }
}
