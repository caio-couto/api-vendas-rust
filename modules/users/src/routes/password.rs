use std::sync::Arc;

use axum::{ Router, routing::post };
use sea_orm::DatabaseConnection;

use crate::controllers::{ forgot_password_controller::ForgotPasswordController, reset_passord_controller::ResetPasswordController};

pub fn password_router() -> Router<Arc<DatabaseConnection>>
{
    let password_router = Router::new()
    .route("/forgot", post(ForgotPasswordController::create))
    .route("/reset", post(ResetPasswordController::create)); 


    password_router
}