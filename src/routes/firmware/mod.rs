use axum::{
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub mod flowmeter;
pub mod moisture;
pub mod ping;

pub fn routes(db_client: Arc<Mutex<Client>>) -> Router {
    let flowmeter_client = db_client.clone();
    let moisture_client = db_client;

    Router::new()
        .route("/ping", get(ping::ping))
        .route(
            "/flowmeter",
            post({
                move |payload: Json<flowmeter::FlowmeterReading>| {
                    flowmeter::send(payload, flowmeter_client.clone())
                }
            }),
        )
        .route(
            "/moisture",
            post({
                move |payload: Json<moisture::MoistureReading>| {
                    moisture::send(payload, moisture_client.clone())
                }
            }),
        )
}
