use std::sync::Arc;

use axum::{response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio::sync::Mutex;
use tokio_postgres::Client;

#[derive(Serialize)]
pub struct MoistureSensorData {
    time: DateTime<Utc>,
    value: f64,
}

#[derive(Serialize)]
pub struct MoistureClientData {
    values: Vec<MoistureSensorData>,
}

#[derive(Serialize)]
pub struct FlowSensorData {
    start: DateTime<Utc>,
    stop: DateTime<Utc>,
    value: f64,
}

#[derive(Serialize)]
pub struct FlowClientData {
    values: Vec<FlowSensorData>,
}

pub async fn moisture(device_id: String, db_client: Arc<Mutex<Client>>) -> impl IntoResponse {
    let client = db_client.lock().await;

    let query = "SELECT time, value FROM moisture WHERE key = $1 ORDER BY time DESC LIMIT 100";
    let rows = client.query(query, &[&device_id]).await;

    match rows {
        Ok(rows) if !rows.is_empty() => {
            let results: Vec<MoistureSensorData> = rows
                .into_iter()
                .map(|row| MoistureSensorData {
                    time: row.get(0),
                    value: row.get(1),
                })
                .collect();

            let client_data = MoistureClientData { values: results };

            Json(client_data)
        }
        _ => {
            let client_data = MoistureClientData { values: vec![] };

            Json(client_data)
        }
    }
}

pub async fn flowmeter(device_id: String, db_client: Arc<Mutex<Client>>) -> impl IntoResponse {
    let client = db_client.lock().await;

    let query =
        "SELECT start, stop, value FROM flowmeter WHERE key = $1 ORDER BY stop DESC LIMIT 100";
    let rows = client.query(query, &[&device_id]).await;

    match rows {
        Ok(rows) if !rows.is_empty() => {
            let results: Vec<FlowSensorData> = rows
                .into_iter()
                .map(|row| FlowSensorData {
                    start: row.get(0),
                    stop: row.get(1),
                    value: row.get(2),
                })
                .collect();

            let client_data = FlowClientData { values: results };

            Json(client_data)
        }
        _ => {
            let client_data = FlowClientData { values: vec![] };

            Json(client_data)
        }
    }
}
