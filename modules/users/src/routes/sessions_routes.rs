use std::sync::Arc;

use axum::{ Router, routing::post };
use sea_orm::DatabaseConnection;

use crate::controllers::sessions_controller::SessionsController;

pub fn sessions_routes() -> Router<Arc<DatabaseConnection>>
{
    let sessions_router = Router::new()
    .route("/", post(SessionsController::create)); 

    sessions_router
}
