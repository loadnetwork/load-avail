use crate::core::babe_tx::store_avail_blobs;
use crate::utils::constants::{AVAIL_MAINNET_WS, AVAIL_TURING_WS};
use crate::utils::env_var::get_env_var;
use axum::{extract::Path, response::Json};
use serde_json::{Value, json};

pub async fn get_status() -> Json<Value> {
    Json(json!({"status": "running"}))
}

pub async fn store_avail_blob_turning(
    Path((block_hash, tx_hash)): Path<(String, String)>,
) -> Json<Value> {
    let load_pk = get_env_var("LOAD_PK").unwrap();
    let storage_txid = store_avail_blobs(AVAIL_TURING_WS, &block_hash, &tx_hash, &load_pk)
        .await
        .unwrap();

    Json(json!({"bundle_txid": storage_txid, "network": "turing"})) // TODO: return blobs count and more data re it
}

pub async fn store_avail_blob_mainnet(
    Path((block_hash, tx_hash)): Path<(String, String)>,
) -> Json<Value> {
    let load_pk = get_env_var("LOAD_PK").unwrap();
    let storage_txid = store_avail_blobs(AVAIL_MAINNET_WS, &block_hash, &tx_hash, &load_pk)
        .await
        .unwrap();

    Json(json!({"bundle_txid": storage_txid, "network": "mainnet"})) // TODO: return blobs count and more data re it
}
