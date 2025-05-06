use axum::{response::IntoResponse, Json};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use super::{types::*, utils, program_utils::*};

pub async fn moisture(key: String, db_client: Arc<Mutex<Client>>) -> impl IntoResponse {
    let client = db_client.lock().await;

    let query = "SELECT time, value FROM moisture WHERE key = $1 ORDER BY time DESC LIMIT 100";
    let rows = client.query(query, &[&key]).await;

    // match fetch_sensors(SensorType::Moisture).await {
    //     Ok(_) => (),                              
    //     Err(e) => eprintln!("fetch_accounts failed: {}", e),
    // }

    match rows {
        Ok(rows) if !rows.is_empty() => {
            let results: Vec<MoisturePoint> = rows
                .into_iter()
                .map(|row| MoisturePoint {
                    time: row.get(0),
                    value: row.get(1),
                })
                .collect();

            let host_data = MoistureData { values: results };

            Json(host_data)
        }
        _ => {
            let host_data = MoistureData { values: vec![] };

            Json(host_data)
        }
    }
}

pub async fn flowmeter(key: String, db_client: Arc<Mutex<Client>>) -> impl IntoResponse {
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

            let values = utils::aggregate_flowmeter(results);

            let host_data = FlowmeterData { values };

            Json(host_data)
        }
        _ => {
            let host_data = FlowmeterData { values: vec![] };

            Json(host_data)
        }
    }
}
