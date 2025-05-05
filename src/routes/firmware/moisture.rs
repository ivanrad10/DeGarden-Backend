use axum::Json;
use chrono::{TimeZone, Utc};
use serde::Deserialize;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

#[derive(Deserialize)]
pub struct MoistureReading {
    pub value: f64,
    pub key: String,
}

// Send data to TimescaleDB
pub async fn send(payload: Json<MoistureReading>, db_client: Arc<Mutex<Client>>) -> String {
    // Process data
    let now_ts = Utc::now().timestamp();
    let now = Utc.timestamp_opt(now_ts, 0).unwrap();

    // TODO: Read from blockchain
    let lat = 0.0;
    let lng = 0.0;

    // Prepare the SQL query to insert moisture data
    let query = "
        INSERT INTO moisture (time, key, value, lat, lng)
        VALUES ($1, $2, $3, $4, $5);
    ";

    // Try to execute the query
    let result = db_client
        .lock()
        .await
        .execute(
            query,
            &[&(now), &(payload.key), &(payload.value), &(lat), &(lng)],
        )
        .await;

    println!("Moisture");

    match result {
        Ok(_) => "Success 200".to_string(),
        Err(_) => "Error 500".to_string(),
    }
}
