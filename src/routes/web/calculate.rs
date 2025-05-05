use axum::{response::IntoResponse, Json};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use super::types::*;

pub async fn moisture(lat: f64, lng: f64, db_client: Arc<Mutex<Client>>) -> impl IntoResponse {
    let client = db_client.lock().await;

    let query = "
        WITH input_point AS (
          -- Define the input location (lng, lat)
          SELECT ST_SetSRID(ST_MakePoint($1, $2), 4326) AS geom
        ),
        aggregated_sensors AS (
          -- Aggregate sensor data by hour
          SELECT
            m.key,
            DATE_TRUNC('hour', m.time) AS hour,
            AVG(m.value) AS avg_value,
            ST_SetSRID(ST_MakePoint(m.lng, m.lat), 4326) AS sensor_geom
          FROM moisture m
          GROUP BY m.key, hour, m.lng, m.lat
        ),
        nearby_sensors AS (
          -- Find nearby sensors
          SELECT
            a.key,
            a.hour,
            a.avg_value,
            ST_Distance(a.sensor_geom, i.geom) AS distance
          FROM aggregated_sensors a, input_point i
          WHERE ST_DWithin(a.sensor_geom, i.geom, 10000000)
        )
        SELECT
          hour,
          SUM(avg_value * EXP(-distance)) / SUM(EXP(-distance)) AS weighted_avg
        FROM nearby_sensors
        GROUP BY hour
        ORDER BY hour;
    ";
    let rows = client.query(query, &[&lng, &lat]).await;

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
