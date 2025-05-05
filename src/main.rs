use axum::Server;
use std::net::SocketAddr;

mod database;
mod routes;

#[tokio::main]
async fn main() {
    // Create the router
    let router = routes::create();

    // Define the socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}", addr);

    // Run the server
    Server::bind(&addr)
        .serve(router.await.into_make_service())
        .await
        .unwrap();
}
