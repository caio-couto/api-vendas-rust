use std::sync::Arc;

use axum::Router;
use sea_orm::DatabaseConnection;

pub fn routes() -> Router<Arc<DatabaseConnection>>
{
    let routes = Router::new()
    .nest("/products", products::routes::products_routes::products_routes());

    routes
}