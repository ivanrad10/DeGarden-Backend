use std::sync::Arc;

use axum::{extract::Path, routing::get, Router};
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub mod calculate;
pub mod direct;
pub mod types;
pub mod utils;

pub fn routes(db_client: Arc<Mutex<Client>>) -> Router {
    let direct_moisture_client = db_client.clone();
    let direct_flowmeter_client = db_client.clone();
    let calculate_moisture_client = db_client.clone();

    Router::new()
        .route(
            "/direct/moisture/:device_id",
            get(move |Path(device_id): Path<String>| {
                direct::moisture(device_id, direct_moisture_client.clone())
            }),
        )
        .route(
            "/direct/flowmeter/:device_id",
            get(move |Path(device_id): Path<String>| {
                direct::flowmeter(device_id, direct_flowmeter_client.clone())
            }),
        )
        .route(
            "/calculate/moisture/:lat/:lng",
            get(move |Path((lat, lng)): Path<(f64, f64)>| {
                calculate::moisture(lat, lng, calculate_moisture_client.clone())
            }),
        )
}
