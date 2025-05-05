use std::sync::Arc;

use axum::{extract::Path, routing::get, Router};
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub mod calculate;
pub mod direct;
pub mod firmware;
pub mod types;
pub mod utils;

pub fn routes(db_client: Arc<Mutex<Client>>) -> Router {
    let direct_moisture_client = db_client.clone();
    let direct_flowmeter_client = db_client.clone();
    let calculate_moisture_client = db_client.clone();

    Router::new()
        .route(
            "/direct/moisture/:key",
            get(move |Path(key): Path<String>| {
                direct::moisture(key, direct_moisture_client.clone())
            }),
        )
        .route(
            "/direct/flowmeter/:key",
            get(move |Path(key): Path<String>| {
                direct::flowmeter(key, direct_flowmeter_client.clone())
            }),
        )
        .route(
            "/calculate/moisture/:lat/:lng",
            get(move |Path((lat, lng)): Path<(f64, f64)>| {
                calculate::moisture(lat, lng, calculate_moisture_client.clone())
            }),
        )
        .route(
            "/firmware/moisture/:board/:ssid/:password/:key",
            get(
                move |Path((board, ssid, password, key)): Path<(
                    String,
                    String,
                    String,
                    String,
                )>| { firmware::moisture(board, ssid, password, key) },
            ),
        )
        .route(
            "/firmware/flowmeter/:board/:ssid/:password/:key",
            get(
                move |Path((board, ssid, password, key)): Path<(
                    String,
                    String,
                    String,
                    String,
                )>| { firmware::flowmeter(board, ssid, password, key) },
            ),
        )
}
