use anyhow::Error;
use avail_rust::block::DataSubmission;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailBlob {
    pub tx_hash: String,
    pub tx_index: u32,
    pub data: Vec<u8>,
    pub tx_signer: String,
    pub app_id: u32,
}

impl AvailBlob {
    pub fn new(blob: DataSubmission) -> Result<Self, Error> {
        let tx_hash = format!("0x{}", hex::encode(blob.tx_hash.0));
        let tx_signer = blob.ss58address().unwrap();

        Ok(Self {
            tx_hash,
            tx_signer,
            tx_index: blob.tx_index,
            data: blob.data,
            app_id: blob.app_id,
        })
    }
}
