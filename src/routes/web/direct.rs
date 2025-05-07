use axum::{http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use super::{blockchain, types::*, utils};

pub async fn moisture(key: String, db_client: Arc<Mutex<Client>>) -> impl IntoResponse {
    let sensor_metadata = match blockchain::get_sensor(key.clone()).await {
        Ok(data) => data,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                format!("Sensor with id '{}' not found on blockchain", key),
            )
                .into_response();
        }
    };

    let client = db_client.lock().await;

    let query = "SELECT time, value FROM moisture WHERE key = $1 ORDER BY time DESC LIMIT 100";
    let rows = client.query(query, &[&key]).await;

    match rows {
        Ok(rows) if !rows.is_empty() => {
            let results: Vec<MoisturePoint> = rows
                .into_iter()
                .map(|row| MoisturePoint {
                    time: row.get(0),
                    value: row.get(1),
                })
                .collect();

            let host_data = MoistureData {
                readings: results,
                metadata: sensor_metadata,
            };
            Json(host_data).into_response()
        }
        _ => (
            StatusCode::NOT_FOUND,
            format!("Sensor with id '{}' has no readings", key),
        )
            .into_response(),
    }
}

pub async fn flowmeter(key: String, db_client: Arc<Mutex<Client>>) -> impl IntoResponse {
    let sensor_metadata = match blockchain::get_sensor(key.clone()).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed getting flowmeter sensor metadata: {}", e);
            return (
                StatusCode::NOT_FOUND,
                format!("Sensor with id '{}' not found on blockchain", key),
            )
                .into_response();
        }
    };

    let client = db_client.lock().await;
    let query =
        "SELECT start, stop, value FROM flowmeter WHERE key = $1 ORDER BY stop DESC LIMIT 100";
    let rows = client.query(query, &[&key]).await;

    match rows {
        Ok(rows) if !rows.is_empty() => {
            let results: Vec<FlowmeterPoint> = rows
                .into_iter()
                .map(|row| FlowmeterPoint {
                    start: row.get(0),
                    stop: row.get(1),
                    value: row.get(2),
                })
                .collect();

            let readings = utils::aggregate_flowmeter(results);

            let host_data = FlowmeterData {
                readings,
                metadata: sensor_metadata,
            };
            Json(host_data).into_response()
        }
        _ => (
            StatusCode::NOT_FOUND,
            format!("Sensor with id '{}' has no readings", key),
        )
            .into_response(),
    }
}
