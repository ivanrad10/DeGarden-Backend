use anyhow::Result;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum SensorStatus {
    Uncollateralized,
    Collateralized,
    Slashed,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum SensorType {
    Moisture,
    Flowmeter,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Sensor {
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

const SENSOR_DATA_ACCOUNT_SIZE: u64 = 91;
const SENSOR_MODEL_OFFSET: usize = 16;

pub async fn fetch_sensors(sensor_type: SensorType) -> Result<Vec<Sensor>> {
    let client = RpcClient::new_with_commitment(
        String::from("https://api.devnet.solana.com"),
        CommitmentConfig::confirmed(),
    );

    let program = pubkey!("H3B2URUN8BMxZvxx6yit2n8sRRWGo8bBdew4yDosurjG");

    let model_byte: u8 = match sensor_type {
        SensorType::Moisture  => 0,
        SensorType::Flowmeter => 1,
    };

    let config = RpcProgramAccountsConfig {
        filters: vec![
            RpcFilterType::DataSize(SENSOR_DATA_ACCOUNT_SIZE),
            RpcFilterType::Memcmp(Memcmp::new(
                SENSOR_MODEL_OFFSET,
                MemcmpEncodedBytes::Bytes(vec![model_byte]),
            )),
        ]
        .into(),
        account_config: RpcAccountInfoConfig {
            encoding: None,
            data_slice: None,
            commitment: None,
            min_context_slot: None,
        },
        with_context: None,
        sort_results: true.into(),
    };

    let accounts_raw = client
        .get_program_accounts_with_config(&program, config)
        .await?;

    let sensors = accounts_raw
        .into_iter()
        .map(|(_pubkey, acct)| {
            Sensor::try_from_slice(&acct.data[8..])
        })
        .collect::<Result<Vec<Sensor>, _>>()?;
    
    Ok(sensors)
}
