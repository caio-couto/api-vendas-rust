use std::sync::Arc;

use axum::{ Router, routing::{post, get, patch}, middleware };
use sea_orm::DatabaseConnection;
use middlewares::is_authenticated::is_authenticated;

use crate::controllers::user_controller::UserController;
use crate::controllers::user_avatar_controller::UserAvatarController;

pub fn user_routes() -> Router<Arc<DatabaseConnection>>
{
    let user_router = Router::new()
    .route("/", get(UserController::list))
    .route("/avatar", patch(UserAvatarController::update))
    .route_layer(middleware::from_fn(is_authenticated))
    .route("/", post(UserController::create));

    user_router
}
