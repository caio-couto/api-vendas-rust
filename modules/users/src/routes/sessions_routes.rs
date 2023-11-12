use std::sync::Arc;

use axum::{ Router, routing::post };
use sea_orm::DatabaseConnection;

use crate::controllers::sessions_controller::SessionsController;

pub fn user_routes() -> Router<Arc<DatabaseConnection>>
{
    let products_router = Router::new()
    .route("/", post(SessionsController::create)); 

    products_router
}
