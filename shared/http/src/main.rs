#[warn(unused_variables)]
#[warn(unused_imports)]
use axum::{ Router, Server };
use std::{net::SocketAddr, sync::Arc};
use database::connection;
use tower_http::services::ServeDir;
pub mod routes;

#[tokio::main]
async fn main() 
{
    let shared_state = Arc::new(connection().await.unwrap());
    let app = Router::new()
    .nest("/", routes::routes())
    .nest_service("/uploads", ServeDir::new("uploads"))
    .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    
    Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
} 