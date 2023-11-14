use std::sync::Arc;

use axum::{ Router, routing::{get, put}, middleware };
use sea_orm::DatabaseConnection;
use middlewares::is_authenticated::is_authenticated;

use crate::controllers::profile_controller::ProfileController;

pub fn profile_routes() -> Router<Arc<DatabaseConnection>>
{
    let profile_router = Router::new()
    .route("/", get(ProfileController::show))
    .route("/", put(ProfileController::update))
    .route_layer(middleware::from_fn(is_authenticated));

    profile_router
}
