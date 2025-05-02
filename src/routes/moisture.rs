use serde::Deserialize;
use chrono::{DateTime, Utc};
use axum::Json;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

#[derive(Deserialize)]
pub struct MoistureReading {
    pub time: DateTime<Utc>,
    pub lat: f64,
    pub lng: f64,
    pub moisture: f64,
}

// Send data to TimescaleDB
pub async fn send(payload: Json<MoistureReading>, db_client: Arc<Mutex<Client>>) -> String {
    // Prepare the SQL query to insert flowmeter data
    let query = "
        INSERT INTO moisture (time, lat, lng, moisture)
        VALUES ($1, $2, $3, $4);
    ";

    // Try to execute the query
    let result = db_client.lock().await
        .execute(query, &[
            &(payload.time),
            &(payload.lat),
            &(payload.lng),
            &(payload.moisture),
        ])
        .await;

    match result {
        Ok(_) => Utc::now().timestamp().to_string(), // Return timestamp
        Err(_) => "Error 500".to_string(), // Error 500
    }
}
