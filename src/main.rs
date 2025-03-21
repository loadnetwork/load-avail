use crate::utils::server::{get_status, store_avail_blob_turning};
use axum::{Router, routing::get};

pub mod core;
pub mod utils;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // load secrets from Secrets.toml into env var;
    secrets.into_iter().for_each(|(key, val)| {
        println!("{:?} {:?}", key, val);
        unsafe {
            std::env::set_var(key, val);
        }
    });

    // server routes
    let router = Router::new().route("/", get(get_status)).route(
        "/v1/store/turing/:block_hash/:tx_hash",
        get(store_avail_blob_turning),
    );

    Ok(router.into())
}
