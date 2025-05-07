use borsh::BorshDeserialize;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{env, io::Result};

use super::types::*;

pub async fn get_sensors(sensor_type: SensorType) -> Result<Vec<SensorMetadata>> {
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
