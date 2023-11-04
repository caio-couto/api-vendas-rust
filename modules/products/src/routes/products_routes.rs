use std::sync::Arc;

use axum::{ Router, routing::{post, get, delete, put}};
use sea_orm::DatabaseConnection;

use crate::controllers::products_controller::ProductsController;

pub fn products_routes() -> Router<Arc<DatabaseConnection>>
{
    let products_router = Router::new()
    .route("/", post(ProductsController::create))
    .route("/:id", get(ProductsController::show))
    .route("/", get(ProductsController::list))
    .route("/:id", delete(ProductsController::delete))
    .route("/:id", put(ProductsController::update));

    products_router
}
