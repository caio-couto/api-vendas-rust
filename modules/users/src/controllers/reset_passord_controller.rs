use std::sync::Arc;

use axum::{response::IntoResponse, extract::State, Json, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::services::reset_password_service::{ResetPasswordDto, ResetPasswordServiceBuilder, ResetPasswordDtoBuilder};

pub struct ResetPasswordController;
impl ResetPasswordController 
{
    pub async fn create(State(connection): State<Arc<DatabaseConnection>>, Json(resert_password_dto): Json<ResetPasswordDto>) -> impl IntoResponse
    {
        let send_forgot_password_email_service = ResetPasswordServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let resert_password_dto = ResetPasswordDtoBuilder::default()
        .token(resert_password_dto.token)
        .password(resert_password_dto.password)
        .build()
        .unwrap();

        let user = send_forgot_password_email_service.execute(resert_password_dto).await;

        match user
        {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => e.into_response()    
        }
    }
}