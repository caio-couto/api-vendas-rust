#[warn(unused_variables)]
#[warn(unused_imports)]
use axum::{ Router, Server };
use std::{net::SocketAddr, sync::Arc};
use database::connection;
pub mod routes;

#[tokio::main]
async fn main() 
{
    let shared_state = Arc::new(connection().await.unwrap());
    let app = Router::new()
    .nest("/", routes::routes())
    .with_state(shared_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    
    Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}