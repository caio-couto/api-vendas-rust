use std::sync::Arc;

use axum::Router;
use sea_orm::DatabaseConnection;

pub fn routes() -> Router<Arc<DatabaseConnection>>
{
    let routes = Router::new()
    .nest("/products", products::routes::products_routes::products_routes())
    .nest("/users", users::routes::user_routes::user_routes())
    .nest("/sessions", users::routes::sessions_routes::user_routes());

    routes
}
