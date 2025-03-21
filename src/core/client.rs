use crate::utils::constants::LOAD_NETWORK_RPC;
use anyhow::Error;
use avail_rust::sdk::SDK;

#[derive(Clone, Debug, Default)]
pub struct Client {
    pub avail_websocket: Option<String>,
    pub avail_client: Option<SDK>,
    pub load_network_rpc: Option<String>,
    pub loader_pk: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn avail_websocket(mut self, endpoint: &str) -> Self {
        let avail_websocket = Some(endpoint.to_string());
        self.avail_websocket = avail_websocket;
        self
    }

    pub fn loader_pk(mut self, pk: &str) -> Self {
        let loader_pk = Some(pk.to_string());
        self.loader_pk = loader_pk;
        self
    }

    pub async fn build(self) -> Result<Self, Error> {
        let avail_websocket = self.avail_websocket.unwrap();
        let loader_pk = self.loader_pk.unwrap();

        Ok(Self {
            avail_websocket: Some(avail_websocket.clone()),
            loader_pk: Some(loader_pk),
            load_network_rpc: Some(LOAD_NETWORK_RPC.to_string()),
            avail_client: Some(SDK::new(&avail_websocket).await.unwrap()),
        })
    }
}
