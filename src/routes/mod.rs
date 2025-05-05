use axum::Router;

use crate::database;

mod firmware;
mod web;

pub async fn create() -> Router {
    let db_client = database::timescale::connect().await;
    println!("Connected to the database!");

    Router::new()
        .nest("/firmware", firmware::routes(db_client.clone()))
        .nest("/web", web::routes(db_client.clone()))
}
