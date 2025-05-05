use axum::Json;
use chrono::{TimeZone, Utc};
use serde::Deserialize;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

#[derive(Deserialize)]
pub struct FlowmeterReading {
    pub start: i64,
    pub stop: i64,
    pub value: f64,
    pub key: String,
}

const TIMEOUT: i64 = 5000;

// Send data to TimescaleDB
pub async fn send(payload: Json<FlowmeterReading>, db_client: Arc<Mutex<Client>>) -> String {
    // Process data
    let stop_ts = Utc::now().timestamp() - TIMEOUT;
    let start_ts = stop_ts - (payload.stop - payload.start) / 1000;
    let start = Utc.timestamp_opt(start_ts, 0).unwrap();
    let stop = Utc.timestamp_opt(stop_ts, 0).unwrap();

    // TODO: Read from blockchain
    let lat = 0.0;
    let lng = 0.0;

    // Prepare the SQL query to insert flowmeter data
    let query = "
        INSERT INTO flowmeter (start, stop, key, value, lat, lng)
        VALUES ($1, $2, $3, $4, $5, $6);
    ";

    // Try to execute the query
    let result = db_client
        .lock()
        .await
        .execute(
            query,
            &[
                &(start),
                &(stop),
                &(payload.key),
                &(payload.value),
                &(lat),
                &(lng),
            ],
        )
        .await;

    println!("Flowmeter");

    match result {
        Ok(_) => "Success 200".to_string(),
        Err(_) => "Error 500".to_string(),
    }
}
