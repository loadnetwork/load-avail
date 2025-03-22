## About

`load-avail` is a tool composed of REST API and Rust SDK to store [Avail's](https://www.availproject.org/da) blobs on [Load Network](https://load.network) -- built on [avail-rust](https://github.com/availproject/avail-rust) and [bundler](https://github.com/bundler)

## REST API

### Store Avail blob data on Load Network

For a given tx hash and a block hash (avail), `load-avail` retrieves the blobs of the block, pack them in `0xbabe1` bundle (a single bundle can handle a maxxed Avail block) and store it on Load Network. Blobs are stored in the `DataSubmission` data struct format.

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

After a successful storage on Load Network, the blob can be retrieved from any of the Load Network data gateways; example: https://gateway.load.rs/bundle/0xf0fcdf6115c8e22c5fbc7628737b61867d092f679d53b3293bc0d65e58a3099e/0


## Roadmap

* SDK Documentation & better error handling.
* Disperse Avail DA blobs with Load Network storage sidecar.
* Deploy `load-archiver` for Avail Network.

## License
This repository is licensed under the [MIT License](./LICENSE)