use std::sync::Arc;

use axum::{extract::Path, routing::get, Router};
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub mod host;

pub fn routes(db_client: Arc<Mutex<Client>>) -> Router {
    let host_moisture_client = db_client.clone();
    let host_flowmeter_client = db_client;
    let client_moisture_client = db_client.clone();
    let client_flowmeter_client = db_client;

    Router::new()
        .route(
            "/host/moisture/:device_id",
            get(move |Path(device_id): Path<String>| {
                host::moisture(device_id, host_moisture_client.clone())
            }),
        )
        .route(
            "/host/flowmeter/:device_id",
            get(move |Path(device_id): Path<String>| {
                host::flowmeter(device_id, host_flowmeter_client.clone())
            }),
        )
        .route(
            "/client/moisture/:device_id",
            get(move |Path(device_id): Path<String>| {
                host::moisture(device_id, client_moisture_client.clone())
            }),
        )
        .route(
            "/client/flowmeter/:device_id",
            get(move |Path(device_id): Path<String>| {
                host::flowmeter(device_id, client_flowmeter_client.clone())
            }),
        )
}
