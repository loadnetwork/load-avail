use crate::core::babe_tx::store_avail_blobs;
use crate::utils::constants::{AVAIL_MAINNET_WS, AVAIL_TURING_WS};
use crate::utils::env_var::get_env_var;
use axum::{extract::Path, response::Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

// server types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoredBlob {
    pub avail_network: String,
    pub load_hash: String,
    pub blobs_count: u32,
    pub blobs_total_size: u64,
}

impl StoredBlob {
    pub fn new(
        avail_network: String,
        load_hash: String,
        blobs_count: u32,
        blobs_total_size: u64,
    ) -> Self {
        Self {
            load_hash,
            avail_network,
            blobs_count,
            blobs_total_size,
        }
    }
}

// server methods

pub async fn get_status() -> Json<Value> {
    Json(json!({"status": "running"}))
}

pub async fn store_avail_blob_turing(
    Path((block_hash, tx_hash)): Path<(String, String)>,
) -> Json<Value> {
    let load_pk = get_env_var("LOAD_PK").unwrap();
    let storage_res = store_avail_blobs(AVAIL_TURING_WS, &block_hash, &tx_hash, &load_pk)
        .await
        .unwrap();

    let res = StoredBlob::new(
        "turing".to_string(),
        storage_res.0,
        storage_res.1,
        storage_res.2,
    );

    Json(json!(res))
}

pub async fn store_avail_blob_mainnet(
    Path((block_hash, tx_hash)): Path<(String, String)>,
) -> Json<Value> {
    let load_pk = get_env_var("LOAD_PK").unwrap();
    let storage_res = store_avail_blobs(AVAIL_MAINNET_WS, &block_hash, &tx_hash, &load_pk)
        .await
        .unwrap();

    let res = StoredBlob::new(
        "mainnet".to_string(),
        storage_res.0,
        storage_res.1,
        storage_res.2,
    );

    Json(json!(res))
}
