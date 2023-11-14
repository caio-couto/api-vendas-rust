use std::sync::Arc;

use axum::{response::IntoResponse, extract::State, Json, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::services::send_forgot_password_email_service::{SendForgotPasswordEmailDto, SendForgotPasswordEmailServiceBuilder, SendForgotPasswordEmailDtoBuilder};

pub struct ForgotPasswordController;
impl ForgotPasswordController 
{
    pub async fn create(State(connection): State<Arc<DatabaseConnection>>, Json(send_forgot_password_email_dto): Json<SendForgotPasswordEmailDto>) -> impl IntoResponse
    {
        let send_forgot_password_email_service = SendForgotPasswordEmailServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let send_forgot_password_email_dto = SendForgotPasswordEmailDtoBuilder::default()
        .email(send_forgot_password_email_dto.email)
        .build()
        .unwrap();

        let user_token = send_forgot_password_email_service.execute(send_forgot_password_email_dto).await;

        match user_token 
        {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => e.into_response()    
        }
    }
}
