use beerus_core::lightclient::beerus::BeerusLightClient;
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result},
    proc_macros::rpc,
};

pub struct BeerusRpc {
    _beerus: BeerusLightClient,
}

#[rpc(server, client)]
trait BeerusApi {
    #[method(name = "hello_world")]
    async fn hello_world(&self) -> Result<String>;
}

#[async_trait]
impl BeerusApiServer for BeerusRpc {
    async fn hello_world(&self) -> Result<String> {
        Ok("Hello World!".to_string())
    }
}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { _beerus: beerus }
    }
}