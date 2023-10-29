use axum::{ routing::get, Router, Server };
use std::net::SocketAddr;

#[tokio::main]
async fn main() 
{
    // build our application with a route
    let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(|| async { "Hello, World!"  }));
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    
    Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();

}
