use crate::core::avail_blob::AvailBlob;
use crate::core::client::Client;
use crate::utils::env_var::get_env_var;
use avail_rust::prelude::*;
// Turing network
// test blockhash: 0xd5f95593d91a581d7ce7b8717789298345be4be47e75ba93e7159cfe23083a7b
// test tx hash: 0x70d8cc521c341d717f5b11d1898fc7a21f9d894c3617929aaabaea71c4814911

pub async fn get_avail_blobs(
    client: Client,
    block_hash: &str,
    tx_hash: &str,
) -> Result<Vec<AvailBlob>, ClientError> {
    let load_avail_client = client.avail_client.unwrap();

    let block_hash = new_h256_from_hex(block_hash)?;

    let block = Block::new(&load_avail_client.client, block_hash).await?;

    // All Block Blobs by Hash
    let tx_hash = new_h256_from_hex(tx_hash)?;

    let blobs = block.data_submissions(Filter::new().tx_hash(tx_hash));
    let mut load_avail_blobs: Vec<AvailBlob> = vec![];

    for blob in blobs {
        load_avail_blobs.push(AvailBlob::new(blob).unwrap());
    }

    Ok(load_avail_blobs)
}
