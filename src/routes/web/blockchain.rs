use borsh::BorshDeserialize;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{
    env,
    io::{Error, ErrorKind, Result},
};

use super::types::*;

pub async fn get_sensor(sensor_id: String) -> Result<SensorMetadata> {
    // Constants for blockchain
    const SENSOR_DATA_ACCOUNT_SIZE: u64 = 91;
    const SENSOR_ID_OFFSET: usize = 8;
    const BORSH_DISCRIMINATOR_LEN: usize = 8;

    // Extract credentials
    let rpc_url = env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL not set in .env");
    let program_id = env::var("SOLANA_PROGRAM_ID").expect("SOLANA_RPC_URL not set in .env");

    // Program id on blockchain
    let program = program_id
        .parse::<Pubkey>()
        .expect("Failed to parse SOLANA_PROGRAM_ID as Pubkey");

    // Create RPC Client
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let sensor_id_u64: u64 = sensor_id.parse().expect("Failed to parse sensor_id to u64");
    let sensor_id_bytes = sensor_id_u64.to_le_bytes().to_vec();

    // Add custom filters for RPC search
    let filters = vec![
        RpcFilterType::DataSize(SENSOR_DATA_ACCOUNT_SIZE),
        RpcFilterType::Memcmp(Memcmp::new(
            SENSOR_ID_OFFSET,
            MemcmpEncodedBytes::Bytes(sensor_id_bytes),
        )),
    ];

    // Create config for RPC search
    let config = RpcProgramAccountsConfig {
        filters: Some(filters),
        account_config: RpcAccountInfoConfig {
            encoding: None,
            data_slice: None,
            commitment: None,
            min_context_slot: None,
        },
        with_context: None,
        sort_results: Some(true),
    };

    // Run the RPC search
    let accounts = client
        .get_program_accounts_with_config(&program, config)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Format response
    for (_, acct) in accounts {
        if let Ok(sensor_metadata) =
            SensorMetadata::try_from_slice(&acct.data[BORSH_DISCRIMINATOR_LEN..])
        {
            return Ok(sensor_metadata);
        }
    }

    Err(Error::new(ErrorKind::NotFound, "Sensor not found"))
}

pub async fn get_all_sensors(sensor_type: SensorType) -> Result<Vec<SensorMetadata>> {
    const SENSOR_DATA_ACCOUNT_SIZE: u64 = 91;
    const SENSOR_MODEL_OFFSET: usize = 16;
    const BORSH_DISCRIMINATOR_LEN: usize = 8;

    let rpc_url = env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL not set in .env");
    let program_id = env::var("SOLANA_PROGRAM_ID").expect("SOLANA_RPC_URL not set in .env");
    let program = program_id
        .parse::<Pubkey>()
        .expect("Failed to parse SOLANA_PROGRAM_ID as Pubkey");

    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let filters = vec![
        RpcFilterType::DataSize(SENSOR_DATA_ACCOUNT_SIZE),
        RpcFilterType::Memcmp(Memcmp::new(
            SENSOR_MODEL_OFFSET,
            MemcmpEncodedBytes::Bytes(vec![sensor_type.into()]),
        )),
    ];

    let config = RpcProgramAccountsConfig {
        filters: Some(filters),
        account_config: RpcAccountInfoConfig {
            encoding: None,
            data_slice: None,
            commitment: None,
            min_context_slot: None,
        },
        with_context: None,
        sort_results: Some(true),
    };

    let accounts = client
        .get_program_accounts_with_config(&program, config)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    accounts
        .into_iter()
        .map(|(_, acct)| SensorMetadata::try_from_slice(&acct.data[BORSH_DISCRIMINATOR_LEN..]))
        .collect()
}
