// using 0xbabe1 bundles given the Avail's 1MB blob size limit
use crate::core::client::Client;
use crate::core::loader::get_avail_blobs;
use anyhow::{Error, anyhow};
use bundler::utils::core::bundle::Bundle;
use bundler::utils::core::envelope::Envelope;
use bundler::utils::core::tags::Tag;

pub fn create_envelope(data: Vec<u8>) -> Result<Envelope, Error> {
    let tags: Vec<Tag> = vec![
        Tag {
            name: "content-type".to_string(),
            value: "application/json".to_string(),
        },
        Tag {
            name: "protocol".to_string(),
            value: "load-avail".to_string(),
        },
    ];
    let envelope = Envelope::new()
        .data(Some(data))
        .target(None)
        .tags(Some(tags))
        .build()
        .map_err(|err| anyhow!("{}", err.to_string()))?;

    Ok(envelope)
}

pub async fn send_babe1_bundle(
    envelopes: Vec<Envelope>,
    private_key: String,
) -> Result<String, Error> {
    let bundle_tx = Bundle::new()
        .private_key(private_key)
        .envelopes(envelopes)
        .build()
        .expect("error propagating bundle")
        .propagate()
        .await?;

    Ok(bundle_tx)
}

pub async fn store_avail_blobs(
    avail_network_endpoint: &str,
    block_hash: &str,
    tx_hash: &str,
    load_pk: &str,
) -> Result<(String, u32, u64), Error> {
    let mut envelopes: Vec<Envelope> = vec![];
    let client = Client::new()
        .avail_websocket(avail_network_endpoint)
        .loader_pk(load_pk)
        .build()
        .await
        .unwrap();

    let blobs = get_avail_blobs(client, block_hash, tx_hash).await.unwrap();

    // as Avail has block size of 2MB (and in dicussing to lift to 4MB)
    // we can fit all block's blobs in a single 0xbabe1 bundle (8MB size limit)
    // more info: https://forum.availproject.org/t/aip-6-increase-block-size-to-4mb-maxappdatalength-to-1-mb/1648

    for blob in blobs.clone() {
        let blob = serde_json::to_vec(&blob).unwrap();
        envelopes.push(create_envelope(blob).unwrap())
    }

    let bundle_id = send_babe1_bundle(envelopes, load_pk.to_string())
        .await
        .unwrap();

    let blobs_count = blobs.len() as u32;
    let blobs_total_size = blobs.iter().map(|blob| blob.data.len()).sum::<usize>() as u64;
    Ok((bundle_id, blobs_count, blobs_total_size))
}
