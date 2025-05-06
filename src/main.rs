use axum::{Router, Server};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

mod database;
mod routes;

#[tokio::main]
async fn main() {
    // Create the router
    let router = routes::create().await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/", router)   
        .layer(cors); 

    // Define the socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}", addr);

    // Run the server
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
