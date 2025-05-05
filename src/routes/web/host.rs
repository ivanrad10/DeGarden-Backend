use std::sync::Arc;

use axum::{response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio::sync::Mutex;
use tokio_postgres::Client;

#[derive(Serialize)]
pub struct SensorData {
    time: DateTime<Utc>,
    value: f64,
}

#[derive(Serialize)]
pub struct HostData {
    values: Vec<SensorData>,
}

pub async fn host(device_id: String, db_client: Arc<Mutex<Client>>) -> impl IntoResponse {
    let client = db_client.lock().await;

    let query = "SELECT time, value FROM moisture WHERE key = $1";
    let rows = client.query(query, &[&device_id]).await;

    match rows {
        Ok(rows) if !rows.is_empty() => {
            let results: Vec<SensorData> = rows
                .into_iter()
                .map(|row| SensorData {
                    time: row.get(0),
                    value: row.get(1),
                })
                .collect();

            let host_data = HostData { values: results };

            Json(host_data)
        }
        _ => {
            let host_data = HostData { values: vec![] };

            Json(host_data)
        }
    }
}
