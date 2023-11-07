use std::sync::Arc;

use axum::{ Router, routing::post };
use sea_orm::DatabaseConnection;

use crate::controllers::user_controller::UserController;

pub fn user_routes() -> Router<Arc<DatabaseConnection>>
{
    let products_router = Router::new()
    .route("/", post(UserController::create)); 

    products_router
}
