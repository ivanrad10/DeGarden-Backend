use borsh::{BorshDeserialize, BorshSerialize};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

// For data fetch
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

#[derive(Deserialize)]
pub struct FirmwareRequest {
    pub board: String,
    pub ssid: String,
    pub password: String,
    pub key: String,
}

// For blockchain
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum SensorStatus {
    Uncollateralized,
    Collateralized,
    Slashed,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum SensorType {
    Moisture,
    Flowmeter,
}

impl From<SensorType> for u8 {
    fn from(sensor_type: SensorType) -> Self {
        sensor_type as u8
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct SensorMetadata {
    pub id: u64,
    pub model: SensorType,
    pub host: Pubkey,
    pub latitude: i64,
    pub longitude: i64,
    pub status: SensorStatus,
    pub last_collateralized_at: i64,
    pub last_uncollateralized_at: i64,
    pub last_slashed_at: i64,
    pub bump: u8,
}
