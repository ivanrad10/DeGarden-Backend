use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct MoisturePoint {
    pub time: DateTime<Utc>,
    pub value: f64,
}

#[derive(Serialize)]
pub struct MoistureData {
    pub values: Vec<MoisturePoint>,
}

#[derive(Serialize)]
pub struct FlowmeterPoint {
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub value: f64,
}

#[derive(Serialize)]
pub struct IrrigationPoint {
    pub time: DateTime<Utc>,
    pub value: f64,
}

#[derive(Serialize)]
pub struct FlowmeterData {
    pub values: Vec<IrrigationPoint>,
}
