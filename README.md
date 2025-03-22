## About

`load-avail` is a tool composed of REST API and Rust SDK to store [Avail's](https://www.availproject.org/da) blobs on [Load Network](https://load.network) -- built on [avail-rust](https://github.com/availproject/avail-rust) and [bundler](https://github.com/weaveVM/bundler)

## Rust SDK

```Cargo.toml
load-avail = {git = "https://github.com/loadnetwork/load-avail.git", branch = "main"}
```

### Store a blob 

```rust
use load_avail::core::babe_tx::store_avail_blobs;
use load_avail::utils::constants::{AVAIL_MAINNET_WS, AVAIL_TURING_WS};

let block_hash: &str = "0xe17c7c47487e6144476991b29f0027491563c796fab80a8e12b20a44831cc348";
let tx_hash: &str = "0x35fa9ebde3060f0ef4f67da004193166a356cf57889ef003b45c644376e6b763";
let load_pk: &str = "...";

let (bundle_id, blobs_count, blobs_total_size) =
    store_avail_blobs(AVAIL_MAINNET_WS, block_hash, tx_hash, load_pk)
        .await
        .unwrap();

```

## REST API

##### API endpoint: [avail.load.rs](https://avail.load.rs)

### Store Avail blob data on Load Network

For a given tx hash and a block hash (avail), `load-avail` retrieves the blobs of the block, pack them in `0xbabe1` bundle (a single bundle can handle a maxxed Avail block) and store it on Load Network. Blobs are stored in the `DataSubmission` data struct format. Storage fees are subsidized by the server.

#### Mainnet

```bash
GET /v1/store/mainnet/:block_hash/:tx_hash
```

#### Turing 

```bash
GET /v1/store/turing/:block_hash/:tx_hash
```

#### Server response

```rust
pub struct StoredBlob {
    pub avail_network: String,
    pub load_hash: String,
    pub blobs_count: u32,
    pub blobs_total_size: u64,
}
```

After a successful storage on Load Network, the blob can be retrieved from any of the Load Network data gateways; example: 

* Turing blob: https://gateway.load.rs/bundle/0xf0fcdf6115c8e22c5fbc7628737b61867d092f679d53b3293bc0d65e58a3099e/0
* Mainnet blob: https://gateway.load.rs/bundle/0x3ae47078f79d1bf224636b7c915adee37858c0c1c94589a83b42e89011c9f2c2/0 


## Roadmap

* SDK Documentation & better error handling.
* Disperse Avail DA blobs with Load Network storage sidecar.
* Deploy `load-archiver` for Avail Network.

## License
This repository is licensed under the [MIT License](./LICENSE)