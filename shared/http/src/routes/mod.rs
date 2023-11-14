use std::sync::Arc;

use axum::Router;
use sea_orm::DatabaseConnection;

pub fn routes() -> Router<Arc<DatabaseConnection>>
{
    let routes = Router::new()
    .nest("/products", products::routes::products_routes::products_routes())
    .nest("/users", users::routes::user_routes::user_routes())
    .nest("/sessions", users::routes::sessions_routes::sessions_routes())
    .nest("/password", users::routes::password::password_router())
    .nest("/profile",   users::routes::profile::profile_routes())
    .nest("/customers", customers::routes::customers_routes::customers_routes());

    routes
}
