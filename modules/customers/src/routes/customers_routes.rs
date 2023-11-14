use std::sync::Arc;

use axum::{ Router, routing::{post, get, delete, put}, middleware};
use sea_orm::DatabaseConnection;
use middlewares::is_authenticated::is_authenticated;

use crate::controllers::customer_controller::CustomersController;

pub fn customers_routes() -> Router<Arc<DatabaseConnection>>
{
    let customerss_router = Router::new()
    .route("/", post(CustomersController::create))
    .route("/:id", get(CustomersController::show))
    .route("/", get(CustomersController::list))
    .route("/:id", delete(CustomersController::delete))
    .route("/:id", put(CustomersController::update))
    .route_layer(middleware::from_fn(is_authenticated));

    customerss_router
}