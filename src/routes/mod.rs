use axum::{
    Json,
    Router,
    routing::{get, post}
};

mod ping;

mod db;
mod flowmeter;
mod moisture;

pub async fn create_router() -> Router {
    let db_client = db::connect_to_db().await;
    println!("Connected to the database!");

    let flowmeter_client = db_client.clone();
    let moisture_client = db_client.clone();

    Router::new()
        .route("/ping", get(ping::ping))
        .route("/flowmeter", post({
            move |payload: Json<flowmeter::FlowmeterReading>| {
                flowmeter::send(payload, flowmeter_client.clone())
            }
        }))
        .route("/moisture", post({
            move |payload: Json<moisture::MoistureReading>| {
                moisture::send(payload, moisture_client.clone())
            }
        }))
}
