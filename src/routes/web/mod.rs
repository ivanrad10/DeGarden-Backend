use std::sync::Arc;

use axum::{extract::Path, routing::get, Router};
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub mod host;

pub fn routes(db_client: Arc<Mutex<Client>>) -> Router {
    let moisture_client = db_client;

    Router::new().route(
        "/host/:device_id",
        get(move |Path(device_id): Path<String>| host::host(device_id, moisture_client.clone())),
    )
}
